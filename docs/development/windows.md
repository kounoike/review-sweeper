# Windows native 開発

初期ターゲットは ADR-0001、ADR-0002、ADR-0010 に従う Windows 11 x64 native GPUI
アプリである。WSL2 は開発用 checkout に利用できるが、製品 GUI の build と実行は
Windows native toolchain で行う。

## 必要な環境

- Windows 11 x64 と DirectX 11 対応 GPU
- Visual Studio 2022 または Build Tools 2022 の Desktop development with C++
- MSVC v143 x64 build tools と Windows 10/11 SDK
- `rustup`、stable `x86_64-pc-windows-msvc` toolchain
- `mise`

Visual Studio Installer で C++ workload と SDK を導入し、x64 Native Tools PowerShell
または `VsDevCmd.bat -arch=x64 -host_arch=x64` を適用した shell を使う。

```powershell
rustup default stable-x86_64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
mise install
mise run windows-check
mise run windows-build-dev
mise run windows-run
```

`mise run windows-run` で `Review Sweeper` と表示する最小 native window が起動する。
ウィンドウを閉じると process が終了する。release build は
`mise run windows-build-release` を使う。GPUI は ADR-0010 で承認された公開版 `0.2.2`
を exact pin し、`Cargo.lock` と合わせて更新を review する。

## WSL checkout を Windows から build する場合

PowerShell から `\\wsl.localhost\<distro>\...` の repository root を開ける。ただし、UNC
path 直下の Cargo incremental build は Windows の file locking で失敗することがある。
その場合は source を移動せず、生成物だけを Windows local filesystem に置く。

```powershell
$env:CARGO_INCREMENTAL = "0"
$env:CARGO_TARGET_DIR = "$env:LOCALAPPDATA\Temp\review-sweeper-target"
mise run windows-check
mise run windows-build-dev
mise run windows-run
```

この基盤は installer、署名、公開 package、ARM64、macOS/Linux build を決定しない。
初期 preview と一般公開方式は TASK-3、TASK-11、TASK-12 の範囲で扱う。
