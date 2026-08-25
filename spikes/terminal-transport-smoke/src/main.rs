#[cfg(windows)]
mod windows_smoke {
    use std::{
        env,
        error::Error,
        io::{ErrorKind, Read, Write},
        process::Command,
        thread,
        time::{Duration, Instant},
    };

    use conpty::{ProcessOptions, io::PipeReader};

    #[derive(Clone, Copy)]
    enum Backend {
        Windows,
        Wsl,
    }

    pub fn run() -> Result<(), Box<dyn Error>> {
        let backend = match env::args().nth(1).as_deref() {
            Some("windows") => Backend::Windows,
            Some("wsl") => Backend::Wsl,
            _ => return Err("usage: terminal-transport-smoke <windows|wsl>".into()),
        };

        let command = match backend {
            Backend::Windows => {
                let mut command = Command::new("cmd.exe");
                command.args(["/Q", "/K"]);
                command
            }
            Backend::Wsl => {
                let distribution = env::var("REVIEW_SWEEPER_WSL_DISTRO")
                    .unwrap_or_else(|_| "Ubuntu-24.04".to_owned());
                let mut command = Command::new("wsl.exe");
                command.args(["-d", &distribution, "--", "bash"]);
                command
            }
        };

        let mut options = ProcessOptions::default();
        options.set_console_size(Some((80, 24)));
        let mut process = options.spawn(command)?;
        let mut input = process.input()?;
        let mut output = process.output()?;
        output.blocking(false);

        match backend {
            Backend::Windows => {
                input.write_all(b"echo READY-WINDOWS & echo INPUT=INPUT-WINDOWS\r\n")?
            }
            Backend::Wsl => {
                input.write_all(b"printf 'READY-WSL\\n'; printf 'INPUT=INPUT-WSL\\n'\n")?
            }
        }
        input.flush()?;

        let ready_marker = match backend {
            Backend::Windows => "INPUT=INPUT-WINDOWS",
            Backend::Wsl => "INPUT=INPUT-WSL",
        };
        let mut transcript = read_until(&mut output, ready_marker, Duration::from_secs(5))?;

        process.resize(100, 30)?;
        match backend {
            Backend::Windows => {
                input.write_all(b"echo SIZE-AFTER=100x30 & echo EXIT-WINDOWS & exit\r\n")?
            }
            Backend::Wsl => input
                .write_all(b"printf 'SIZE-AFTER='; stty size; printf 'EXIT-WSL\\n'; exit 0\n")?,
        }
        input.flush()?;

        let exit_marker = match backend {
            Backend::Windows => "EXIT-WINDOWS",
            Backend::Wsl => "EXIT-WSL",
        };
        transcript.push_str(&read_until(
            &mut output,
            exit_marker,
            Duration::from_secs(5),
        )?);
        let exit_code = process.wait(Some(5_000))?;
        transcript.push_str(&read_remaining(&mut output, Duration::from_secs(1))?);

        let expected = match backend {
            Backend::Windows => [
                "READY-WINDOWS",
                "INPUT=INPUT-WINDOWS",
                "SIZE-AFTER=100x30",
                "EXIT-WINDOWS",
            ],
            Backend::Wsl => [
                "READY-WSL",
                "INPUT=INPUT-WSL",
                "SIZE-AFTER=30 100",
                "EXIT-WSL",
            ],
        };

        println!(
            "BACKEND={}",
            match backend {
                Backend::Windows => "windows",
                Backend::Wsl => "wsl",
            }
        );
        println!("EXIT_CODE={exit_code}");
        println!("OUTPUT={}", transcript.escape_debug());
        if exit_code == 0 && expected.iter().all(|marker| transcript.contains(marker)) {
            println!("RESULT=ok");
            Ok(())
        } else {
            Err(format!("unexpected PTY transcript or exit code {exit_code}").into())
        }
    }

    fn read_until(
        output: &mut PipeReader,
        marker: &str,
        timeout: Duration,
    ) -> Result<String, Box<dyn Error>> {
        let started = Instant::now();
        let mut transcript = String::new();
        let mut buffer = [0_u8; 4096];
        while started.elapsed() < timeout {
            match output.read(&mut buffer) {
                Ok(0) => thread::sleep(Duration::from_millis(10)),
                Ok(read) => {
                    transcript.push_str(&String::from_utf8_lossy(&buffer[..read]));
                    if transcript.contains(marker) {
                        return Ok(transcript);
                    }
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => return Err(error.into()),
            }
        }
        Err(format!("timed out waiting for {marker}; output={transcript:?}").into())
    }

    fn read_remaining(
        output: &mut PipeReader,
        timeout: Duration,
    ) -> Result<String, Box<dyn Error>> {
        let started = Instant::now();
        let mut transcript = String::new();
        let mut buffer = [0_u8; 4096];
        while started.elapsed() < timeout {
            match output.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => transcript.push_str(&String::from_utf8_lossy(&buffer[..read])),
                Err(error)
                    if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::BrokenPipe) =>
                {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => return Err(error.into()),
            }
        }
        Ok(transcript)
    }
}

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    windows_smoke::run()
}

#[cfg(not(windows))]
fn main() {
    eprintln!("このsmoke testはWindows native buildで実行してください");
}
