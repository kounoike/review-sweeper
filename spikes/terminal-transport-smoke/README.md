# Windows ConPTY / WSL transport smoke test

TASK-1でterminal frontendを実装せず、process作成、入出力、resize、終了だけを確認する隔離testである。Windowsでは`conpty` 0.7.0を使う。

```powershell
cargo run --manifest-path .\spikes\terminal-transport-smoke\Cargo.toml -- windows
$env:REVIEW_SWEEPER_WSL_DISTRO = 'Ubuntu-24.04'
cargo run --manifest-path .\spikes\terminal-transport-smoke\Cargo.toml -- wsl
```

これはVT parser/grid、GUI統合、IME、selection/copy、accessibility、長時間process管理を検証しない。WSL modeもWindows processとして`wsl.exe`をConPTY内で起動するsmoke testに留まり、製品のbackend RPC/stream設計を確定しない。
