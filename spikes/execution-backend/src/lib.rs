//! TASK-7 の実行バックエンド境界を検証する最小 prototype。

use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    fmt,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use process_wrap::std::CommandWrap;
#[cfg(windows)]
use process_wrap::std::JobObject;
#[cfg(unix)]
use process_wrap::std::ProcessGroup;

/// 永続化・表示に使う安定した backend identifier。
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BackendIdentifier(String);

impl BackendIdentifier {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 永続化する backend の種別。identifier は種別に加えて distro 等の instance を識別する。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendKind {
    WindowsNative,
    Wsl,
    MacosNative,
    LinuxNative,
}

/// Windows host 側の path。backend 内 path とは暗黙変換できない。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostPath(PathBuf);

impl HostPath {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

/// 特定 backend に束縛された path。
///
/// host pathとの暗黙混在はcompile時に拒否される。
///
/// ```compile_fail
/// use execution_backend_spike::{BackendPath, HostPath};
/// fn accepts_backend_path(_: BackendPath) {}
/// accepts_backend_path(HostPath::new("C:\\repo"));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendPath {
    backend: BackendIdentifier,
    path: PathBuf,
}

impl BackendPath {
    /// 永続化済みbindingを復元する。呼び出し側は保存済みidentifierを変更してはならない。
    #[must_use]
    pub fn restore(backend: BackendIdentifier, path: impl Into<PathBuf>) -> Self {
        Self {
            backend,
            path: path.into(),
        }
    }

    #[must_use]
    pub fn backend(&self) -> &BackendIdentifier {
        &self.backend
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.path
    }
}

/// 親環境に対する追加・上書き・削除だけを表す。secret を格納する契約ではない。
pub type EnvironmentDelta = BTreeMap<OsString, Option<OsString>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandRequest {
    pub program: OsString,
    pub argv: Vec<OsString>,
    pub cwd: BackendPath,
    pub env: EnvironmentDelta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogStream {
    Stdout,
    Stderr,
}

/// UTF-8 を仮定しない、到着順の stdout/stderr chunk。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogEvent {
    pub sequence: u64,
    pub stream: LogStream,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Termination {
    Exited { code: Option<i32> },
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Completion {
    pub backend: BackendIdentifier,
    pub process_id: u32,
    pub termination: Termination,
}

#[derive(Clone, Debug, Default)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    BackendMismatch {
        expected: BackendIdentifier,
        actual: BackendIdentifier,
    },
    PathConversion {
        backend: BackendIdentifier,
        path: PathBuf,
        reason: &'static str,
    },
    UnsupportedHost {
        backend: BackendIdentifier,
    },
    Launch {
        program: OsString,
        source: io::Error,
    },
    Io {
        operation: &'static str,
        source: io::Error,
    },
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendMismatch { expected, actual } => write!(
                formatter,
                "backend mismatch: expected {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::PathConversion {
                backend,
                path,
                reason,
            } => write!(
                formatter,
                "cannot convert {} for backend {}: {reason}",
                path.display(),
                backend.as_str()
            ),
            Self::UnsupportedHost { backend } => {
                write!(
                    formatter,
                    "backend {} is unavailable on this host",
                    backend.as_str()
                )
            }
            Self::Launch { program, source } => {
                write!(
                    formatter,
                    "failed to launch {}: {source}",
                    program.to_string_lossy()
                )
            }
            Self::Io { operation, source } => write!(formatter, "{operation} failed: {source}"),
        }
    }
}

impl std::error::Error for ExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Launch { source, .. } | Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// GUI・domain 層が OS process API の代わりに依存する境界。
pub trait ExecutionBackend {
    fn kind(&self) -> BackendKind;
    fn identifier(&self) -> &BackendIdentifier;

    /// host pathをこのbackendに束縛されたpathへ変換する。
    ///
    /// # Errors
    ///
    /// backendがこのhostで利用不能、またはpathを安全に変換できない場合に返す。
    fn host_path_to_backend(&self, path: &HostPath) -> Result<BackendPath, ExecutionError>;

    /// commandを実行し、stdout/stderr eventを完了前から通知する。
    ///
    /// # Errors
    ///
    /// backend/path不一致、起動失敗、process制御またはpipe I/O失敗の場合に返す。
    fn execute(
        &self,
        request: &CommandRequest,
        cancellation: &CancellationToken,
        emit: &mut dyn FnMut(LogEvent),
    ) -> Result<Completion, ExecutionError>;
}

/// `std::process` と process-wrap を用いる native prototype。
#[derive(Clone, Debug)]
pub struct StdNativeBackend {
    kind: BackendKind,
    identifier: BackendIdentifier,
    supported: bool,
}

/// 初期targetであるWindows nativeのstdベースprototype。
#[derive(Clone, Debug)]
pub struct WindowsNativeBackend(StdNativeBackend);

impl WindowsNativeBackend {
    #[must_use]
    pub fn new() -> Self {
        Self(StdNativeBackend::windows_native())
    }
}

impl Default for WindowsNativeBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionBackend for WindowsNativeBackend {
    fn kind(&self) -> BackendKind {
        self.0.kind()
    }

    fn identifier(&self) -> &BackendIdentifier {
        self.0.identifier()
    }

    fn host_path_to_backend(&self, path: &HostPath) -> Result<BackendPath, ExecutionError> {
        self.0.host_path_to_backend(path)
    }

    fn execute(
        &self,
        request: &CommandRequest,
        cancellation: &CancellationToken,
        emit: &mut dyn FnMut(LogEvent),
    ) -> Result<Completion, ExecutionError> {
        self.0.execute(request, cancellation, emit)
    }
}

impl StdNativeBackend {
    #[must_use]
    pub fn windows_native() -> Self {
        Self {
            kind: BackendKind::WindowsNative,
            identifier: BackendIdentifier::new("windows-native:v1"),
            supported: cfg!(windows),
        }
    }

    /// Windows 実機外で共通 executor 契約を検証するための host-native harness。
    #[must_use]
    pub fn host_native_harness() -> Self {
        #[cfg(target_os = "macos")]
        let (kind, identifier) = (BackendKind::MacosNative, "macos-native:v1");
        #[cfg(all(unix, not(target_os = "macos")))]
        let (kind, identifier) = (BackendKind::LinuxNative, "linux-native:v1");
        #[cfg(windows)]
        let (kind, identifier) = (BackendKind::WindowsNative, "windows-native:v1");

        Self {
            kind,
            identifier: BackendIdentifier::new(identifier),
            supported: true,
        }
    }

    fn ensure_supported(&self) -> Result<(), ExecutionError> {
        if self.supported {
            Ok(())
        } else {
            Err(ExecutionError::UnsupportedHost {
                backend: self.identifier.clone(),
            })
        }
    }
}

impl ExecutionBackend for StdNativeBackend {
    fn kind(&self) -> BackendKind {
        self.kind
    }

    fn identifier(&self) -> &BackendIdentifier {
        &self.identifier
    }

    fn host_path_to_backend(&self, path: &HostPath) -> Result<BackendPath, ExecutionError> {
        self.ensure_supported()?;
        if !path.as_path().is_absolute() {
            return Err(ExecutionError::PathConversion {
                backend: self.identifier.clone(),
                path: path.as_path().to_path_buf(),
                reason: "an absolute host path is required",
            });
        }
        Ok(BackendPath::restore(
            self.identifier.clone(),
            path.as_path(),
        ))
    }

    fn execute(
        &self,
        request: &CommandRequest,
        cancellation: &CancellationToken,
        emit: &mut dyn FnMut(LogEvent),
    ) -> Result<Completion, ExecutionError> {
        self.ensure_supported()?;
        if request.cwd.backend() != self.identifier() {
            return Err(ExecutionError::BackendMismatch {
                expected: self.identifier.clone(),
                actual: request.cwd.backend().clone(),
            });
        }

        let mut command = std::process::Command::new(&request.program);
        command
            .args(&request.argv)
            .current_dir(request.cwd.as_path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (name, value) in &request.env {
            if let Some(value) = value {
                command.env(name, value);
            } else {
                command.env_remove(name);
            }
        }

        let mut wrapped = CommandWrap::from(command);
        #[cfg(windows)]
        wrapped.wrap(JobObject);
        #[cfg(unix)]
        wrapped.wrap(ProcessGroup::leader());

        let mut child = wrapped.spawn().map_err(|source| ExecutionError::Launch {
            program: request.program.clone(),
            source,
        })?;
        let process_id = child.id();
        let stdout = child.stdout().take().ok_or_else(|| ExecutionError::Io {
            operation: "take stdout pipe",
            source: io::Error::other("stdout pipe was not created"),
        })?;
        let stderr = child.stderr().take().ok_or_else(|| ExecutionError::Io {
            operation: "take stderr pipe",
            source: io::Error::other("stderr pipe was not created"),
        })?;

        let (sender, receiver) = mpsc::channel();
        let stdout_reader = spawn_reader(stdout, LogStream::Stdout, sender.clone());
        let stderr_reader = spawn_reader(stderr, LogStream::Stderr, sender);
        let mut sequence = 0;
        let mut cancelled = false;

        let status = loop {
            if let Err(error) = drain_one(&receiver, &mut sequence, emit) {
                let _ = child.kill();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err(error);
            }
            if let Some(status) = child.try_wait().map_err(|source| ExecutionError::Io {
                operation: "poll process",
                source,
            })? {
                break status;
            }
            if cancellation.is_cancelled() && !cancelled {
                child.kill().map_err(|source| ExecutionError::Io {
                    operation: "cancel process tree",
                    source,
                })?;
                cancelled = true;
            }
        };

        finish_reader(stdout_reader, "join stdout reader")?;
        finish_reader(stderr_reader, "join stderr reader")?;
        while let Ok(message) = receiver.try_recv() {
            forward_message(message, &mut sequence, emit)?;
        }

        Ok(Completion {
            backend: self.identifier.clone(),
            process_id,
            termination: if cancelled {
                Termination::Cancelled
            } else {
                exit_termination(status)
            },
        })
    }
}

enum ReaderMessage {
    Data(LogStream, Vec<u8>),
    Failed(LogStream, io::Error),
}

fn spawn_reader(
    mut reader: impl Read + Send + 'static,
    stream: LogStream,
    sender: mpsc::Sender<ReaderMessage>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => return,
                Ok(length) => {
                    if sender
                        .send(ReaderMessage::Data(stream, buffer[..length].to_vec()))
                        .is_err()
                    {
                        return;
                    }
                }
                Err(error) => {
                    let _ = sender.send(ReaderMessage::Failed(stream, error));
                    return;
                }
            }
        }
    })
}

fn drain_one(
    receiver: &mpsc::Receiver<ReaderMessage>,
    sequence: &mut u64,
    emit: &mut dyn FnMut(LogEvent),
) -> Result<(), ExecutionError> {
    match receiver.recv_timeout(Duration::from_millis(10)) {
        Ok(message) => forward_message(message, sequence, emit),
        Err(mpsc::RecvTimeoutError::Timeout | mpsc::RecvTimeoutError::Disconnected) => Ok(()),
    }
}

fn forward_message(
    message: ReaderMessage,
    sequence: &mut u64,
    emit: &mut dyn FnMut(LogEvent),
) -> Result<(), ExecutionError> {
    match message {
        ReaderMessage::Data(stream, bytes) => {
            emit(LogEvent {
                sequence: *sequence,
                stream,
                bytes,
            });
            *sequence += 1;
            Ok(())
        }
        ReaderMessage::Failed(stream, source) => Err(ExecutionError::Io {
            operation: match stream {
                LogStream::Stdout => "read stdout",
                LogStream::Stderr => "read stderr",
            },
            source,
        }),
    }
}

fn finish_reader(
    handle: thread::JoinHandle<()>,
    operation: &'static str,
) -> Result<(), ExecutionError> {
    handle.join().map_err(|_| ExecutionError::Io {
        operation,
        source: io::Error::other("reader thread panicked"),
    })
}

fn exit_termination(status: ExitStatus) -> Termination {
    Termination::Exited {
        code: status.code(),
    }
}

#[must_use]
pub fn os(value: impl AsRef<OsStr>) -> OsString {
    value.as_ref().to_os_string()
}
