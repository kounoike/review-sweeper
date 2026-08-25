---
id: TASK-1
title: Windows向けRust UIフレームワークを技術検証する
status: Done
assignee:
  - '@kounoike'
created_date: '2026-08-20 18:06'
updated_date: '2026-08-25 05:31'
labels:
  - project-setup
milestone: m-0
dependencies: []
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0010-windows-ui-gpui.md
documentation:
  - doc-5
modified_files:
  - spikes/ui-framework-gpui/src/main.rs
  - spikes/ui-framework-eframe/Cargo.toml
  - spikes/ui-framework-eframe/Cargo.lock
  - spikes/ui-framework-eframe/src/main.rs
  - spikes/windows-ui-evidence/README.md
  - spikes/windows-ui-evidence/gpui-system-font-preset0.png
  - spikes/windows-ui-evidence/gpui-system-font-preset1.png
  - spikes/windows-ui-evidence/eframe-system-font-preset0.png
  - spikes/windows-ui-evidence/eframe-system-font-preset1.png
  - spikes/windows-ui-evidence/gpui-system-font-events.log
  - spikes/windows-ui-evidence/eframe-system-font-events.log
  - spikes/windows-ui-evidence/eframe-system-font-copy.txt
  - >-
    backlog/docs/technical/ui-framework-windows/doc-5 -
    Windows-Rust-UIフレームワーク技術検証.md
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0010-windows-ui-gpui.md
type: spike
ordinal: 1
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0001の第一候補であるGPUIを中心に、WindowsネイティブGUIとしてReview Sweeperの初期実装に採用できるかを最小プロトタイプで検証する。製品コードの大部分が依存する前に、採用または代替案を確定する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 GPUIを含む候補をWindows対応、ビルド環境、非同期処理、描画性能、アクセシビリティ、保守性で比較する
- [x] #2 Windows上でウィンドウ、基本入力、スクロール可能な大量行表示、バックグラウンド処理からの更新を最小プロトタイプで確認する
- [x] #3 採用案、見送った案、既知の制約、再検討条件をADRまたは技術メモに記録する
- [x] #4 UIフレームワークを交換可能に保つ境界を後続の基盤タスクから参照できる
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. 一次情報と既存ADRを基に、GPUI・eframe/egui・Slint・IcedをWindows対応、ビルド環境、非同期処理、描画/大量行、アクセシビリティ、保守性で比較する。
2. 調査用の最小プロトタイプを製品基盤から分離して作成し、ウィンドウ、キーボード/ポインター入力、仮想化した大量行スクロール、バックグラウンド処理からのUI更新を実装する。
3. UI非依存の状態・入力・コマンド境界と、フレームワーク固有アダプターの境界をコードとテストで示し、TASK-10など後続タスクから参照できる技術メモにする。
4. Linux/WSL上でfmt・lint・test・checkを実行し、Windows実機ではPowerShell再現コマンドにより起動・入力・スクロール・更新・アクセシビリティを確認する。現環境でWindows実機検証できない場合は未検証として制約を記録し、成功扱いにしない。
5. 採用候補、見送り候補、既知制約、再検討条件を提示し、アーキテクチャ最終決定はコーディネーター経由で承認を得てから確定する。

6. Review SweeperのPR review workloadを、diff表示、terminal、再利用可能ecosystem、実装コスト/リスクの4観点に分解し、GPUI公開版/mainとeframe/eguiを一次情報・外部報告・ローカル実証・未検証に区別して比較する。

7. フル機能実装は行わず、UI非依存のdiff visible-range/selection/incremental-updateモデルとterminal frontend/backend境界を小さなpure Rust実証として追加し、両framework adapterへ持ち出さない契約をunit testで確認する。

8. Zedのeditor/terminal実装がGPUI crate単体で再利用できる範囲、egui系diff/editor/terminal crateのversion・license・保守状況を公開manifest/sourceで確認し、存在だけで容易と推定しない。Windows nativeの描画・ConPTY・IME・accessibilityは未検証として残す。

9. WSL/Windows interopとWindows側Rust/MSVC toolchainを読み取り専用で棚卸しし、可能ならWindows native cargo.exeでGPUIとeframe/eguiの同等sliceをbuildする。不可能な場合はcargo-xwin/MSVC targetを第一候補、MinGW GNU targetを補助候補としてcfg解決・compile・link/PE・起動・実機機能の各段階を区別して検証し、再現手順と未確認事項をdoc-5へ追記する。TASK-1はIn Progressを維持する。

10. Windows desktop上の客観的操作検証を再現可能にするため、GPUI prototypeへ選択・background更新・text inputの観測点を最小追加し、eframe/egui 0.36.1の同等sliceを独立spikeとして保存する。computer-use screenshot/UIA treeとPowerShell process samplingで描画・pointer/key/focus/scroll/update/Unicode入力と性能を同一手順で測り、IME composition/Narrator音声など自動化不能項目は未確認とする。portable ConPTY/WSL smoke sliceはprocess作成・I/O・resize・終了だけを検証し、製品terminal frontendは実装しない。

11. 日本語対応をglyph coverage/font fallback/IME compositionに分解し、eframe/egui 0.36.1とGPUI 0.2.2/mainの公開実装、およびZed・Rerun・Alacritty・Neovide等のRustアプリのfont配布/OS fallback/IME戦略を一次情報で比較する。fontのライセンス、binary size、再現性、custom diff/terminalへの影響を整理し、採用提案への影響をdoc-5へ追記する。

12. 同一の日本語corpusをGPUI 0.2.2（DirectWrite system fallback／明示fallback）とeframe/egui 0.36.1（OFLのproportional／monospace CJK subsetを明示登録）でWindows native実描画し、diffのselection/copy/syntax span/wrap/side-by-side、terminalの幅責務、IME/UIA、size/memory/rasterizationを客観証拠で比較する。日本語表示をblocking requirementとして重み付けし、doc-5とTASK-1 notesへ選択案を記録するが、ADRとDoneはユーザー承認まで変更しない。

13. ユーザー決定に従い、font同梱を前提とせずUI proportional・diff/editor monospace・terminal monospaceと各fallback listをユーザー選択可能にする設定境界を評価する。GPUIはDirectWrite system collectionのfamily指定とruntime切替、eframeはfontdbによるinstalled family→file/TTC index解決とFontDefinitions runtime再登録を同一日本語corpusでWindows実測し、初期値・欠落/削除/無効family・preview/error、license上のsystem利用/再配布差、選択案をdoc-5/TASK-1へ記録する。font picker製品UIとADR確定は行わない。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-23 着手時確認: 作業ツリーはclean。ADR-0001/0002はAccepted、ADR doctorは成功。実行環境はWindows 11上のWSL2 Linuxで、Windows Rust targetおよびWindows GUIセッションは未提供のため、Windows実機操作の成否はこの環境だけでは確認できない。

2026-08-23 進捗: GPUI 0.2.2の隔離prototypeを追加し、100,000行のuniform_list、click/key intent、BackgroundExecutorからの更新、UI非依存stateを実装した。WSL2ではfmt、cargo check、clippy、lib unit test 2件が成功。full cargo testはLinux側libxkbcommon不足でbinary link失敗。Windows target cargo checkはMSVC lib.exe不在でring build scriptが失敗したため、いずれもWindows実機検証の成功とは扱わない。現行mainのGPUI READMEはWindowsでWin32/DirectWrite、追加feature不要と明記する一方、crates.io 0.2.2同梱READMEはmacOS/Linuxのみで記載がずれている。比較・交換境界・再現手順はdoc-5へ記録し、採用最終決定は承認待ち。

2026-08-23 部分検証: 受け入れ条件 #1 はdoc-5の6軸比較、#4はdoc-5の依存方向・境界規則とprototypeのUiIntent/ReviewUiStateで確認した。#2はWindows実機がなく未完了。#3は比較、既知制約、再検討条件を記録済みだが採用案が未承認のため未完了。git diff --checkは成功。

2026-08-23 追加調査と判断保留: コーディネーター確認の結果、候補の暫定採用を含むアーキテクチャ決定は保留となった。GPUIは公開crate 0.2.2の同梱README、公開ソース、mainでWindows対応範囲が異なり、公開版にはWindows backend/IME実装がある一方、mainで追加されたAccessKitは未公開かつZed製品上もexperimentalである。GPUI mainにはWindowsで持続的key入力と10–20 Hz background notifyが重なる描画停止報告がある。eframe/eguiはAccessKitとvisible-row APIを一次情報で確認したが、0.36.1のNarrator/IME/focus、大量行性能は未実測。Slint/Icedも同一基準で再整理した。doc-5では一次情報・ローカル実測・外部報告・未検証を区別し、採用候補の順位付けを撤回してWindows実機matrixを具体化した。TASK-1はIn Progress、AC #2/#3は未完了、ADRは未確定のままとする。

2026-08-23 最終確認: cargo fmt --check、cargo test --lib（2件）、cargo clippy --all-targets -D warnings、cargo check、git diff --check、adr-doctor は成功した。依存crate proc-macro-error2のfuture incompatibility warningを観測した。Windows native build/run、full binary test、性能、Narrator/UI Automation、IMEは未完了であり、Doneへ変更しない。

2026-08-23 継続計画: PR review向けdiff/editorと内蔵terminalについて、GPUI対eframe/eguiの実現容易性を追加評価する。Zed実績と外部crateは依存関係・公開範囲・license・更新状況まで確認し、Windows native frontendと将来のWSL backend（process/PTY側）を明示的なportで分離する。採用決定は行わず、TASK-1/ADRの状態を維持する。

2026-08-23 PRレビュー機能の追加評価: Zed mainのeditor/buffer_diff/terminal/terminal_viewを確認し、unified/split diff、syntax、gutter、selection、fold/search、review comment、Alacritty-based terminalの実績はあるが、これらはpublish=false・GPL-3.0-or-laterのZed workspace crateでありGPUI単体部品として再利用できないと整理した。eframe/eguiはshow_rows、LayoutJob/TextEdit/AccessKitと第三者crateを利用できるが、egui_code_editorは巨大diffを仮想化せず、egui_tty 0.2.0は新規かつZig必須・ConPTY/a11y未提供である。WSLではegui_ttyのlib tests 52件と、追加したDiffViewport/TerminalTransport境界を含むprototype lib tests 4件が成功したが、Windows GUI/ConPTY/IME/Narrator/性能は未検証。比較とWindows同一条件sliceはdoc-5へ追記し、採用決定は保留、TASK-1はIn Progress、ADRは未確定のまま維持する。

2026-08-23 継続作業の検証: prototypeはcargo fmt --check、lib test 4件、clippy --all-targets -D warnings、cargo checkが成功し、git diff --check、backlog-check、adr-doctorも成功した。egui_tty 0.2.0のlib test 52件はWSL上で成功。依存proc-macro-error2のfuture incompatibility warningは継続。Windows native、ConPTY、IME、Narrator、GPU性能は未検証のためAC #2/#3とDoD #1/#3は変更せず、Done/final summary/ADR更新を行わない。

2026-08-23 WSLからのWindows追加検証: WSL2/interop有効、Windows cargo/rustc 1.98.0、Visual Studio Community 2022のMSVC 14.39とWindows SDK 10.0.22000を確認した。VsDevCmd.batを子cmd.exe内だけに適用し、UNC checkout上のGPUI 0.2.2 prototypeはcargo metadata、compile、linkに成功して24,370,176-byteのx86-64 PEを生成した。eframe/egui 0.36.1も100,000行show_rows、key/pointer selection、background request_repaintを含む一時sliceでcompile/linkし47,931,392-byteのPEを生成した。両exeはWSLからWindows native processとして5秒間起動し、process aliveと非zero Win32 window handleを確認後、検証用processだけを終了した。GPUIでD3D11/DirectWrite/DXGI、eframeでD3D12/DXGI/UIAutomationCore等のDLL loadを確認したが、画面内容、入力操作、GPU実描画、ConPTY、IME、Narrator/UI Automation tree、focus、性能は未確認である。cargo-xwin/MinGWはnative経路が成立したため未導入・未実行とし、代替条件、SDK/CRT/linker/C++依存、Microsoft license注意、再現コマンドをdoc-5へ記録した。build成功によりdiff visible-rowとGUI frontendのWindows build riskは低下したが、PR diffの実性能とterminal runtimeの実現容易性順位は変更しない。TASK-1はIn Progress、AC #2/#3とDoD #1/#3は未完了のまま維持する。

2026-08-23 追加調査の文書検証: git diff --check、mise run backlog-check、mise run adr-doctorは成功した。Windows build用に作成した%LOCALAPPDATA%\Temp配下のGPUI target、eframe source/targetは、生成物とprocess/module情報を記録後に対象パスを検証して削除した。Windows cargo registry cacheには通常のdependency downloadが残る。

2026-08-23 Windows desktop実操作: Windows native release buildのGPUI 0.2.2とeframe/egui 0.36.1を、test PIDだけ起動・停止してcomputer-use UIA backendとPowerShellで観測した。両候補で実pixel描画、row click、`Down`選択、100,000行virtual listのscroll、500 ms background更新後のgeneration 0→1をscreen、event log、eframeではUIA statusでも確認した。non-zero HWNDまでGPUI 1,014 ms/eframe 1,027 ms、idle 3秒CPUは両方0 ms、idle WS/PrivateはGPUI 53.74/94.34 MiB、eframe 175.75/381.53 MiB。scroll CPU差分はGPUI +15.62 ms（8 action）、eframe +15.625 ms（8 page指定の1 action）、background actionは+15.62/+31.25 msだった。computer-use wall timeはUIA/screenshot/WSL往復を含みframe timeではなく、feature集合も違うため単純順位付けしない。GPU present/frame timeと20 Hz継続更新競合は未測定。

2026-08-23 accessibility/IME: GPUI公開0.2.2はprototypeと同梱input exampleのどちらもUIA window 1 nodeだけで、content/focus/selection/actionを公開しなかった。synthetic Unicode入力は正しい日本語にならず、clipboard pasteはinput example内で`ctrl-v`として扱われたため、IME成功証拠は得られなかった。eframeはheading/status/button/edit/visible row/scrollbarを23〜24 UIA nodeとして公開し、clipboard経由の`日本語テストabc`を内部値・UIA Value・event logで確認したが、既定fontでは日本語が豆腐表示になった。実Microsoft IME composition/候補位置、AltGr、Narrator音声、Tab focus順序は未確認で成功扱いにしない。

2026-08-23 terminal transport: Windows native `conpty` 0.7 smokeで`cmd.exe`と`wsl.exe -d Ubuntu-24.04 -- bash`のprocess作成、marker I/O、80x24→100x30 resize、明示exit、exit code 0を確認し、WSLの`stty size`も`30 100`を返した。先行`portable-pty` 0.9はinherit-cursorによるCSI 6n queryへfrontend replyがなく初期化停止したため、VT reply責務を記録してflags 0の最小ConPTY境界へ切り替えた。GUI frontend統合、VT grid、IME、selection/copy、screen reader、長時間lifecycleは未確認。

2026-08-23 採用案（承認待ち）: 初期PR review UIはeframe/egui 0.36.1を暫定第一候補として提案する。両候補のWindows描画/基本操作は成立したが、eframeは公開版で意味的UIA treeとTextEdit Valueを提供し、公開permissive部品で初期実装するリスクが低い。GPUI 0.2.2はmemory、`uniform_list`、text shaping、executorに利点がある一方、公開版UIA不在とIME/clipboard成功証拠不足のため現時点の見送り案。AccessKit版のcrates.io公開、同じcustom diff/terminalのNarrator・IME・focus、20 Hz更新競合を通せば再検討する。eframeも日本語font、custom diff row accessibility、selection/copy、製品相当performance、terminal grid a11yを満たせなければ再検討する。詳細・スクリーンショット・再現手順はdoc-5と`spikes/windows-ui-evidence/README.md`。TASK-1/ADRはユーザー承認と残る実機確認まで確定・Doneにしない。

2026-08-23 検証完了: GPUI/eframe/terminal smoke各crateでcargo fmt --check、cargo clippy --all-targets -D warnings、cargo test、cargo checkが成功（GPUI lib 4件、eframe/terminal 0件）。eframe/terminalのLinux cargo buildと、3 crateのWindows native cargo build --release --lockedが成功し、ConPTY Windows/WSL実行もRESULT=ok。GPUIのLinux full binary cargo buildだけは既知のWSL system dependency不足（-lxkbcommon/-lxkbcommon-x11）で失敗したが、対象Windows native PEの再buildは成功。git diff --check、backlog-check、adr-doctorは成功。客観的screenshot/UIA/event logがAC #2を、doc-5の採用案・見送り理由・制約・再検討条件がAC #3を満たすため両方をcheckedとする。DoD #1/#3とTASK Doneは、残るIME/Narrator/focus/製品相当性能およびユーザー承認のため未checkのまま維持する。

2026-08-23 日本語対応追加調査: eframe実測の豆腐表示はUnicode保持/clipboard/AccessKitではなく、既定font（Hack/Ubuntu Light/emoji系）に日本語glyphがないことが原因。egui 0.36.1はFontDefinitions/FontInsertでfallbackを追加でき、egui-winitはPreedit/Commit、IME allowed、cursor areaを別経路で扱う。暫定eframe案にはSIL OFLのNoto Sans JP subset（UI）とNoto Sans Mono CJK JP相当（diff/terminal）をpinして同梱し、license notice、binary/atlas増分を測る条件を追加する。Windows Yu Gothic UI/Meiryoをruntime利用する案は配布sizeを抑えるがoptional font/TTC/platform差があり、Windows font自体の再配布は原則不可。GPUI 0.2.2はDirectWrite system fallbackと明示FontFallbacks、IMM32 composition/result/candidate position処理を持ち、設計上はWindows日本語へ到達しやすいが、custom InputHandler実装が必要で今回の実IME成功証拠はない。Zedはsystem+user fallback、RerunはInterのみ同梱、Alacritty/NeovideはOS/user指定fallbackを使っており、実アプリでもfontとIMEを別問題として扱う。推奨順位は維持するが、eframe採用条件に日本語font同梱とMicrosoft IME実測を明記した。詳細はdoc-5「日本語font・IME追加調査」。

2026-08-24 日本語blocking再評価: 同一corpusをWindows native release buildでGPUI 0.2.2（DirectWrite system fallback/明示FontFallbacks）とeframe/egui 0.36.1（一時OFLのNoto Sans CJK JP proportional 85,700 bytes / Noto Sans Mono CJK JP 81,204 bytesを明示登録）へ実描画した。GPUIは日本語、非BMP𠮷、combining、emoji/ZWJを最も完全に表示したが、custom diff selection/copyは未実装でclipboard sentinel不変、UIAはwindow 1 node。eframeは日本語/非BMP/全半角/combining、diff Ctrl+A/CのUTF-8保持、日本語UIA Value 19 nodeが成立したが、emoji ZWJ/variation sequenceにmissing glyphが残った。

選択案（承認待ち）: 暫定第一候補は条件付きでeframe/eguiを維持する。日本語表示だけならGPUIが優位だが、日常blocking workload全体のdiff selection/copyと日本語UIAでeframeの公開版実証が上回るため。ただしOFL CJK UI/mono subset+notice、emoji/VS/ZWJ表示、Microsoft日本語IME preedit/変換/commit/candidate位置、custom diff同期/UIA、terminal幅/cursor、font atlas/isolated cold rasterを全て採用前必須とする。emoji表示を解消できない場合または表示品質を絶対優先する判断ならGPUIへ順位変更し、custom diff/input/accessibilityコストを受け入れる。ADRは未変更。

証拠: spikes/windows-ui-evidence/gpui-japanese-corpus.png、eframe-japanese-corpus.png、eframe-copy-utf8.txt、README.md。OFL評価assetは/tmpのみで、恒久追加なし。eframe PE 14,432,768 bytes、package一時asset+notice 181,616 bytes。3秒idle同一PE比較ではfont追加のmemory増をnoise超で検出できず、初回rasterizationは未分離。Microsoft日本語IME/Narrator/terminal cursor alignmentは未成功・未確認として残す。

2026-08-24 検証: 各spikeのcargo fmt --check、cargo clippy --all-targets -D warnings、cargo checkは成功。eframe/terminalはcargo testとcargo build成功。GPUI cargo test --libは4件成功、Windows native cargo build --release --lockedは最終sourceで成功した。GPUIのLinux full cargo test/buildのみ既知のWSL system dependency不足（-lxkbcommon/-lxkbcommon-x11）でlink失敗し、成功扱いにしない。rootのmise run fmtもCargo.tomlがrootにない設定上の理由で失敗した。git diff --check、mise run backlog-check、mise run adr-doctorは成功。TASK-1はIn Progress、DoD未check、ADR未変更を維持する。

2026-08-25 ユーザー決定: アプリへのfont同梱を採用前提とせず、ユーザーが用途別fontとfallback listを選択できる要件へ変更する。前回の一時OFL subset実測は描画・size参考値として保持するが、OFL CJK font同梱とnoticeを採用必須条件から外す。

2026-08-25 installed font再実測: 設定境界をUI proportional、diff/editor monospace、terminal monospaceの3 roleと各primary/fallback listに分離した。GPUIはDirectWrite system collectionから261 familyを列挙し、Yu Gothic UI/Meiryo UI、Cascadia Mono/Consolas、BIZ UDゴシックを事前validation後にruntime切替して日本語corpusを再描画した。DirectWrite family名はlocale依存で、registryのBIZ UDGothic等をそのまま使うと無効になり得るうえ、GPUI release pathは無効primaryをsystem UIへsilent fallbackするため、起動時照合・resolved preview・欠落警告が必須。

eframe/eguiはsystem discoveryを持たないためfontdb 0.23.0をprototypeに追加し、316 aliasから実file/TTC faceを解決した。YuGothM.ttc#1、meiryo.ttc#0、BIZ-UDGothicR.ttc#0、msgothic.ttc#0をFontData.index付きで読み、3 roleのFontDefinitionsをruntime再登録した。切替後も左diffの252-byte UTF-8 copyと日本語UIA Value 21 nodeは成立したが、custom familyを割り当てないheading/labelの豆腐とemoji VS/ZWJ missing boxが残った。prototype snapshotはGPUI PE 11,101,184 bytes・WS/Private 56.65/107.18 MiB、eframe PE 14,517,248 bytes・318.37/545.31 MiBで、eframe側はfull font fileのread/cache重複排除が未実装。warm登録81 ms/first UI 609 ms、cold寄り別runは2,388/3,025 msで、統制benchmarkは残る。

障害時案: runtime変更はvalidation成功後のみ適用してlast-known-goodを維持し、再起動時に削除/無効familyなら順序付きfallback、その後role別system-safe defaultへ復旧する。resolved face/file/TTC index、monospace適合、blocking日本語corpus previewとcoverage警告を表示する。system fontのその場利用はWindows上で許される一方、font fileのcopy/同梱は別のredistribution権が必要。ユーザー指定file pathは評価可能だが製品APIを未確定とする。terminal cell幅はVT/grid engine責務のまま。

選択案（承認待ち）: font選択可能要件を最重視し、暫定第一候補をGPUIへ変更する案を提示する。DirectWrite native collection/fallback/runtime切替で同梱なしの日本語/emoji表示が最も完全で、アプリによるfull font複製を避けられるため。GPUIの必須条件はlocale差を吸収するfamily identity/validation/preview、missing設定移行、custom diffのgrapheme-aware selection/copy/syntax/wrap/side-by-side同期、日本語UIA/AccessKit・Microsoft IME、terminal grid幅/cursor試験。公開0.2.2のwindow-only UIAまたはcustom editor/input工数が解消しない場合、font registry/cacheとemoji rendererを実装する条件でeframeを再検討する。ADRと最終採用は未変更。証拠はdoc-5とspikes/windows-ui-evidence/README.md、gpui-system-font-preset{0,1}.png、eframe-system-font-preset{0,1}.png、両events.log、eframe-system-font-copy.txt。

2026-08-25 検証: GPUI/eframeともcargo fmt --check、cargo clippy --all-targets -D warnings、cargo checkが成功。GPUI cargo test --libは4件成功、eframe cargo testは0件成功、eframe Linux cargo buildも成功。両候補のWindows native cargo.exe build --release --lockedは最終sourceで成功し、再起動でGPUI HWND=10361694、eframe HWND=14555554のnon-zero windowを確認して対象PIDだけ停止した。GPUI Linux full cargo buildは既知のWSL system dependency不足（-lxkbcommon/-lxkbcommon-x11）でlink失敗し、成功扱いにしない。git diff --check、mise run backlog-check、mise run adr-doctorは成功。Microsoft日本語IME/Narrator、GPUI custom diff selection/UIA、terminal cursor/cell alignment、統制cold font benchmarkは残るためTASK-1はIn Progress、DoD未check、ADR未変更を維持する。

2026-08-25 ユーザー承認済み決定: Windows native UI frameworkはGPUIを採用する。DirectWrite system font collection/fallback、font同梱なしの用途別primary/fallback設定、日本語表示の実測品質、uniform_list・memory・executor、GPUI/AccessKitの将来性を根拠とし、eframe/eguiは公開selection/copy/UIAの強みを持つ第二候補・fallbackとした。fontはUI proportional、diff/editor monospace、terminal monospaceを分離し、無効/CJK不足のvalidation、preview、安全なfallbackを必須条件とする。公開GPUI 0.2.2のcontent UIA不足、custom diff selection/copy/accessibility、Microsoft日本語IME、font validation/preview、terminal cell幅/cursor、main/AccessKitとの差を採用撤回条件としてADR-0010/doc-5へ記録した。製品API/settings UIは未確定で、follow-up taskは作成しない。

2026-08-25 最終検証: GPUI/eframe/terminal各spikeでcargo fmt -- --check、cargo clippy --all-targets -- -D warnings、cargo checkが成功。GPUI cargo test --libは4件成功し、eframe/terminal cargo testおよびcargo buildは成功した。GPUIのLinux full cargo test/buildはWSLにlibxkbcommon/libxkbcommon-x11がないためbinary linkだけ失敗し、成功扱いにしていない。一方、対象Windows native release build --lockedは3 crateすべて成功し、GPUI HWND=139194、eframe HWND=5118662の起動を確認して対象PIDだけ停止、ConPTY Windows/WSL smokeはexit code 0とRESULT=okだった。repo rootのmise run fmt/lint/test/check/buildはrootにCargo.tomlがない構造上の理由ですべて実行不能であり、上記個別manifest検証を正本とする。git diff --check、mise run backlog-check、mise run adr-doctor、mise run adr-listは成功した。既知のproc-macro-error2 future-incompatibility warningは継続する。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Windows native UI frameworkとしてGPUIを採用するユーザー承認を、Accepted ADR-0010とdoc-5へ正式記録し、ADR-0001/0002およびTASK-1へリンクした。DirectWrite system font collection/fallback、日本語表示品質、uniform_list・memory・executor、GPUI/AccessKitの将来性を採用根拠とし、font非同梱、用途別primary/fallback、validation・preview・安全なfallbackを必須設計条件、eframe/eguiを第二候補とした。公開0.2.2のcontent UIA不足、custom diff selection/copy/accessibility、Microsoft日本語IME、terminal cell幅/cursor、main/AccessKitとの差は採用撤回条件として残し、製品API/settings UIやfollow-up taskは確定していない。各spikeの個別fmt/clippy/test/check/build、Windows native build/run、ConPTY Windows/WSL smoke、git diff --check、backlog-check、adr-doctorで検証した。
<!-- SECTION:FINAL_SUMMARY:END -->
