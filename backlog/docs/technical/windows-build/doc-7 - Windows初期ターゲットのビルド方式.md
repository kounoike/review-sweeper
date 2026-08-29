---
id: doc-7
title: Windows初期ターゲットのビルド方式
type: specification
created_date: '2026-08-29 03:16'
updated_date: '2026-08-29 03:44'
---
# Windows初期ターゲットのビルド方式

## 目的と決定状態

調査日は2026-08-29。ADR-0001、ADR-0002、ADR-0003、ADR-0010に従い、GPUIを使うRustネイティブGUIをWindowsで開発・CI・配布ビルドする経路を整理する。Windows初期ターゲットのbuild基準はWindows native MSVC toolchainとする。一般公開時の配布チャネル、署名主体、publisher identity、更新方式は製品・運用判断を伴うため、本メモだけでは確定しない。

## 採用するbuild基準

| 項目 | 初期基準 | 理由 |
| --- | --- | --- |
| CPU | x86-64 | 現在のWindows実証、GitHub-hosted runner、一般的な開発機の共通範囲。ARM64対応を否定しないが初期成果物を増やさない |
| Rust target | `x86_64-pc-windows-msvc` | RustのTier 1 with host toolsであり、Windows 10+/Server 2016+、PE/COFF、Microsoft ABIを公式に扱う |
| build host | Windows x64 native | Rust公式はnon-Windows hostからMSVC targetへのcross compileをsupportedとはしていない。compile/linkだけでなくWindows上のtest/runを同じ経路で確認できる |
| Rust | `mise.toml`の`rust = "stable"`と各crateの`Cargo.lock` | ADR-0003の共通入口に従い、dependencyは`--locked`で固定する。ただし`stable`は更新されるためbit-for-bit再現ではない。release候補ではRust versionとrunner imageを記録する |
| C/C++ toolchain | Visual Studio 2022 Build ToolsのMSVC、Windows SDK | GPUIおよびnative dependencyのcompile/link、Windows resource、system import libraryを供給する |
| UI framework | GPUIのADRで承認されたversionをCargo manifest/lockfileでpin | 公開版とmainの能力差をbuildへ混入させない |

ARM64の`aarch64-pc-windows-msvc`もRustのTier 1 with host toolsだが、初期対象には含めない。ARM64利用者需要、Windows ARM64 runnerの安定提供、GPUIのARM64実機検証が揃った時点で追加を再検討する。native dependencyのcompile/link、GPU/IME/accessibility、package architectureもその実機検証で確認する。`i686-pc-windows-msvc`は新規需要が示されない限り対象外とする。

## 開発・CI・配布build経路の比較

| 用途 | 実行環境と入口 | 成果物・検証 | 採否 |
| --- | --- | --- | --- |
| 開発 | Windows 11 x64上のVS 2022 x64 Native Tools PowerShellで`mise install`後、`mise run windows-build-dev` | `target/x86_64-pc-windows-msvc/debug/*.exe`。高速なincremental buildと手動起動 | 基準経路。WSL上のLinux binaryはWindows GUIの代替にしない |
| CI | `windows-2022`を明示したGitHub-hosted x64 runnerで`mise install`後、`mise run windows-check` | format、clippy、unit/integration testをWindows target上で実行。必要なGUI smokeはinteractive desktopを持つself-hosted Windows 11で別途行う | 基準経路。`windows-latest`は移行でimageが変わるため使わない |
| 配布build | cleanなWindows x64 runnerで`mise run windows-build-release` | `target/x86_64-pc-windows-msvc/release/*.exe`。release profile、lockfile、Rust/runner/toolchain version、checksum、license/SBOM、署名前hashを記録 | 基準経路。package/sign/publishは別段階とし、secretを通常のbuild/testへ渡さない |
| LinuxからMSVC cross build | `cargo-xwin`等でPEを生成 | compile/linkの早期検出には使えるが、Windows run/test、GPU、IME、UIA、署名を保証しない | native runner障害時の補助候補。release基準にしない |
| MinGW GNU target | `x86_64-pc-windows-gnu`とMinGW toolchain | GNU ABIの別成果物 | MSVC前提のSDK/native dependencyとの差を増やすため初期不採用 |

CIは通常のheadless unit/integration testと、desktopが必要なGPUI実機smokeを分ける。GitHub-hosted runner上でwindow表示の成否だけを製品UIの保証にしない。署名用credentialはfork PRや通常CIへ公開せず、保護されたrelease workflowだけで扱う。

## GPUIとWindows native依存

GPUI 0.2.2のWindows target metadataは、`windows` crate経由でWin32 window/message、COM、Direct3D 11、DirectComposition、DirectWrite、DWM、DXGI、GDI、IME、Shell、WinSock等を参照し、`embed-resource`もWindows dependencyに持つ。TASK-1ではWindows 11、Rust MSVC、VS 2022/Windows SDKでx86-64 PEのcompile/link、DirectWrite/D3D11を使う実window描画を確認済みである。

したがって開発機とCIには次が必要になる。

- Windows 11 x64（Rust target自体の最低条件はWindows 10だが、製品の最低OSは製品判断として別途確定する）
- Visual Studio 2022 Build ToolsのDesktop development with C++相当、x64 MSVC toolset、Windows 10/11 SDK
- `mise`が導入するRust stable MSVC host toolchainと、lockfileで固定したcrate群
- 実行確認ではDirect3D 11対応GPU/driver、DirectWrite、IME/UI Automationを含むWindows desktop session

成果物はまずPE/COFFの`.exe`である。実製品crateがasset、icon、manifest、native DLLを導入した場合は、それらもpackage inventoryへ明示する。Visual C++ runtimeについては推測で同梱せず、release binaryを`dumpbin /DEPENDENTS`等で検査し、必要な場合はMicrosoftのredistributable licenseに従い、推奨されるcentral deploymentを配布設計へ組み込む。PDBを生成するprofileを採用する場合は公開packageへ無条件に含めず、同一buildに紐づくdiagnostic artifactとしてアクセスを分離する。

## miseによる再現手順

現在は製品workspaceが未作成のため、承認済みGPUI 0.2.2 evidence crateを同じWindows基準でbuildする。製品crate追加時はmanifest pathだけを製品workspaceへ移し、task名とtarget契約を維持する。

```powershell
# VS 2022 x64 Native Tools PowerShellで、repository rootから実行
mise install
mise run windows-build-dev
mise run windows-check
mise run windows-build-release
```

- `windows-build-dev`: `--locked --target x86_64-pc-windows-msvc`のdev build。
- `windows-check`: `cargo fmt --check`、同targetの`cargo clippy --all-targets -D warnings`、`cargo test`。
- `windows-build-release`: `windows-check`成功後に`cargo build --release --locked`。
- 明示した`--target`により出力は`spikes/ui-framework-gpui/target/x86_64-pc-windows-msvc/{debug,release}`へ分離される。

release候補のprovenanceにはcommit、dirty flag、`rustc -Vv`、`cargo -V`、`mise --version`、runner image、MSVC/Windows SDK、GPUI/Cargo.lock hash、artifact SHA-256を残す。現在の`rust = "stable"`は毎回同一compilerを保証しないため、release再現性が必要になった時点でRust version pinまたはrelease manifestへのversion記録を追加する。これはツール更新ポリシーの変更なので本spikeで勝手に固定しない。

## 初期preview配布成果物の決定

| 選択肢 | 長所 | 制約 |
| --- | --- | --- |
| portable ZIP（初期preview採用） | release EXE、license、checksumをまとめるだけで可逆。package identityやinstaller技術を先送りできる | unsigned EXEはSmartScreen警告や企業blockの可能性。install/update/uninstall統合なし。一般公開の最終方式にはしない |
| Microsoft Store向けMSIX | StoreがMSIXを再署名し、trusted install、更新、差分downloadを提供 | Partner Center、package identity、manifest、Store policy/運用を早期に確定する |
| direct download署名済みMSIX | App Installer更新、package identity、自前配布制御 | CA-trusted署名（MicrosoftはAzure Artifact Signing等を案内）、timestamp、hosting、更新運用が必要。self-signedは一般配布に不適 |
| MSI/EXE installer | 既存Win32配布や柔軟なbootstrapに適する | installerとPEを自前署名し、更新・hosting・prerequisiteを自前運用する。新規アプリの初期基準にする根拠は弱い |

2026-08-29にユーザーが選択肢Aを承認したため、初期MVP/previewの配布build成果物は、Windows native MSVCで生成したx64 release EXE、製品のlicense、EXEに対するSHA-256 checksumを1つのportable ZIPにまとめ、非公開のCI artifactとして保存する。ZIP自体は署名も一般公開も行わず、通常のbuild/test jobへ署名credentialを渡さない。製品crateとrepository licenseが追加された後、TASK-11のCI構築でこの契約をworkflowへ配線する。

これは初期MVP/previewの基準であり、一般公開方式の決定ではない。署名済みMSIX、Microsoft Store、direct download、GitHub Releases、成果物命名、publisher identity、secret、更新方式は、一般公開前にTASK-12で再評価する。

## 将来のmacOS/Linuxとcross compile

共通にできるのはRust domain/application crate、task名、lockfile、artifact provenanceの契約である。GUI/native adapter、SDK、linker、package、署名、runtime testはOSごとにnative runnerを用意する。

- macOS: macOS runner、Apple SDK/Xcode、code signing/notarization、DMGまたはapp bundleを別経路で設計する。Windows MSVC taskを抽象化して単一cross buildにしない。
- Linux: distribution/glibc baseline、Wayland/X11、system library、AppImage/Flatpak等を別途評価する。TASK-1で観測したLinux native dependency不足はWindows成果物のblockerではない。
- Windows ARM64: x64と別artifact/test matrixにし、実機検証とpackage architecture対応後に追加する。
- non-Windows→Windows cross compile: compile-only補助として再検討できるが、Rust公式のsupported baselineではない。native dependency、Microsoft SDK/CRT license、C/C++ build script、resource compiler、署名、Windows runtime testを個別に満たす必要がある。

再検討条件は、Windows runnerの長期障害または費用、ARM64需要、native build時間がrelease SLAを妨げること、GPUI/native dependencyが公式cross pathを提供すること、macOS/Linuxの製品優先度が承認されることである。初期TASK-3は他OSの配布対応を約束しない。

## 一次情報と実証

- Rust Windows MSVC target: https://doc.rust-lang.org/rustc/platform-support/windows-msvc.html
- Rust platform tier: https://doc.rust-lang.org/rustc/platform-support.html
- Cargo build/profile: https://doc.rust-lang.org/stable/cargo/commands/cargo-build.html および https://doc.rust-lang.org/cargo/reference/profiles.html
- GitHub-hosted runner: https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/choose-the-runner-for-a-job
- Windows 2022 runner inventory: https://github.com/actions/runner-images/blob/main/images/windows/Windows2022-Readme.md
- mise tasks: https://mise.jdx.dev/tasks/ および https://mise.jdx.dev/tasks/task-configuration.html
- Microsoft packaging/distribution: https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/packaging/ および https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/choose-distribution-path
- MSIX signing: https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview
- Visual C++ deployment/runtime: https://learn.microsoft.com/en-us/cpp/windows/deployment-in-visual-cpp および https://learn.microsoft.com/en-us/cpp/windows/redistributing-visual-cpp-files
- GPUI Windows source: https://github.com/zed-industries/zed/tree/main/crates/gpui_windows
- ローカル実証: TASK-1、doc-5、`spikes/windows-ui-evidence/README.md`

## 2026-08-29 Windows native再検証

Windows Rust 1.98.0、VS 2022 MSVC 14.39のx64 developer environmentで、mise taskと同一のcommand bodyを実行した。Windows側にmise.exeが未導入のため、このhostではmise processそのものではなく、Linux側でmise task schemaを検証した上でWindows native cargo.exeを直接実行した。結果は次のとおり。

- mise tasks validate: 14 task、error 0、warning 0。windows-build-releaseからwindows-checkへの依存も解決した。
- cargo fmt --check: 成功。
- cargo clippy --locked --target x86_64-pc-windows-msvc --all-targets -- -D warnings: 成功。proc-macro-error2 2.0.1のfuture incompatibility warningは既知dependency warningとして残る。
- cargo test --locked --target x86_64-pc-windows-msvc: lib test 4件、bin/doc test 0件、すべて成功。
- cargo build --release --locked --target x86_64-pc-windows-msvc: 成功。11,101,696 byteのx64 PE、SHA-256 0d6d001cc999137164f9ab5f5e9dd7c76bbcd759aa5568fac32ec28a9cd0ba05を生成した。
- dumpbin: evidence binaryは現状Windows CUI subsystemであり、製品GUI crateではwindows_subsystem=windows、icon/version resource、manifestをrelease gateで確認する必要がある。これはbuild方式の選定であり、本taskではprototype codeを変更しない。
- dumpbin /DEPENDENTS: Windows system DLL群に加えてVCRUNTIME140.dllとUniversal CRT API setへの依存を確認した。portable previewでは対応WindowsとVisual C++ Redistributable prerequisiteを明記し、一般配布packageではMicrosoft推奨のcentral deployment/installer prerequisiteを評価する。runtime DLLを推測でZIPへcopyしない。

Windows実機の起動・描画・入力はTASK-1の既存証拠を参照し、今回の再検証ではtest/build process以外を起動していない。
