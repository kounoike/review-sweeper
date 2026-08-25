# Windows native UI / terminal 実測記録

2026-08-23、Windows 11 host 上の WSL2 から Windows native Rust 1.98.0 と Visual Studio Community 2022（MSVC 14.39、Windows SDK 10.0.22000）を呼び出した。測定対象は `--release --locked` の GPUI 0.2.2 slice と eframe 0.36.1 slice で、製品相当の diff renderer や terminal frontend ではない。スクリーンショットと stdout log は本ディレクトリに保存した。

## GUI の確認結果

| 観測 | GPUI 0.2.2 | eframe/egui 0.36.1 |
| --- | --- | --- |
| cold build | 153,142 ms | 85,265 ms |
| process start から non-zero HWND | 1,014 ms | 1,027 ms |
| 3秒 idle CPU | 0 ms | 0 ms |
| idle Working Set / Private | 53.74 / 94.34 MiB | 175.75 / 381.53 MiB |
| pointer click | row 1 を選択。画面と `EVENT intent=SelectRow(1)` で確認 | UIA button action で row 1 を選択。UIA status と `EVENT pointer=click selected=1` で確認 |
| keyboard | `Down` で row 2 へ移動。画面と event log で確認 | `Escape` で widget focus を解除後、`Down` で row 2 へ移動。UIA status と event log で確認 |
| 100,000行 list scroll | 可視範囲 `0..26` から `24..50`。8回の computer-use scroll 中 CPU +15.62 ms、WS 53.77 MiB、Private 94.32 MiB | 可視範囲 `0..16` から `7..24`。8 page 指定の1 action 中 CPU +15.625 ms、WS 177.11 MiB、Private 382.14 MiB |
| background update | 500 ms 後に generation 0→1。action 中 CPU +15.62 ms、WS 54.09 MiB | 500 ms 後に generation 0→1。action 中 CPU +31.25 ms、WS 177.12 MiB |
| computer-use action wall time | scroll 8 action 合計 30,659 ms、update 3,935 ms | scroll 1 action 3,540 ms、update 3,876 ms |
| UI Automation tree | window 1 nodeのみ。内容、focus、選択、actionは非公開 | heading/status/button/edit/可視row/scrollbar の23〜24 node。編集値 `日本語テストabc` も公開 |

computer-use action wall time は screenshot、Windows UIA snapshot、WSLとの往復を含むため、アプリの入力 latency や frame timeではない。両 slice は feature 集合、renderer、accessibility tree、既定 window sizeが異なり、cold build・memory値を単純な優劣に使えない。GPU present/frame time、driver別性能、継続 20 Hz update と key repeat の競合は未測定である。

画像の意味は次のとおり。

- `gpui-initial.png`、`eframe-initial.png`: 実描画された初期 window。
- `*-click.png`、`*-keyboard.png`、`*-scrolled.png`、`*-background.png`: 各操作後の可視状態。対応する `*-events.log` が state transition を記録する。
- `gpui-input-*.png`: crates.io GPUI 0.2.2 同梱 `examples/input.rs` の Windows native 実行。synthetic `type-text` の日本語は正しい文字列にならず、clipboard paste action は文字列ではなく `ctrl-v` を入力した。
- `eframe-unicode.png`: clipboard 経由で `日本語テストabc` が内部値と UIA Value に正しく入った。既定 egui fontでは日本語 glyph が豆腐表示になったため、製品では日本語 font fallback/bundlingが必要。
- `eframe-tab.png`: `Tab` 自体は処理されたが、provider の `focusedElementId` は `null` のままで focus traversal の意味的確認には不足した。

実 Microsoft IME の preedit、変換、候補 window位置、確定、AltGr は自動化していない。Narrator の音声内容も未確認である。eframe は UIA tree の存在と品質を確認できたが、Narrator 読み上げ品質を確認済みとは扱わない。GPUI公開版は window以外の UIA nodeを出さず、公開 0.2.2 の accessibility要件を満たす証拠は得られなかった。

## 日本語blocking corpusの同一条件再評価（2026-08-24）

PR title/path/comment/diff本文/terminalで日本語を日常利用できることをblocking requirementとして、同一corpusを両候補のWindows native release buildで実描画した。corpusは、ひらがな、カタカナ、一般漢字、非BMP漢字`𠮷`、日本語/Latin混在、全角/半角、precomposed濁点と`か`+U+3099、emoji/variation selector/ZWJ、`C:\レビュー\差分\日本語ファイル.rs`、diff gutter、長文wrap/ellipsis、side-by-side、syntax span、terminal cell例を含む。

### 客観的結果

| 観測 | GPUI 0.2.2 | eframe/egui 0.36.1 |
| --- | --- | --- |
| glyph実描画 | `gpui-japanese-corpus.png`。DirectWrite system fallbackと`Segoe UI -> Yu Gothic UI -> Meiryo -> Segoe UI Emoji`の明示`FontFallbacks`を同じ画面に描画し、日本語、`𠮷`、combining、emoji/ZWJを視認できた | `eframe-japanese-corpus.png`。一時OFL assetを`FontDefinitions`へ明示登録し、日本語、`𠮷`、全半角、combiningを視認できた。emoji単体の一部は描画するが、ZWJ/variation sequenceにmissing glyph boxが残った |
| font再現性 | system fallbackと明示family listの両方が成立。ただしWindowsのinstalled font/version、locale、DirectWrite fallback順に依存し、backslashが日本語fontでyen glyphになる箇所もある | Noto Sans CJK JP proportional subsetとNoto Sans Mono CJK JP subsetをpinすればCJK glyphは決定的。emoji用font/shapingは別途必要 |
| selection/copy | 今回のcustom `div` diffはselection/input handlerを実装せず、Ctrl+A/C後もclipboard sentinelが不変。GPUI同梱input example相当のselection、UTF-8/grapheme境界、clipboard実装をcustom diffへ持ち込む必要がある | 左diff `TextEdit`をCtrl+A/Cし、`eframe-copy-utf8.txt`で日本語、`𠮷`、半角カナ、combining、ZWJのUTF-8 byte列保持を確認 |
| UIA | window 1 nodeだけ。日本語content/value/selectionを取得不可 | 19 node。日本語corpusと左右diffのValueを取得し、copy対象文字列とも一致 |
| side-by-side / wrap / gutter | 両列を同じflex幅で描画したが、wrap後の高さとgutter同期、selection、syntax hit-testはアプリ側custom layoutの責任 | 2つの`TextEdit`でwrap・gutter・copyは成立。ただし左右の独立編集stateなので製品では論理行/visual row mappingと同期scrollが必要 |
| terminal | frameworkはglyph rasterizationだけを担う。cell幅、cursor位置、selectionは未実装 | 同左 |

### 一時font assetとコスト

- Debian `fonts-noto-cjk`の`NotoSansCJK-Regular.ttc`（19,484,784 bytes）から評価corpusだけをFontTools 4.63.0で抽出した。UI proportional OTFは85,700 bytes、diff/terminal monospace OTFは81,204 bytes、OFLを含むpackage copyright noticeは14,712 bytes、合計181,616 bytesである。大容量fontをリポジトリへ恒久追加していない。
- SHA-256はUI `022c9b776fafbd027c7ca2e04c0092f4a092e5ba35cc9f7825c46f96f7cd8e9c`、mono `954960caab27ebb948617767248f3e80b1fb48eb07a043b1f759cdca494acd4e`、notice `849f4ea9c214fa4ac3593b770c699f387534b11ce671264c1b10d85bdcb5997b`。subsetは一時assetで、製品corpus/Unicode更新戦略とOFL notice配置は未決定である。
- eframe release PEは14,432,768 bytesで、fontは実行時assetのためPE増分0 bytes、評価package増分181,616 bytes。3秒idleの同一PE比較は既定font modeがWS/Private 140.21/370.83 MiB、subset modeが138.63/365.87 MiBで、測定noiseを超える増加を検出できなかった。font登録後のglyph atlasは利用glyphに応じて成長するため、製品corpusの初回rasterization/frame spikeは未分離である。
- non-zero HWNDまでのhot-runは既定font 983 ms、subset 326 msだったがcache順序が異なるため、fontで高速化した証拠にも劣化なしの証拠にも使わない。isolated cold first-raster計測は残課題である。

### terminal幅の責任分界

GPUI/eguiはいずれも文字列をshape/rasterizeできるが、terminalのcell semanticsは提供しない。East Asian WidthのWide/Fullwidth、Ambiguousを1/2のどちらにするか、combiningを直前cellへ付ける処理、emoji/VS/ZWJ graphemeを1 clusterかつ何cellにするか、cursor/copy range、ConPTYのVT cursor query応答はVT parser/grid engineの責任である。framework adapterはgridが決めたcell rectへのglyph描画、clip、hit-test、IME候補rect/UIA公開を担当する。したがって今回のterminal行が視覚的に描画できたことはcursor/cell alignment成功を意味しない。

### IME / UIA / Narratorの限界

font表示とIMEは別判定とした。eframeはclipboard commit相当の日本語とUIA Value、GPUIは既存input exampleの不成功までは確認したが、computer-useはMicrosoft日本語IMEのpreedit文字列、変換候補選択、candidate window位置を観測するAPIを提供しないため、実IMEのpreedit/変換/commit/候補位置は未成功のままである。Narrator音声も自動記録していない。UIA treeは日本語文字列で取得し、eframeはcontent/valueを公開、GPUI公開0.2.2はwindowのみという差を再確認した。

### 日本語を最重視した選択案（承認待ち）

表示品質だけならGPUIが優位で、system/explicit DirectWrite fallbackの双方が同一corpusを最も完全に描画した。しかしblocking workload全体にはdiff selection/copyと日本語UIAも含まれ、公開GPUI 0.2.2はその実装・証拠がない。eframeはCJK font asset管理という明示コストとemoji ZWJ欠落がある一方、UTF-8 selection/copyと日本語UIA Valueが公開版で成立した。

よって暫定第一候補は条件付きでeframe/eguiを維持するが、以前の「fontを追加すれば合格」から厳格化する。採用の必須条件は、(1) OFL CJK UI/mono subsetとlicense notice、(2) emoji/VS/ZWJ対応fontまたはrenderer経路、(3) Microsoft日本語IMEのpreedit/変換/commit/candidate位置、(4) custom diffの同期selection/copy/UIA、(5) terminal gridの日本語幅/cursor試験である。特に(2)が解消しない場合、または日本語表示品質をselection/UIAより絶対優先する判断なら、GPUIを第一候補へ変更し、custom diff/input/accessibility実装コストを受け入れる案を再提示する。ADRは変更しない。

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

### 更新した選択案（承認待ち）

font選択可能という要件を最重視すると、暫定第一候補を **GPUIへ変更する案** を提示する。DirectWrite system collection、family fallback、runtime切替がnative経路にあり、同梱なしで日本語/emoji表示が最も完全かつ、アプリがfull font fileを複製するmemory/load costを避けられるためである。eframe/eguiはselection/copy/UIAで依然優位だが、installed font selectorを成立させるためのdiscovery/resolution/TTC/cache層がアプリ責務で、custom familyのapp-wide style適用漏れとemoji sequence欠落も残るため今回は第二候補とする。

GPUI採用の必須条件は、(1) locale差を吸収するfamily identity/validationとpreview、(2) missing/deleted settingの安全な移行、(3) custom diffのgrapheme-aware selection/copy/syntax span/wrap/side-by-side同期、(4) 日本語UIA/AccessKitとMicrosoft IME、(5) terminal grid幅/cursor試験である。公開GPUI 0.2.2のwindow-only UIAまたはcustom editor/input工数が解消しない場合は、font registry/cacheとemoji rendererをアプリ側で実装する条件付きでeframeへ戻す。これはADRや最終採用ではなく、ユーザー承認待ちの選択案である。

## ConPTY / WSL transport smoke

`terminal-transport-smoke` を Windows nativeで buildし、Windows backend は `cmd.exe /Q /K`、WSL backend は `wsl.exe -d Ubuntu-24.04 -- bash` を ConPTY に接続した。どちらも 80×24 で process作成、markerのwrite/read、100×30へのresize、明示 exit、exit code 0を確認した。WSL側の `stty size` は実際に `30 100` を返した。

```text
BACKEND=windows READY-WINDOWS INPUT=INPUT-WINDOWS SIZE-AFTER=100x30 EXIT-WINDOWS EXIT_CODE=0 RESULT=ok
BACKEND=wsl READY-WSL INPUT=INPUT-WSL SIZE-AFTER=30 100 EXIT-WSL EXIT_CODE=0 RESULT=ok
```

最初に試した `portable-pty` 0.9 は `PSEUDOCONSOLE_INHERIT_CURSOR` と Win32 input modeを有効にし、ConPTY が送る CSI 6n cursor-position queryへVT frontendが応答しない smokeでは初期化が停止した。このため transport境界だけを見る最小実証は flags 0 で `CreatePseudoConsole` を呼ぶ `conpty` 0.7へ切り替えた。この結果は product terminalに必要な VT parser/reply、grid描画、selection/copy、IME、screen reader、GUI lifecycle統合を確認するものではない。

## 再現コマンド

Developer Command Prompt相当の環境を子 `cmd.exe` 内だけに適用し、checkoutを Windows pathへ変換して実行する。

```powershell
cmd.exe /d /s /c '"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64 && cargo.exe build --release --locked --manifest-path <checkout>\spikes\ui-framework-gpui\Cargo.toml --target-dir %LOCALAPPDATA%\Temp\review-sweeper-task1-gpui'
cmd.exe /d /s /c '"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64 && cargo.exe build --release --locked --manifest-path <checkout>\spikes\ui-framework-eframe\Cargo.toml --target-dir %LOCALAPPDATA%\Temp\review-sweeper-task1-eframe'
cmd.exe /d /s /c '"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64 && cargo.exe build --release --locked --manifest-path <checkout>\spikes\terminal-transport-smoke\Cargo.toml --target-dir %LOCALAPPDATA%\Temp\review-sweeper-task1-terminal'
```

GUIは Windows desktop 上で test processだけを起動し、`orca-ide computer get-app-state/click/press-key/scroll/paste-text` で観測した。性能値は PowerShell `Get-Process` の `TotalProcessorTime`、`WorkingSet64`、`PrivateMemorySize64` を操作前後に採取した。
