use std::{collections::BTreeMap, ffi::OsString, path::PathBuf, thread, time::Duration};

#[cfg(not(windows))]
use execution_backend_spike::StdNativeBackend;
use execution_backend_spike::{
    BackendIdentifier, CancellationToken, CommandRequest, ExecutionBackend, ExecutionError,
    HostPath, LogEvent, LogStream, Termination, WindowsNativeBackend, os,
};

fn backend_and_cwd() -> (
    Box<dyn ExecutionBackend>,
    execution_backend_spike::BackendPath,
) {
    #[cfg(windows)]
    let backend: Box<dyn ExecutionBackend> = Box::new(WindowsNativeBackend::new());
    #[cfg(not(windows))]
    let backend: Box<dyn ExecutionBackend> = Box::new(StdNativeBackend::host_native_harness());
    let cwd = backend
        .host_path_to_backend(&HostPath::new(
            std::env::current_dir().expect("current directory"),
        ))
        .expect("convert cwd");
    (backend, cwd)
}

#[test]
fn windows_backend_identifier_is_stable() {
    let backend = WindowsNativeBackend::new();
    assert_eq!(backend.identifier().as_str(), "windows-native:v1");
}

fn request(argument: &str) -> (Box<dyn ExecutionBackend>, CommandRequest) {
    let (backend, cwd) = backend_and_cwd();
    let request = CommandRequest {
        program: OsString::from(env!("CARGO_BIN_EXE_execution_backend_fixture")),
        argv: vec![os(argument)],
        cwd,
        env: BTreeMap::default(),
    };
    (backend, request)
}

#[test]
fn streams_stdout_and_stderr_and_completes_successfully() {
    let (backend, request) = request("success");
    let mut events = Vec::new();
    let completion = backend
        .execute(&request, &CancellationToken::default(), &mut |event| {
            events.push(event);
        })
        .expect("execute fixture");

    assert_eq!(
        completion.termination,
        Termination::Exited { code: Some(0) }
    );
    assert_stream_contains(&events, LogStream::Stdout, b"stdout-marker");
    assert_stream_contains(&events, LogStream::Stderr, b"stderr-marker");
    assert!(
        events
            .windows(2)
            .all(|pair| pair[0].sequence < pair[1].sequence)
    );
}

#[test]
fn nonzero_exit_is_a_completion_not_an_execution_error() {
    let (backend, request) = request("nonzero");
    let completion = backend
        .execute(&request, &CancellationToken::default(), &mut |_| {})
        .expect("non-zero is a completed launch");

    assert_eq!(
        completion.termination,
        Termination::Exited { code: Some(23) }
    );
}

#[test]
fn launch_failure_is_distinct_from_completion() {
    let (backend, mut request) = request("success");
    request.program = OsString::from("review-sweeper-definitely-missing-executable");

    let error = backend
        .execute(&request, &CancellationToken::default(), &mut |_| {})
        .expect_err("missing executable must fail launch");
    assert!(matches!(error, ExecutionError::Launch { .. }));
}

#[test]
fn cancellation_terminates_the_contained_process() {
    let (backend, request) = request("sleep");
    let cancellation = CancellationToken::default();
    let trigger = cancellation.clone();
    let canceller = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        trigger.cancel();
    });

    let completion = backend
        .execute(&request, &cancellation, &mut |_| {})
        .expect("cancel process");
    canceller.join().expect("join canceller");

    assert_eq!(completion.termination, Termination::Cancelled);
}

#[test]
fn backend_paths_reject_relative_and_cross_backend_use() {
    let (backend, cwd) = backend_and_cwd();
    let relative = backend.host_path_to_backend(&HostPath::new("relative"));
    assert!(matches!(
        relative,
        Err(ExecutionError::PathConversion { .. })
    ));

    let wrong_cwd = execution_backend_spike::BackendPath::restore(
        BackendIdentifier::new("another-backend:v1"),
        cwd.as_path(),
    );
    let request = CommandRequest {
        program: os(env!("CARGO_BIN_EXE_execution_backend_fixture")),
        argv: vec![os("success")],
        cwd: wrong_cwd,
        env: BTreeMap::default(),
    };
    let error = backend
        .execute(&request, &CancellationToken::default(), &mut |_| {})
        .expect_err("another backend must not accept the path");
    assert!(matches!(error, ExecutionError::BackendMismatch { .. }));

    assert_ne!(
        backend.identifier(),
        &BackendIdentifier::new("another-backend:v1")
    );
    let _host_only: PathBuf = HostPath::new("host-only").as_path().to_path_buf();
}

fn assert_stream_contains(events: &[LogEvent], stream: LogStream, needle: &[u8]) {
    let combined = events
        .iter()
        .filter(|event| event.stream == stream)
        .flat_map(|event| event.bytes.iter().copied())
        .collect::<Vec<_>>();
    assert!(
        combined
            .windows(needle.len())
            .any(|window| window == needle),
        "stream {stream:?} did not contain {needle:?}: {combined:?}"
    );
}
