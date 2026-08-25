---
id: doc-5
title: Windows Rust UIフレームワーク技術検証
type: other
created_date: '2026-08-23 06:55'
updated_date: '2026-08-25 05:29'
---
# Windows Rust UIフレームワーク技術検証

## 目的と判定方法

TASK-1 の技術検証結果を記録する。調査日は 2026-08-23。検証環境は Windows 11 ホスト上の WSL2 Linux である。Windows interop、Windows native Rust、Visual Studio C++ toolchainを子プロセス内だけで利用し、両候補のPE生成、Windows desktopでのpixel描画、pointer/key入力、100,000行scroll、background更新を確認した。ConPTY/WSL transportもprocess・I/O・resize・終了まで確認した。一方、実IME composition、Narrator音声、focus traversal、製品相当diff/terminalとGPU frame timeは未検証である。

以下は根拠を `[一次情報]`（公式文書・公開ソース）、`[ローカル実測]`（この WSL で再現）、`[外部報告]`（公式 issue 等）、`[未検証]`（Windows 実機が必要）に区別する。同じバージョン番号でも公開 crate とリポジトリ main を同一視しない。

## 比較結果

| 候補 | Windows・ビルド | 非同期処理 | 大量行・描画 | アクセシビリティ・IME | 保守性・制約 |
| --- | --- | --- | --- | --- | --- |
| GPUI crates.io 0.2.2 | `[一次情報]` 公開ソースに Win32、Direct3D 11、DirectWrite、IME backend を含む。ただし同梱 README は macOS/Linux のみ | event loop 統合 executor、Entity/Context から background task と UI 更新を扱える | `uniform_list` は可視範囲だけを生成 | 公開版に AccessKit 依存・公開 accessibility API を確認できない。Windows IME handler はある | pre-1.0。Zed と同時開発で変更が大きい |
| GPUI main | `[一次情報]` README は Windows で Win32/DirectWrite を利用し、追加 feature 不要と明記 | executor 統合 | editor と仮想化 list の実装実績 | AccessKit、Windows adapter、role/name/focus/action、a11y example が追加済み。ただし Zed では experimental で製品 UI 全体は未対応 | Cargo version はなお 0.2.2 だが `gpui_platform`/`gpui_windows` 分割など公開版との差が大きく、未公開 API を前提にできない |
| eframe/egui 0.36.1 | `[一次情報]` Windows 対応。winit と wgpu で native app を起動 | 任意の thread/runtime から `Context::request_repaint` で UI を起床。async state/channel はアプリ側で所有 | immediate mode。`ScrollArea::show_rows` は可視行範囲だけを生成 | AccessKit を native の既定経路で利用し、標準 widget は tree を生成。custom widget は WidgetInfo が必要。現行版の Narrator、IME、focus は未実測 | MIT/Apache-2.0。API は流動的で release 間の移行負担がある |
| Slint 1.17.1 | `[一次情報]` Windows 対応。MSVC target と Visual Studio Build Tools を案内 | UI thread 制約があり、worker から `invoke_from_event_loop` | Model/ListView で data と表示を分離。対象 workload の frame time は未実測 | accessibility feature は既定有効。Windows text field と NVDA の未解決報告あり | 1.x API。GPL-3.0-only、royalty-free、commercial の適用条件を製品利用前に確認要 |
| Iced 0.14.0 | `[一次情報]` Windows/macOS/Linux/Web。wgpu は DX12、tiny-skia fallback あり | Task/Subscription を第一級で提供 | GPU/software renderer。100,000 行の具体的な仮想化・frame time は未実測 | Windows screen reader の明確な保証を確認できず、accessibility/日本語 IME の未解決報告あり | MIT。公式 README が experimental と明記 |

### GPUI の公開範囲と記載差

- `[一次情報]` crates.io の最新 GPUI は 0.2.2。公開 package の VCS revision は 2025-10-22 の commit で、同梱 README のサポート記載は macOS/Linux に留まる。
- `[一次情報]` 一方、公開 0.2.2 の Cargo manifest と source には Windows target dependencies と `platform/windows` があり、Win32 window、Direct3D 11、DirectWrite、keyboard layout/AltGr、IME composition を実装している。README の記載不足だけを理由に Windows backend がないとは判断できない。
- `[一次情報]` 2026-08-22 時点の main README は Windows 対応を明記する。main は `gpui_platform` と `gpui_windows` に分割されているが `gpui_platform` は crates.io で公開を確認できず、main の Cargo version も 0.2.2 のままである。公開 0.2.2 と main は互換な同一成果物ではない。
- `[一次情報]` AccessKit 対応は 2026-05-27 の main に「first step」として追加された。role、label、focus、action、Windows adapter、example は存在する。
- `[外部報告]` Zed 側では 2026-06-17 時点で `ZED_EXPERIMENTAL_A11Y=1` を使う実験機能で、settings UI を中心に対応し、主要 UI と keyboard focus には制約が記載される。公開 GPUI 0.2.2 にはこの実装を確認できない。
- `[外部報告]` 2026-07 の GPUI main では、持続的な keyboard input と 10–20 Hz の background `notify` が重なると Windows の描画が 5–15 秒停止し得る報告が open。TASK-1 の入力＋background 更新 workload と直接関係するため実機で再現確認する。
- `[外部報告]` 日本語 IME の複数問題は 2025-11 に修正された一方、中国語 IME の二重入力/DirectWrite crash 報告が 2026-05 時点で open。日本語 IME の変換、候補位置、focus 移動は別途実測する。

### eframe/egui の実用性

- `[一次情報]` AccessKit は eframe native integration に組み込まれ、標準 button、checkbox、text edit 等は accessibility tree を生成する。独自 diff row、syntax span、selection では適切な role/name/value/action をアプリ側で付与する必要がある。
- `[外部報告]` Windows Narrator の動作例は過去に公開されているが、0.36.1 と Review Sweeper の custom widget での実測ではない。
- `[一次情報]` `Context` は clone/send 可能で、background thread から `request_repaint` を呼ぶと eframe が UI thread を起床する。runtime、cancel、channel、結果の寿命は framework が所有せず、application 層で設計するため、中程度の統合負担がある。
- `[一次情報]` immediate mode と `show_rows` により visible row だけを毎 frame 構築できる。ただし diff highlight、複数 pane、100,000 行での CPU/frame time と allocation は `[未検証]`。
- `[一次情報]` 公式文書は API が流動的で breaking changes があると明記する。release 更新時は adapter 内に差分を閉じ込める必要がある。
- `[ローカル実測]` 0.36.1の標準widgetは意味的UIA treeを生成した。`[未検証]` Narrator音声、tab order/focus announcement、shortcut、AltGr、日本語IMEの確定/候補位置を追加確認する。

### 他候補

Slint は安定 1.x、宣言 UI、thread-to-event-loop API、既定 accessibility が利点だが、license 適合と NVDA text input、大量行の実測が必要である。Iced は Task/Subscription が非同期処理に適するが、experimental API、Windows accessibility/IME、大量行仮想化の不確実性が相対的に大きい。いずれも同じ Windows 実機 matrix を通すまでは見送りとも採用とも確定しない。

## 一次情報

- [GPUI README](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md)
- [GPUI 0.2.2 source](https://crates.io/crates/gpui/0.2.2)
- [GPUI AccessKit integration](https://github.com/zed-industries/zed/commit/1d029c5f)
- [Zed on Windows](https://github.com/zed-industries/zed/blob/main/docs/src/windows.md)
- [Zed Windows accessibility issue #41138](https://github.com/zed-industries/zed/issues/41138)
- [GPUI Windows repaint starvation issue #61469](https://github.com/zed-industries/zed/issues/61469)
- [eframe 0.36.1](https://docs.rs/eframe/0.36.1/eframe/)
- [egui accessibility guide](https://github.com/emilk/egui/blob/main/docs/accessibility.md)
- [Slint Rust API](https://docs.slint.dev/latest/docs/rust/slint/)
- [Slint build guide](https://github.com/slint-ui/slint/blob/master/docs/building.md)
- [Slint Windows accessibility issue #8732](https://github.com/slint-ui/slint/issues/8732)
- [Iced README](https://github.com/iced-rs/iced)

## GPUI 最小プロトタイプ

調査コードは `spikes/ui-framework-gpui` に隔離し、製品基盤には組み込んでいない。960 x 720 window、click と上下 key binding、`uniform_list` で 100,000 行、`BackgroundExecutor` 完了時の Entity 更新、framework 非依存の `ReviewUiState`/`UiIntent` を実装した。

`[ローカル実測]` WSL2 で次を確認した。

- `cargo fmt --manifest-path spikes/ui-framework-gpui/Cargo.toml -- --check`: 成功
- `cargo test --manifest-path spikes/ui-framework-gpui/Cargo.toml --lib`: 2 tests 成功
- `cargo clippy --manifest-path spikes/ui-framework-gpui/Cargo.toml --all-targets -- -D warnings`: 成功
- `cargo check --manifest-path spikes/ui-framework-gpui/Cargo.toml`: 成功
- full `cargo test`: Linux linker に `libxkbcommon` と `libxkbcommon-x11` がなく binary link に失敗
- `cargo check --target x86_64-pc-windows-msvc`: WSL に MSVC `lib.exe` がなく `ring` build script で失敗

後二件は環境/toolchain 制約であり、Windows 成功とも framework 不具合とも扱わない。

## WSL からの Windows native build 追加検証

調査日は 2026-08-23。先行記録時点の「MSVC Build Tools は利用できない」は、通常の `PATH` だけを見た判定だった。追加調査では Visual Studio の developer environment を子 `cmd.exe` にだけ読み込むことで、ユーザー設定や既存 toolchain を変更せず Windows native build が成立した。

### 環境と toolchain の棚卸し

| 項目 | `[ローカル実測]` 結果 |
| --- | --- |
| WSL | Ubuntu 24.04 / WSL2、kernel `6.18.33.2-microsoft-standard-WSL2` |
| interop | `WSL_INTEROP=/run/WSL/1230257_interop`、`/proc/sys/fs/binfmt_misc/WSLInterop` は `enabled` |
| Windows | Windows 11 build `10.0.22631.6199`、Windows PowerShell `5.1.22621.6133` |
| Windows Rust | `cargo 1.98.0` / `rustc 1.98.0`、host と installed target は `x86_64-pc-windows-msvc` |
| Visual Studio | Community 2022 17.9.5。`VsDevCmd.bat -arch=x64 -host_arch=x64` 後に MSVC 14.39 の `cl.exe`、`link.exe`、`lib.exe` と Windows SDK 10.0.22000 の `rc.exe`、`mt.exe` を確認 |
| checkout path | `\\wsl.localhost\Ubuntu-24.04\home\kounoike\orca\workspaces\review-sweeper\task-1-ui-framework`。`cmd.exe` は UNC current directory を拒む警告を出すが、PowerShell と `cargo.exe --manifest-path <UNC>` は到達可能 |
| Linux fallback | `x86_64-pc-windows-msvc` target は導入済み。`cargo-xwin`、`x86_64-w64-mingw32-gcc`、GNU Windows target は未導入。`zig` は存在する |

読み取り確認と developer environment の使い方は次のとおり。

```bash
uname -a
cat /proc/version
cat /proc/sys/fs/binfmt_misc/WSLInterop
command -v powershell.exe cmd.exe cargo.exe rustc.exe
wslpath -w "$PWD"
powershell.exe -NoProfile -Command '$PSVersionTable.PSVersion; cargo.exe -Vv; rustc.exe -Vv'
powershell.exe -NoProfile -Command '& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath'
```

`VsDevCmd.bat` は現在の Linux shell やユーザーの永続環境を変更せず、以下の `cmd.exe` 子プロセスだけに適用した。

```powershell
$dev = 'C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat'
$manifest = '\\wsl.localhost\Ubuntu-24.04\home\kounoike\orca\workspaces\review-sweeper\task-1-ui-framework\spikes\ui-framework-gpui\Cargo.toml'
$target = "$env:LOCALAPPDATA\Temp\review-sweeper-task1-gpui"
cmd.exe /d /s /c "`"$dev`" -arch=x64 -host_arch=x64 >nul && cargo.exe build --locked --manifest-path `"$manifest`" --target-dir `"$target`""
```

### GPUI と eframe/egui の build 結果

- GPUI 0.2.2: repository の `spikes/ui-framework-gpui` を UNC `--manifest-path` で指定した。`cargo metadata --no-deps` が Windows path と Windows target dependency を解決し、Windows native `cargo.exe build --locked` が cold build 2分24秒で成功した。MSVC link 済みの `review-sweeper-gpui-spike.exe` は 24,370,176 bytes の x86-64 PE32+ だった。
- eframe/egui 0.36.1: Windows `%LOCALAPPDATA%\Temp` に一時 crate を作り、GPUI と同じく pointer/key selection、`ScrollArea::show_rows` による100,000行、background threadからの `Context::request_repaint` を実装した。既定 feature（`accesskit`、`wgpu` 等）の dependency を解決し、MSVC compile/link が成功した。`review-sweeper-eframe-slice.exe` は 47,931,392 bytes の x86-64 PE32+ だった。最初に旧 `eframe::App::update` APIを使ったため compile errorになり、0.36.1の必須 `App::ui` に合わせて成功した。この差は API 更新追随を adapter 内へ閉じ込める必要性を具体化する。
- 生成物は WSL から `powershell.exe Start-Process` で5秒間ずつ起動した。GPUI は process alive と非zero Win32 window handle、eframe は process alive、非zero handle、window title `Review Sweeper eframe slice` を確認し、検証用に開始した各 process IDだけを終了した。
- module 読み取りでは GPUI process に `user32.dll`、`d3d11.dll`、`dwrite.dll`、`dxgi.dll`、`imm32.dll`、eframe process に `user32.dll`、`d3d12.dll`、`dxgi.dll`、`imm32.dll`、`uiautomationcore.dll` を確認した。これは backend DLL のロードとWin32 window作成の証拠であり、pixel出力、GPU adapter、IME、UI Automation treeの品質を確認した証拠ではない。

### 検証段階ごとの判定

| 段階 | GPUI 0.2.2 | eframe/egui 0.36.1 | 判定の限界 |
| --- | --- | --- | --- |
| (a) `cfg` / dependency解決 | Windows native `cargo metadata` と build dependency解決が成功 | Windows native dependency解決が成功 | 公開versionと今回のfeatureだけ |
| (b) compile | prototypeのlib/binとC/C++依存をMSVC環境でcompile成功 | 同等sliceと依存をcompile成功 | release build、clean machineは未確認 |
| (c) link / PE生成 | MSVCで24.4 MB debug PE生成 | MSVCで47.9 MB debug PE生成 | size比較はfeature集合が同一でなく製品評価値ではない |
| (d) Windows native起動 | process alive + Win32 handle | process alive + Win32 handle/title | 自動操作や長時間安定性は未確認 |
| (e) Windows実機機能 | D3D11/DirectWrite/Win32/IMM DLL loadまで | D3D12/Win32/IMM/UIAutomationCore DLL loadまで | 描画内容、GPU adapter/driver、ConPTY、IME変換、Narrator/UI Automation tree、focus traversal、mouse/key repeat、scroll、background更新の視覚結果、性能は未確認 |

このため Windows build blocker は両候補で解消したが、TASK-1 AC #2 の操作・描画確認は完了していない。cross compile、Wine、DLL load、window handle のいずれも runtime/accessibility/performance の実機試験の代替にしない。TASK-1 は `In Progress`、AC #2/#3 と DoD #1/#3 は未完了のままとする。

### cargo-xwin / MinGW fallback

今回は Windows native toolchain が成立したため fallback を導入・実行していない。Linux `cargo check --target x86_64-pc-windows-msvc` が以前 `ring` build script の `lib.exe` 不在で失敗したのは、Rust targetだけでは MSVC linker、Windows SDK/Universal CRT、C/C++ toolchainを提供しないためである。

native toolchainを使えないCIでは `cargo-xwin` を第一候補とする。公式READMEに従う再現候補は `cargo install --locked cargo-xwin`、`rustup target add x86_64-pc-windows-msvc`、`cargo xwin build --target x86_64-pc-windows-msvc --manifest-path ...` で、既定は `clang-cl`、linkerはLLVM系、Microsoft CRT/Windows SDKをcacheし、CMake依存にはtoolchain fileを生成する（CMake利用時は Ninja が必要）。`cargo-xwin` 自体はMITだが、利用時は同READMEが示すMicrosoft licenseへの同意が必要で、CRT/SDKとVC++ runtimeの利用・再配布条件はMicrosoft Software License Termsを別途確認する。assemblyが `ml64.exe` 等へ固定されたcrateは追加対応が必要になり得る。

MinGW GNU targetは、依存crateがGNU ABIを公式に扱い、`x86_64-w64-mingw32-gcc`/binutilsとGNU targetをpinできる場合の第二候補に留める。MSVCを前提とするWindows SDK library、build script、prebuilt native libraryとのABI/linker差が増えるため、今回MSVCで成功したGPUI/eframeの基準経路をGNUへ置き換える根拠はない。いずれのcross buildでPEを作れてもWindows実機runtime検証は別に必要である。

一次情報:

- [cargo-xwin README](https://github.com/rust-cross/cargo-xwin)
- [Visual C++ Redistributable and license note](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170)
- [Visual Studio Build Tools component IDs](https://learn.microsoft.com/en-us/visualstudio/install/workload-component-id-vs-build-tools?view=visualstudio)

### PRレビュー機能への影響

- git diff表示: 両frameworkでWindows cfg/compile/link/window初期化まで通ったため、「Windowsでprototype自体をbuildできない」という採用上の不確実性は下がった。GPUIの `uniform_list` と eframeの `show_rows` を含む同等sliceがcompileしたことは、固定高visible-row方式の実装開始条件を満たす。ただしbuild結果だけでは実現容易性の順位を変えない。その後のWindows desktop操作結果を本書末尾の採用提案へ反映したが、syntax layout、gutter、side-by-side同期、selection/copy、inline comment、50,000〜100,000行のframe time/input latencyは未測定である。
- 内蔵terminal: GUI frontendをWindows native binaryへ組み込めることは両方で確認したが、今回のsliceはVT engineもConPTY transportも持たない。GPUIでDirectWrite/D3D11、eframeでD3D12/AccessKit関連DLLがloadしたことも、terminal gridのIME、selection/copy、screen reader、ConPTY resize/lifecycleを保証しない。Windows GUI frontendと `TerminalTransport::{WindowsConPty, Wsl}` の境界評価は維持し、実機ConPTY/WSL session試験を採用判断前に行う。

## UI 交換境界

後続 TASK-10 では次の依存方向を維持する。

```text
domain/application state
        │ UiSnapshot / UiIntent / UiEffect
        ▼
UI port（framework 非依存）
        │
        ├── GPUI adapter
        └── 他 framework adapter
```

- domain/application は GPUI/egui 等の Entity、Context、widget、key event を参照しない。
- UI は immutable snapshot を描画し、操作を semantic な `UiIntent` として application へ返す。
- background work は application service が所有し、UI へ progress/result の snapshot または event だけを渡す。
- window lifecycle、clipboard、dialog、focus、theme、accessibility metadata は platform/UI adapter に閉じ込める。
- 大量行は domain vector を widget tree へ複製せず、stable ID と visible range で問い合わせる。
- framework 固有型を crate 間の公開 API に出さない。

TASK-10 は本節と prototype の `src/lib.rs` を境界例として参照できる。

## 判断状態・制約・再検討条件

2026-08-25、ユーザーは将来性を含めて Windows native UI framework に GPUI を採用することを承認した。eframe/egui は第二候補・fallback とする。既知制約は採用撤回条件として後続実装で検証するが、この spike の採用決定を未承認状態へ戻すものではない。

再判断に最低限必要な条件は次のとおり。

- 対象となる公開 version を pin し、clean Windows 11 + MSVC で build/run できる
- Narrator と Accessibility Insights で inbox、diff row、button、focus、selection、custom widget の role/name/value/action を確認する
- mouse、keyboard、focus traversal、shortcut、AltGr、日本語 IME の変換・確定・候補位置を確認する
- 100,000 行 scroll と複数 pane を release build で測定し、frame time、入力遅延、memory、GPU/driver を記録する
- background 更新を 1 Hz と 20 Hz で継続しながら key repeat と scroll を行い、描画 starvation の有無を確認する
- async cancellation/error と UI close の競合を adapter 境界内で処理できる
- accessibility、license、pre-1.0/breaking update の受容条件を製品判断として承認する

## Windows 実機で残る手動・製品相当検証

Windows 11 の PowerShell で次を実行する。

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo run --release --manifest-path .\spikes\ui-framework-gpui\Cargo.toml
cargo run --release --manifest-path .\spikes\ui-framework-eframe\Cargo.toml
```

基本window操作は両候補で確認済みである。残る実機作業は、NarratorとAccessibility Insightsでcustom diff row/terminal gridのrole/name/focus/selectionを確認し、Microsoft IMEで日本語変換・preedit・候補位置を確認することである。OS build、Rust/framework version、GPU/driver、release/debug、frame time、入力遅延も製品相当sliceで記録する。これらとユーザー承認がない限りTASK-1はDoneにしない。

## PRレビュー機能に照らした GPUI と eframe/egui の追加比較

この節は 2026-08-23 時点の Zed main `d9ad6aff67e47de43abb270d22de75dd950f1b48`、crates.io の公開 package、各 repository の source を確認した結果である。「Zed に存在する機能」と「GPUI 単体で再利用できる機能」、「egui 向け crate が存在すること」と「Review Sweeper でそのまま利用できること」を区別する。

### 結論の範囲

- GPUI は text shaping、custom element、`uniform_list`、executor という低レベル基盤を提供するが、公開 crate に diff/editor/terminal widget はない。Zed と同等の実績は性能上の実現可能性を示すに留まり、実装をそのまま再利用できる根拠ではない。
- eframe/egui は `ScrollArea::show_rows`、`LayoutJob`/custom painter、`TextEdit`、標準 AccessKit integration と、現行 egui 0.36 に追随する第三者 terminal widget がある。初期の組み立てに使える部品は GPUI 単体より多いが、PR diff は専用 widget がなく、terminal crate も新しく、Windows/IME/accessibility は未実測である。
- Windows desktop実測とfont再実測を経て、DirectWrite system font collection/fallback、日本語表示品質、`uniform_list`、memory、executor、GPUI/AccessKitの将来性を根拠にGPUI採用が承認された。

### git diff 表示

| 要件 | GPUI 単体 | Zed 実装の再利用可能性 | eframe/egui 0.36 |
| --- | --- | --- | --- |
| inline / side-by-side | custom element と layout primitive から独自実装 | `[一次情報]` Zed editor は `DiffViewStyle::Unified/Split` と多数の split diff test を持つ。ただし `editor` は別 crate | `columns`/panel と scroll state で構成可能だが、左右の行対応、scroll同期、focus/selectionを独自実装 |
| 数万行の仮想化 | `uniform_list` は可視 range のみ生成。ただし固定高さ前提で inline comment/fold の可変高さは別設計 | Zed editor は独自 display map/element/rope/sum-tree を使い、単純な `uniform_list` widget ではない | `ScrollArea::show_rows` は可視 range のみ生成し固定高さ。可変高さは `show_viewport` と高さ index 等が必要 |
| syntax highlighting | GPUI は shaped text/highlight style の描画 primitive。parser/cache/incremental invalidation は application 側 | Zed `language`/`text`/editor が tree-sitter、semantic token、display map を統合するが別 crate 群 | `LayoutJob` と `egui_extras::syntax_highlighting`（任意 `syntect`）があり結果を memoize。可視行単位の cache/invalidation は独自設計 |
| 行番号・gutter | 独自 element | Zed editor に line number、fold、git hunk、review indicator の実装あり | 横 layout/custom painter で実装。hit test、幅計算、side-by-side 対応を独自実装 |
| 行/範囲選択・copy・keyboard | focus/input/text API から独自の anchor/selection model が必要 | Zed editor の selection、clipboard、key action は広範だが Zed model と密結合 | `TextEdit` は selection/copy/keyboard/IME/AccessKit を持つが、巨大 diff 全体を一つの `TextEdit` で layout する設計は仮想化と両立しにくい。row-based custom selection が必要 |
| comment UI・折り畳み・検索 | overlay/block/intent を独自実装 | `[一次情報]` Zed editor に diff review comment、fold、buffer search があるが feature flag と多数の project/UI 依存を持つ | 標準 widget/`CollapsingHeader`/search input は利用できるが、diff anchor の寿命、可変高さ、検索位置への scroll を独自実装 |
| 増分更新 | Entity snapshot と `cx.notify` に適合するが、stable anchor と invalidation は application 側 | Zed `buffer_diff` は imara-diff、rope、sum-tree、GPUI Entity を統合。GPL・未公開で単独転用しにくい | immediate mode は新 snapshot を描画しやすい。diff計算を background で行い `request_repaint` 可能だが、visible row cache と stable anchor は application 側 |
| accessibility | 公開 0.2.2 は tree API なし。main は AccessKit が追加されたが experimental | Zed editor source で Review Sweeper の要件を満たす公開 accessibility contract は確認できない | 標準 `TextEdit` は text/selection を AccessKit に公開。custom virtual diff row/gutter/comment は `WidgetInfo` 等を追加し、Narrator で検証が必要 |

`[一次情報]` `egui_code_editor` 0.4.1（MIT）は egui 0.36、line number、simple keyword highlight、selection、completion を提供する。しかし source は全行番号を一つの `String` にし、本文も一つの multiline `TextEdit` として layout する。diff hunk、side-by-side、comment anchor、数万行仮想化を提供しないため、そのままの採用ではなく参考実装に留める。

今回の `[ローカル実証]` では `DiffViewport`/`DiffUpdate` を追加し、100,000 行のうち visible range と changed range の交差だけを repaint 対象として返し、選択 range を UI framework 外で保持する契約を unit test した。syntax/layout の性能実測ではない。

### 内蔵 terminal

terminal は次の境界を必須とする。

```text
Windows GUI adapter
  └─ Terminal frontend（cell grid / selection / input / accessibility）
       └─ TerminalTransport（write / reply / resize / output stream）
            ├─ WindowsConPty backend（Windows process）
            └─ WSL backend（明示した distribution 上の process、RPC/stream 境界）
```

Windows native frontend が暗黙に `wsl.exe` を起動する設計や、WSL process を Windows ConPTY session と同一型の暗黙 fallback にする設計は採らない。session 作成時に backend target を明示し、VT bytes と resize/lifecycle event だけを transport 境界で交換する。

| 要件 | GPUI / Zed | eframe/egui ecosystem |
| --- | --- | --- |
| VT/xterm engine | GPUI 単体にはない。Zed `terminal` は Zed fork の `alacritty_terminal` と `vte` を使用 | `egui_tty` 0.2.0 は `libghostty-vt`、`egui_term` は `alacritty_terminal` を使用 |
| PTY/ConPTY境界 | Zed `terminal` は Alacritty PTYを所有し Windows backend を使う。Zed の `terminal_view` README は backend-neutral types を説明するが crate 自体は Zed 固有 | `egui_tty::Tty` は `write/reply/resize` の小さい port で local PTY、socket、remote shellを差し替え可能。ConPTY実装は含まず application 側で必要。`egui_term` は widget内部でlocal PTYを生成 |
| font/color/grid | GPUI text shaping/custom element で制御可能。Zed terminal element は cell、cursor、underline、hyperlink 等を独自描画 | `egui_tty` は Ghostty gridを egui painterへ描画し、color、cursor、selectionを実装。font fallback/ligature/IME候補位置のWindows実測はない |
| input/IME | Zed は raw key、action、IME commit、paste の複数経路を統合するが GPUI widgetとして公開されない | `egui_tty` は key encoding、paste、IME commit相当の `Text` eventを処理。preedit/候補表示、AltGr、日本語IMEはWindows未実測 |
| scrollback/search/link/resize | Zed実装に存在 | `egui_tty` は scrollback、search、OSC 8/URL link、selection/copy、resizeを公開。`egui_term` も基本機能を持つが under development |
| accessibility | main GPUI AccessKit はあるが Zed terminal cell tree の完成を確認できない | `egui_tty` 0.2.0 source に `WidgetInfo`/AccessKit node登録を確認できず、custom-painted grid は現状 screen readerへ内容を公開しないと見込む。実機で確認要 |

`egui_tty` 0.2.0 は MIT、egui 0.36 と整合し、VT engine と transport を分離する点は今回の境界に最も近い。`[ローカル実測]` WSL で同 crate の lib unit test 52件が成功し、VT escape、keyboard encoding、selection/copy、scrollback/search、link、resizeに関する pure/mock test を確認した。ただし build に Zig 0.15.x が必要で、この実測は Windows、ConPTY、GUI、IME、accessibility の確認ではない。

`egui_term` 公開 0.1.0（MIT）は egui 0.31/alacritty_terminal 0.25、同梱 README は macOS/Linux のみとしていた。2026-07-30 の main は egui 0.35/alacritty_terminal 0.26へ更新し README で Windows test 済みとするが、まだ 0.1.0/under development で公開 artifact と main に差があり、current eframe 0.36 へそのまま組み込めない。`egui-terminal` 0.1.0 は egui 0.22 かつ GPL-3.0-or-later で候補外に近い。

今回の `[ローカル実証]` では `TerminalTransport`、`TerminalCommand`、`TerminalBackendTarget::{WindowsConPty, Wsl { distribution }}` を追加し、backend所有場所を型で区別した。VT/PTY実装や framework adapter の動作実証ではない。

### ecosystem と license

| 部品 | 公開・license | 再利用評価 |
| --- | --- | --- |
| `gpui` | crates.io 0.2.2、Apache-2.0 | UI primitive は利用可能。main との差と pre-1.0 migration が課題 |
| Zed `editor` / `buffer_diff` / `terminal` / `terminal_view` / `ui` | workspace `publish = false`、各 manifest は GPL-3.0-or-later | Zed製品での実績確認用。Review Sweeperへコピー/直接依存するにはlicenseと巨大なworkspace依存を伴い、GPUI部品として再利用不可と評価 |
| `eframe` / `egui` / `egui_extras` 0.36.x | MIT OR Apache-2.0 | 公開APIと標準widgetは再利用可能。breaking releaseへのadapter隔離が必要 |
| `egui_code_editor` 0.4.1 | MIT、egui 0.36 | simple editor/lexer。virtualized PR diffではない |
| `egui_tty` 0.2.0 | MIT、egui 0.36、Ghostty VT | transport分離とterminal基本機能は有望だが新規crate、Zig build、Windows/IME/a11y未検証 |
| `egui_term` 0.1.0 | MIT | 公開版はegui 0.31、mainは未公開更新。widgetがlocal PTYを所有しbackend分離には追加refactor |

GPUI main には Apache-2.0 の `gpui` から GPL の workspace crateへ到達する可能性を指摘する open issue #55470 がある。法的結論は出さず、採用検討時は pin した公開 artifact の `cargo tree` と license scan を別途実行する。egui側も第三者crateごとにlicenseを確認し、framework本体のlicenseを全ecosystemへ一般化しない。

### 実装コスト・リスク

| 観点 | GPUI | eframe/egui |
| --- | --- | --- |
| 独自widget量 | diff/editor/terminalを公開部品だけで作る場合は多い。Zed sourceは設計参考にできるが転用不可 | diff compositeは多いが標準widgetと公開第三者crateで初期量を減らせる可能性。品質要件を上げるとcustom painter/selection/a11yが必要 |
| 描画性能 | editor向けGPU/custom element実績と細かな制御。公開GPUI 0.2.2の対象workload実測なし | visible-row immediate renderingが単純。syntax/layout cacheを誤ると毎frame CPU負荷。数万行+side-by-side実測なし |
| text layout | GPUI/Zedで高度な制御実績。ただしeditor display mapを独自構築 | `Galley`/`LayoutJob`/TextEditを利用可能。row仮想化と複数行selection/IMEを同時に満たす独自設計が必要 |
| state/async | Entity/Context/executorに統合しやすいがframework型漏出に注意 | snapshotを毎frame描画しやすい。runtime/channel/cancel/lifetimeをapplicationが所有し `request_repaint` で橋渡し |
| test容易性 | pure stateは容易。GPUI TestAppContextはmainと公開版差に注意 | `Context::run`によるheadless UI testとpure state testが可能。第三者widgetのlib testも利用可能 |
| UI交換境界 | Entity/Contextをadapter内に閉じれば適合 | egui Context/Response/Idをadapter内に閉じれば適合。今回の `DiffViewport`/`TerminalTransport` は両者で共有可能 |

### 残る同一条件の実証

Windows 11 実機で両候補について次の製品相当feature sliceを比較するまでは、最終採用を確定しない。

1. 50,000 行の unified/split diffを fixed-height virtual rowで表示し、syntax color、gutter、range selection、copy、検索移動、inline comment 1件、fold 1件を操作する。
2. 1,000行の部分更新をbackgroundで適用し、全モデル再構築の有無、frame time、input latency、memoryを記録する。
3. terminalは同一のVT fixtureを流した後、WindowsConPty backendでPowerShellを起動し、keyboard/AltGr、日本語IME、selection/copy、10,000行scrollback、OSC 8 link、resizeを確認する。
4. WSL backendは別sessionとして明示選択し、Windows GUIとの間をbyte stream/resize/lifecycleだけで接続する。WindowsConPtyとの暗黙fallbackは許容しない。
5. NarratorとAccessibility Insightsでdiff row/gutter/comment、terminal grid/selection/focusのrole/name/value/actionを記録する。
6. clean machineでbuildし、Rust/MSVC/Zig/GPU依存、binary size、cold build timeを記録する。

## PRレビュー追加調査の一次情報

- [Zed editor manifest](https://github.com/zed-industries/zed/blob/main/crates/editor/Cargo.toml)
- [Zed terminal manifest](https://github.com/zed-industries/zed/blob/main/crates/terminal/Cargo.toml)
- [Zed terminal_view design notes](https://github.com/zed-industries/zed/blob/main/crates/terminal_view/README.md)
- [Zed buffer_diff manifest](https://github.com/zed-industries/zed/blob/main/crates/buffer_diff/Cargo.toml)
- [egui ScrollArea::show_rows](https://docs.rs/egui/0.36.1/egui/containers/scroll_area/struct.ScrollArea.html#method.show_rows)
- [egui TextEdit](https://docs.rs/egui/0.36.1/egui/widgets/struct.TextEdit.html)
- [egui_extras syntax highlighting](https://docs.rs/egui_extras/0.36.1/egui_extras/syntax_highlighting/)
- [egui_code_editor 0.4.1](https://crates.io/crates/egui_code_editor/0.4.1)
- [egui_tty 0.2.0](https://crates.io/crates/egui_tty/0.2.0)
- [egui_term repository](https://github.com/Harzu/egui_term)
- [GPUI license dependency report #55470](https://github.com/zed-industries/zed/issues/55470)

## Windows native 実操作と transport 実測（2026-08-23）

### 検証範囲と段階

同一 checkout から GPUI 0.2.2 と eframe/egui 0.36.1 の release sliceを Windows native `cargo.exe` でbuildし、Windows desktop上の実windowを computer-use（Windows UIA backend）と PowerShellで観測した。再現コマンド、event log、UIA snapshot相当の観測値、スクリーンショットは `spikes/windows-ui-evidence/README.md` と同directoryに保存した。

| 段階 | 到達点 |
| --- | --- |
| (a) cfg/dependency解決 | `[ローカル実測]` 両候補とConPTY smokeで `cargo metadata/check` 成功 |
| (b) compile | `[ローカル実測]` Windows Rust 1.98.0、MSVC 14.39で両GUIとsmokeをrelease compile |
| (c) link/PE生成 | `[ローカル実測]` Windows SDK 10.0.22000を使って3本のx86-64 PEを生成 |
| (d) Windows native起動 | `[ローカル実測]` 両GUIで実windowのpixel描画、click、key、scroll、background更新を画面・UIA・event logで確認。test PIDだけを停止 |
| (e) platform機能 | `[ローカル実測]` Win32/GPU描画は実画面で確認、eframeのUIA tree、ConPTYのWindows/WSL process・I/O・resize・exitを確認。`[未検証]` DirectWriteの個別品質、GPU present/frame time、実IME composition、Narrator音声、製品terminal frontend統合 |

前回のprocess/handle/DLL loadだけの観測から進み、今回は画面内容が実際に変化したことを成功条件にした。cross compileやWineは使用していないが、この結果もWindows runtime/accessibility/performance全体の確認済みを意味しない。

### 同一手順のGUI結果

両 sliceで100,000行のvirtual listを描画し、row click、`Down`選択、scroll、500 ms background update後のgeneration更新を確認した。GPUIは画面と stdout telemetry、eframeはそれに加えてUIA status/valueを照合した。GPUIの可視範囲は `0..26` から `24..50`、eframeは `0..16` から `7..24` へ変化し、単にmodelへ100,000件を置いただけではなく実window上のscrollを確認した。

| release実測 | GPUI 0.2.2 | eframe/egui 0.36.1 |
| --- | ---: | ---: |
| cold build | 153,142 ms | 85,265 ms |
| non-zero HWNDまで | 1,014 ms | 1,027 ms |
| idle 3秒 CPU | 0 ms | 0 ms |
| idle Working Set / Private | 53.74 / 94.34 MiB | 175.75 / 381.53 MiB |
| scroll操作中 CPU差分 | 15.62 ms（8 action） | 15.625 ms（8 page指定の1 action） |
| background action中 CPU差分 | 15.62 ms | 31.25 ms |

computer-use actionのwall timeはGPUI scroll 30,659 ms（8 action）、eframe scroll 3,540 ms（1 action）であったが、UIA tree取得、screenshot、WSL往復を含み、appの入力latency/frame timeとして比較できない。feature集合、renderer、accessibility node数、window sizeも異なるため、build timeとmemoryの差も順位付けには使わない。製品相当diffのsyntax layout、side-by-side、selection/copy、20 Hz更新競合、GPU frame/present timeは未測定である。

### IME、focus、UI Automation

- `[ローカル実測]` GPUI公開0.2.2 sliceと同梱`examples/input.rs`はいずれもUIA treeがwindow 1 nodeだけで、content、selection、focus、actionを公開しなかった。synthetic Unicode typeは正しい日本語文字列にならず、clipboard paste actionはexample内で文字列ではなく`ctrl-v`として扱われた。実Microsoft IMEのpreedit/変換を実行していないため、これはIME非対応の断定ではなく、今回の経路で成功証拠を得られなかったという結果である。
- `[ローカル実測]` eframeはheading、status、button、edit、可視row、scrollbarを23〜24 UIA nodeとして公開し、UIA actionによるrow選択と、clipboard経由で入れた`日本語テストabc`をeditのValueとして正しく公開した。ただし既定egui fontでは日本語glyphが豆腐表示になり、font fallback/bundlingが必要である。UIA `SetValue` 自体はvalue mismatchとなったが、標準clipboard pasteは成功した。
- `[未検証]` eframeで`Tab` event後もproviderの`focusedElementId`は`null`で、focus traversalの順序とfocus announcementは客観的に確認できなかった。実IMEの候補window位置・確定・再変換、AltGr、Narrator音声判断は両候補とも未確認である。

この差により、公開versionのままaccessibility要件を満たす初期実装コストはeframeが明確に低い。GPUI mainの未公開AccessKit実装を公開0.2.2の能力として扱わない。

### 最小ConPTY / WSL transport

`spikes/terminal-transport-smoke` をWindows nativeで実行し、Windows backendは`cmd.exe /Q /K`、WSL backendは`wsl.exe -d Ubuntu-24.04 -- bash`をConPTYへ接続した。両backendで80×24のprocess作成、marker write/read、100×30 resize、明示終了、exit code 0を確認し、WSLの`stty size`も`30 100`を返した。

初回の`portable-pty` 0.9経路はinherit-cursor modeが生成するCSI 6n queryへVT frontendがreplyしないため初期化が停止した。この観測は、transport smokeであってもfrontendのVT query reply責務を無視できないことを示す。最小transport境界はflags 0の`CreatePseudoConsole`を使う`conpty` 0.7で実証した。GUI frontendとの統合、VT parser/grid、selection/copy、font、IME、screen reader、close競合、長時間lifecycleは未確認で、製品terminalを実装済みとは扱わない。

### 採用提案とPR reviewへの影響

`[当時の推論・未承認案]` 初期のPR review UIには **eframe/egui 0.36.1を暫定第一候補** として提案する。両候補ともWindows native描画・基本操作・100,000行virtual scroll・background updateを通したが、eframeは公開versionで標準widgetの意味的UIA treeと編集Valueを提供し、公開・permissiveな部品で初期機能を組み立てるリスクが低い。これは最終採用決定ではなく、TASK-1とADRはIn Progress/既存状態を維持する。

GPUI 0.2.2は描画時memoryが小さく、`uniform_list`、text shaping、executor統合、複雑なeditor surfaceの制御余地がある。一方、公開版のcontent UIA不在、公開crateだけではZed editor/terminalを再利用できないこと、IME/clipboardの成功証拠不足から、accessibilityを初期要件に含む現時点では見送る案とする。再検討条件は、AccessKit対応版のcrates.io公開とAPI安定化、同じcustom diff row/terminal gridでNarrator・IME・focusを通すこと、20 Hz background更新とkey repeatの描画停止を再現しないことである。

eframe側の既知制約は、日本語font同梱、custom virtual diff row/gutter/commentへrole/name/value/actionを付ける作業、複数行selection/copyと仮想化の両立、third-party terminal crateのZig/Windows/a11yリスク、今回大きかったmemory値である。製品相当sliceでframe time、allocation、GPU/driver、Narrator、IMEを満たせない場合、またはGPUIの公開AccessKit版が同条件で優れる場合に再検討する。

- git diff表示: 両候補のWindows build/runtime riskは低下した。GPUI `uniform_list` とegui `show_rows`の固定高visible-rowは実動したが、syntax/gutter/split同期/selection/copy/inline commentの実装容易性は未確認である。eframeのcustom rowにも明示的accessibility metadataが必要であり、標準buttonが読めた結果をdiff widgetへ一般化しない。
- 内蔵terminal: GUI frameworkの差より、VT frontendと`TerminalTransport`境界が支配的である。ConPTY/WSLのprocess・I/O・resize・exitはframework非依存smokeで成立したためtransport riskは下がったが、GPUI/egui frontendへの統合、IME、selection/copy、screen reader、frame timeは残る。egui ecosystemの公開部品は初期工数を下げ得るが、a11y未提供のcustom-painted gridを採用根拠にしない。

残課題は、実Microsoft IME、Narrator音声とAccessibility Insights、focus traversal、custom diff/terminal accessibility、製品相当diff性能、GPU frame time、長時間background更新競合である。これらとユーザー承認が揃うまでTASK-1をDoneにせず、ADR-0001/0002も更新しない。

## 日本語font・IME追加調査（2026-08-23）

### 問題の分離

前節でeframeのTextEditへclipboard経由で`日本語テストabc`を入れた際、内部`String`、stdout、UIA Valueはいずれも正しかったが、画面は豆腐glyphになった。この結果から今回の失敗はUTF-8保持やclipboardではなく、描画時のglyph coverage不足である。日本語対応は次の三点を別々に判定する必要がある。

1. text modelがUnicodeを破損せず保持し、selection/copy/accessibility Valueへ出せること。
2. 選ばれたfont stackに日本語glyphがあり、CJKのfont metrics・全角幅・句読点・fallback runを正しくlayoutできること。
3. platform IMEのpreedit、変換、commit、候補window位置、focus変更をwidgetが処理できること。

fontを追加してもIME #3は自動的に直らず、IME backendがあってもfont #2がなければ確定文字は豆腐になる。

### eframe/egui 0.36.1

`[一次情報]` eguiの既定fontはHack、Ubuntu Light、Noto Emoji、emoji-icon-fontで、日本語本文fontを含まない。公式READMEもnon-Latin文字には`Context::set_fonts`で独自TTF/OTFを登録する必要があると明記する。`FontDefinitions`はfamilyごとにfont名の優先列を持ち、glyphがなければ次のfontへ進む。0.36.1では`Context::add_font`と`FontInsert`でも既存familyへhighest/lowest priorityで追加できる。

`[一次情報]` IMEはfontとは別に、egui-winitがwinitの`Ime::Preedit`/`Commit`を`egui::ImeEvent`へ変換する。focused TextEditは`IMEOutput`を返し、winit windowへ`set_ime_allowed`と`set_ime_cursor_area`を設定する。0.36.0 sourceにはWindows 11 Microsoft Pinyinでのevent順序と`VK_PROCESSKEY` workaroundが記録されるが、日本語Microsoft IMEの同条件保証ではない。`[ローカル実測]` 今回は日本語commit相当のclipboard Valueだけ成功し、実IME compositionは未実行である。

Review Sweeperでは、proportional UI用の日本語fallbackと、diff/terminal用のmonospace日本語fallbackを明示的に登録する必要がある。Latin用fontの後ろへ日本語fontをfallbackとして置けば、英数字の見た目を維持し、Japanese glyphだけを補える。再現性を優先する推奨案は、SIL OFL 1.1のNoto Sans JP subsetとNoto Sans Mono CJK JP相当をpinしてapplication assetとして同梱し、license noticeを配布物へ含めることである。WindowsのYu Gothic UI/Meiryoをruntimeに読む案はbinary増分を避けられるが、egui自身はsystem font discoveryを提供せず、Windows optional font状態・TTC face index・platform差をapplicationが扱う必要がある。Windows付属fontをアプリへコピー再配布することはMicrosoft FAQ上原則許可されないため、systemからその場で利用することと同梱を区別する。

font追加後に日本語label/TextEdit、PR title/path/comment、diffの全角・半角混在、terminal cell幅、selection/copy、UIA Value、Microsoft IME候補位置をWindowsで再実測する。fontのbinary/working-set/atlas増分と初回glyph rasterizationも未測定である。

### GPUI 0.2.2 / Zed

`[一次情報]` GPUI 0.2.2のWindows text systemはDirectWriteを使い、styleの`FontFallbacks`で指定したfamilyをUnicode rangeへmappingした後、`GetSystemFontFallback`のmappingを追加する。GPUI mainも同じ方針を維持し、ZedのUI/editor/terminal settingsはfallback listをframeworkへ渡す。WindowsではOSのlocale-aware font fallbackを利用できるため、eguiの既定fontだけを使う場合より日本語glyphへ到達しやすい設計である。ただし公開0.2.2でfallback指定がない全widgetの結果を今回実測しておらず、「GPUIなら常に日本語が描画できる」とは断定しない。

`[一次情報]` GPUI 0.2.2は`WM_IME_STARTCOMPOSITION`/`WM_IME_COMPOSITION`を処理し、IMM32のcomposition/result文字列を`InputHandler::replace_and_mark_text_in_range`/`replace_text_in_range`へ渡す。caret boundsから`ImmSetCompositionWindow`と`ImmSetCandidateWindow`も設定し、日本語IMEが`lparam = 0`を送るcaseも明示処理する。つまりcore backendは日本語IMEを考慮しているが、各custom editorがselection range、marked text、caret boundsを正しく実装する責任を持つ。

`[ローカル実測]` 公開0.2.2同梱`examples/input.rs`へのsynthetic typeは日本語を正しく生成せず、computer-useのclipboard pasteは`ctrl-v`文字列になった。これは実Microsoft IME試験ではなく、exampleのinput/shortcut実装も製品Zed editorより小さいため、GPUI coreのIME不成立を証明しない。一方、公開0.2.2のcontent UIA不在というaccessibility差はfont/IME改善では解消しない。

### 他のRustアプリケーションの実例

| アプリ | framework/text stack | 実際の方針 | Review Sweeperへの示唆 |
| --- | --- | --- | --- |
| Zed | Rust + GPUI | system font collectionとDirectWrite/CoreText等のplatform fallbackに加え、UI/editor/terminalごとのuser-configurable fallback listを渡す | OS fallbackだけに固定せず、用途別fallback設定とsystem fallbackを併用する |
| Rerun Viewer | Rust + eframe/egui | `FontDefinitions::default()`へInter Mediumを同梱追加するが、確認したsourceにはCJK font追加がない | 大規模egui appであること自体は日本語対応の証拠にならず、製品側font assetが必要 |
| Alacritty | Rust + winit + platform font stack | Windows既定Consolas、Linux fontconfig等を使い、Windows font fallbackを実装。IME候補位置はwinit/platform issueとして継続的に修正 | terminalはglyph fallbackとIME候補位置を独立して実機matrix化する |
| Neovide | Rust + winit/Skia系 | userがcomma区切りでfallback fontを指定でき、日本語ではNoto Sans Mono CJK JP等を選ぶ運用例がある | system/user font方式は配布sizeを抑えるが、font名・install状態・fallback順による再現差が残る |

実アプリの方針は「open fontを同梱して決定的にする」「OS/user fontを探索して軽量にする」「両者を併用する」の三類型である。Review SweeperはWindows初期targetかつレビュー内容に日本語が入り得るため、最小限のOFL日本語fallbackを同梱し、ユーザー指定fontとOS fallbackを将来追加するhybridが最も再現性が高いと推論する。これはeframeを最終採用する決定ではなく、eframe暫定案へ追加される必須条件である。

一次情報:

- [egui README: non-Latin font](https://github.com/emilk/egui/blob/main/README.md#can-i-use-egui-with-non-latin-characters)
- [egui custom font example](https://github.com/emilk/egui/blob/main/examples/custom_font/src/main.rs)
- [egui-winit 0.36.0 IME bridge](https://docs.rs/egui-winit/0.36.0/src/egui_winit/lib.rs.html)
- [GPUI 0.2.2 Windows IME source](https://docs.rs/crate/gpui/0.2.2/source/src/platform/windows/events.rs)
- [GPUI 0.2.2 DirectWrite source](https://docs.rs/crate/gpui/0.2.2/source/src/platform/windows/direct_write.rs)
- [Zed main DirectWrite fallback](https://github.com/zed-industries/zed/blob/main/crates/gpui_windows/src/direct_write.rs)
- [Rerun re_ui font setup](https://github.com/rerun-io/rerun/blob/main/crates/viewer/re_ui/src/design_tokens.rs)
- [Alacritty font configuration](https://github.com/alacritty/alacritty/blob/master/extra/man/alacritty.5.scd)
- [Alacritty changelog](https://github.com/alacritty/alacritty/blob/master/CHANGELOG.md)
- [Neovide CJK fallback investigation](https://github.com/neovide/neovide/issues/2071)
- [Noto CJK deployment formats](https://github.com/notofonts/noto-cjk/blob/main/Sans/README.md)
- [Noto/Source Han SIL OFL 1.1](https://github.com/adobe-fonts/source-han-sans/blob/master/LICENSE.txt)
- [Windows international UI fonts](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/loc-international-fonts)
- [Microsoft font redistribution FAQ](https://learn.microsoft.com/en-us/typography/fonts/font-faq)

## 日本語をblocking requirementとした再評価（2026-08-24）

PR title/path/comment/diff本文/terminalで日本語を日常利用できることを最重要のblocking requirementとし、既存順位を前提にせずGPUI 0.2.2とeframe/egui 0.36.1をWindows native release buildで再測定した。同一corpusは、ひらがな、カタカナ、一般漢字、非BMP漢字`𠮷`、日本語/Latin混在、全角/半角、precomposed濁点と`か`+U+3099、emoji/variation selector/ZWJ、日本語path、diff gutter、長文wrap/ellipsis、side-by-side、syntax span、terminal cell例を含む。証拠は`spikes/windows-ui-evidence/gpui-japanese-corpus.png`、`eframe-japanese-corpus.png`、`eframe-copy-utf8.txt`、同READMEに保存した。

### 重みと判定

| 軸 | 重み | blocking判定 |
| --- | ---: | --- |
| 日本語glyph/shape/fallbackの表示品質と再現性 | 35 | PR/diff/pathで欠落glyph不可。emoji/VS/ZWJも日常corpusに含む |
| diffのselection/copy/syntax/wrap/side-by-side | 20 | UTF-8/graphemeを壊さず、論理行とvisual rowを同期できること |
| 日本語IME | 15 | preedit/変換/commit/candidate位置をfont表示とは別に合格させる |
| UIA/Narrator | 10 | 日本語content/value/selectionを意味的に公開する |
| terminal frontend適用 | 10 | VT/gridの幅・cursor契約をframeworkから分離し描画できる |
| package/memory/初回rasterization/保守性 | 10 | font license/asset更新を再現可能にし、許容budgetを決める |

最重要35点の表示品質だけではGPUIが優位だった。DirectWrite system fallbackと`Segoe UI -> Yu Gothic UI -> Meiryo -> Segoe UI Emoji`の明示`FontFallbacks`を同一画面で実描画し、日本語、`𠮷`、combining、emoji/ZWJを視認できた。ただしinstalled font/version/localeに依存し、backslashがyen glyphになる箇所がある。system fallback成功を別Windows環境の再現性とは扱わない。

eframeは一時OFL assetのNoto Sans CJK JP proportional subsetとNoto Sans Mono CJK JP subsetを`FontDefinitions`へ明示登録した。日本語、`𠮷`、全半角、combiningは描画し、左diffをCtrl+A/Cした結果は日本語、非BMP、半角カナ、combining、ZWJのUTF-8 byte列を保持した。UIAは日本語corpusと左右diff Valueを含む19 nodeを公開した。一方、emoji単体の一部は描画してもZWJ/variation sequenceにmissing glyph boxが残り、表示blocking要件には未合格である。

GPUIのcustom `div` diffは今回selection/input handlerを持たず、Ctrl+A/C後もclipboard sentinelが不変だった。selection、UTF-8/grapheme境界、clipboard、syntax hit-test、wrap後の左右gutter同期をcustom editorへ実装する必要がある。公開0.2.2のUIAはwindow 1 nodeだけで、日本語content/value/selectionを取得できない。eframeの2つの`TextEdit`はwrap/copy/UIA Valueが成立したが、左右独立stateのため製品では論理行/visual row mappingと同期scrollを追加する。

### font asset、size、memory、初回rasterization

Debian `fonts-noto-cjk`の`NotoSansCJK-Regular.ttc` 19,484,784 bytesから評価corpusだけをFontTools 4.63.0で抽出した。UI OTF 85,700 bytes、mono OTF 81,204 bytes、OFLを含むnotice 14,712 bytes、評価package増分は合計181,616 bytes。assetは`/tmp`だけで使用し、リポジトリへ恒久追加していない。製品corpus/Unicode更新戦略、emoji font、OFL notice配置は未決定である。

eframe release PEは14,432,768 bytesで、runtime assetのためPE増分0。3秒idleの同一PEは既定font mode WS/Private 140.21/370.83 MiB、subset mode 138.63/365.87 MiBで、noiseを超える増加を検出できなかった。hot-run HWNDは既定983 ms、subset326 msだがcache順序が異なり比較値にできない。glyph atlasは利用glyphに応じて成長し、isolated cold first-raster/frame spikeは未分離なので残リスクとする。

### terminalとIME/UIAの責任分界

frameworkはterminal glyph rasterization、cell rectへのclip/hit-test、IME candidate rect、UIA公開を担う。East Asian WidthのWide/Fullwidth、Ambiguousを1/2のどちらにするか、combiningを直前cellへ付ける処理、emoji/VS/ZWJ graphemeのcell数、cursor/copy range、ConPTYのVT replyはVT parser/grid engineの責任である。今回terminal corpusが描画できたことはcursor/cell alignment成功を意味しない。

font表示とIMEも別判定とした。eframeはclipboard commit相当とUIA Value、GPUIは既存input exampleの不成功までは確認したが、computer-useではMicrosoft日本語IMEのpreedit/候補を観測できず、preedit/変換/commit/candidate位置は両候補とも未成功である。Narrator音声も未確認。UIA treeだけは日本語文字列で再取得し、eframe content/value、GPUI windowのみという差を確認した。

### 当時の選択案（履歴、2026-08-25の承認済み決定で置換）

暫定第一候補は条件付きでeframe/eguiを維持する。ただし理由は既存順位ではなく、日本語表示35点ではGPUIが上回る一方、blocking workload全体のdiff selection/copyと日本語UIAがeframe公開版で実証でき、GPUI custom実装と公開0.2.2 accessibilityのリスクがより大きいためである。以前の「CJK fontを追加すれば合格」は撤回し、次を採用前の必須条件とする。

1. OFL CJK UI/mono subsetとlicense notice、corpus/Unicode更新手順を確定する。
2. emoji/VS/ZWJ対応fontまたはrenderer経路をWindows実描画で合格させる。
3. Microsoft日本語IMEのpreedit/変換/commit/candidate位置を実測する。
4. custom diffの同期selection/copy/syntax/wrap/UIAを同一corpusで合格させる。
5. terminal gridのEast Asian Width/ambiguous/combining/emoji/cursor testを合格させる。
6. 製品corpusでfont atlasとisolated cold first-rasterを測る。

特に2が解消しない場合、または日本語表示品質をselection/UIAより絶対優先する製品判断なら、GPUIを第一候補へ変更し、custom diff/input/accessibility実装コストを受け入れる案を再提示する。ADR-0001/0002は変更せず、アーキテクチャ決定はユーザー承認まで確定しない。

## ユーザー選択font方針による再評価（2026-08-25）

ユーザー決定により、アプリへのCJK font同梱は採用前提から外し、installed fontを用途別に選べることをblocking requirementへ変更した。前節の一時OFL subset 181,616 bytesと計測値は描画・package costの参考として保持するが、採用必須条件ではない。同一日本語corpusをWindows nativeで再実行し、font picker製品UIではなく列挙、解決、validation、preview、runtime切替の最小spikeだけを追加した。

### 設定境界と障害時方針（案、製品API未確定）

設定は少なくとも `ui.proportional`、`diff.monospace`、`terminal.monospace` を分け、それぞれprimary familyと順序付きfallback family listを持つ。terminalのfont選択はglyph描画だけに作用し、East Asian Width、ambiguous width、combining、emoji cluster、cursor/cell alignmentは引き続きVT/grid engineが決定する。ユーザー指定font file pathとTTC face indexも同じvalidation層へ追加できるが、公開設定schemaや永続化形式はユーザー承認前に確定しない。

Windows初期候補はinstalled collectionから解決できたfamilyだけを提示し、UIは `Yu Gothic UI -> Meiryo UI -> Segoe UI Emoji`、diffは `Cascadia Mono -> BIZ UDゴシック -> Segoe UI Emoji`、terminalは `Consolas -> BIZ UDゴシック -> Segoe UI Emoji` を候補例とする。ただしDirectWriteのfamily名はlocale依存で、同じBIZ fontがGPUI列挙では `BIZ UDゴシック`、fontdbでは英名alias `BIZ UDGothic`も返った。registry display nameをそのまま設定値にせず、起動ごとにbackend collectionへ照合し、解決されたface/file/TTC index、用途適合（monospace）、日本語corpus previewを表示する必要がある。

未導入、削除済み、無効family、CJK glyph不足はsilent successにしない。runtime変更は検証成功後だけ適用し、失敗時はlast-known-goodを維持してrole単位の警告を返す。次回起動時に設定を解決できなければ順序付きfallbackを試し、それも失敗したroleはsystem-safe defaultへ一時復旧し、欠落familyとpreviewを明示する。CJK/emoji coverageはfamily名だけでは保証できないためblocking corpusを用いたpreviewとglyph coverage警告を設ける。

### 同一条件のWindows実測

| 観測 | GPUI 0.2.2 | eframe/egui 0.36.1 + fontdb 0.23.0 |
| --- | --- | --- |
| installed列挙 | DirectWrite system collectionを `all_font_names()` で261 family列挙 | egui単体にはsystem discoveryがなく、アプリ側 `fontdb::Database::load_system_fonts()` で316 family aliasを列挙 |
| family解決 / load | `Font { family, fallbacks }`をDirectWriteがsystem collectionから解決。アプリはfileを読まない | アプリがfamily aliasからfaceを選び、file/TTCを読み、`FontData.index`を設定して `Context::set_fonts` へ登録 |
| TTC実証 | DirectWrite内部で解決 | `YuGothM.ttc#1`、`meiryo.ttc#0`、`BIZ-UDGothicR.ttc#0`、`msgothic.ttc#0`を実際に解決 |
| runtime切替 | `f`で3 roleの `Font`/fallbackを切替え `notify()`。両presetのprimary照合true | buttonで3 roleの `FontDefinitions`を再登録。両presetでUIA Valueとcopyを維持 |
| 日本語表示 | `gpui-system-font-preset0.png` / `preset1.png`。日本語、非BMP、combining、emoji/ZWJを視認し、roleごとのfont変更も確認 | `eframe-system-font-preset0.png` / `preset1.png`。日本語、非BMP、combining、全半角を視認したが、emoji VS/ZWJにはmissing boxが残った。また既定styleのheading/labelへcustom UI familyを割り当てない箇所は豆腐表示 |
| selection/copy/UIA | custom diffはselection/copy未実装、UIAはwindow 1 nodeのみ | 左diffを切替後にCtrl+A/Cし、252-byte UTF-8を `eframe-system-font-copy.txt` に保存。日本語path、非BMP、combining、ZWJを保持。UIAも21 nodeと日本語Valueを維持 |
| 実行時cost（切替後snapshot） | PE 11,101,184 bytes、CPU 156.25 ms、WS 56.65 MiB、Private 107.18 MiB | PE 14,517,248 bytes、CPU 328.12 ms、WS 318.37 MiB、Private 545.31 MiB。font fileをfallbackごとに読み込むprototypeであり、製品化にはbyte/face cacheと重複排除が必須 |
| 初回登録 | system collection利用で独立計測なし | warm runはfont登録81 ms、first UI 609 ms。別のcold寄りrunは登録2,388 ms、first UI 3,025 msで、cache状態を統制したbenchmarkは未実施 |

GPUIでは存在しないprimary familyを指定してもrelease pathがsystem UI fontへsilent fallbackするため、`all_font_names()`での事前validationが必須である。eframeではface/file/TTC indexまで再現可能だが、system discovery、weight/style選択、file read、重複排除、削除検知、再登録をすべてアプリ側で実装する必要がある。どちらもruntime切替後の日本語PR title/path/comment/diff、全角/半角、combining、非BMPを再確認したが、font成功をIME成功やterminal cell semantics成功とは扱わない。Microsoft日本語IMEのpreedit/変換/commit/候補位置とNarrator音声は引き続き未完了である。

Windows installed fontのその場利用とfont file再配布はlicense上別である。MicrosoftのFAQはWindows device上のinstalled fontをアプリが表示・編集・出力に利用できる一方、font fileのcopy、conversion、アプリへの同梱には別のredistribution権が必要と説明する。DirectWriteはsystem collectionとcustom font file/setを別経路として提供する。一次情報は [Microsoft font redistribution FAQ](https://learn.microsoft.com/en-us/typography/fonts/font-faq)、[DirectWrite custom font sets](https://learn.microsoft.com/en-us/windows/win32/directwrite/custom-font-sets-win10)、[DirectWrite introduction](https://learn.microsoft.com/en-us/windows/win32/directwrite/introducing-directwrite)、[egui FontDefinitions](https://docs.rs/egui/latest/egui/text/struct.FontDefinitions.html)、[fontdb](https://github.com/RazrFalcon/fontdb) を参照した。ユーザー指定file path案はfileをpackageへcopyしない前提でも、当該font licenseとアクセス継続性を利用者へ委ねるvalidationが必要である。

### 承認済み決定（2026-08-25）

ユーザー承認により、Windows native UI framework は **GPUIを採用する**。eframe/eguiは第二候補・fallbackとする。正式な決定はADR-0010に記録し、ADR-0001/0002と関連付けた。

採用理由は、GPUIがDirectWrite system font collectionとsystem fallbackをnativeに利用でき、font同梱なしで用途別primary/fallbackを選択可能にしやすいこと、Windows実測で日本語・非BMP漢字・combining・emoji/ZWJの表示品質が最も高かったこと、`uniform_list`、小さいmemory footprint、framework統合executorが大量diffとbackground更新に適合すること、GPUI mainで進むAccessKit統合を含む将来性である。公開版とmainを同一視せず、差はpinと検証で管理する。

fontは同梱前提ではない。`UI proportional`、`diff/editor monospace`、`terminal monospace`を別設定とし、installed system fontのprimary familyと順序付きfallback listをユーザーが選択可能にする。無効・削除済みfont、locale依存family名、CJK glyph不足をsilent successにせず、適用前validation、解決結果とblocking日本語corpusのpreview、coverage警告、last-known-good、順序付きfallback、role別system-safe defaultを必須設計条件とする。製品の公開settings API、永続化schema、settings UI、font pickerの詳細はこのspikeで確定しない。

eframe/eguiは公開版でselection/copyと意味的UIA treeを実証できる強みを持つ。一方、installed font selectionにはsystem font discovery、familyからfile/TTC faceへの解決、file read/cache、runtimeの`FontDefinitions`再登録、app-wide style適用をアプリ側で担う必要があり、同梱なしの日本語・emoji fallbackを成立させるコストを見送り理由とする。

既知制約と採用撤回条件は次のとおりである。これらは後続作業候補として保持するが、ユーザー承認なしに新しいBacklog taskは作成しない。

- crates.io GPUI 0.2.2はcontent UIA/AccessKit treeを公開せず、Windows実測ではwindow 1 nodeのみだった。mainのAccessKit実装を公開版の能力として扱わない。
- custom diff/editorにはgrapheme-aware selection/copy、syntax span、wrap、side-by-side同期、focus、accessibility metadata/actionが必要である。
- Microsoft日本語IMEのpreedit、変換、commit、candidate位置をcustom input surfaceで実測する必要がある。
- font validation/preview、欠落設定の安全なfallback、CJK/emoji coverage警告が必要である。
- terminalはEast Asian Width、ambiguous width、combining/emoji cluster、cell幅、selection/copy、cursor alignment、IME、accessibilityをVT/grid責務として検証する必要がある。
- GPUI main/AccessKitとcrates.io公開版のAPI・behavior・安定性の差を継続管理する必要がある。

accessibility、custom diffの基本selection/copy、Microsoft日本語IME、terminal cell/cursorのいずれかを製品要件内で実現できない場合は、eframe/eguiをfallbackとして再評価する。
