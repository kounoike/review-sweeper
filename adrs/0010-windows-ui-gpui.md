# 10. WindowsネイティブUIフレームワークにGPUIを採用する

Date: 2026-08-25

## Status

Accepted

Relates to [1. Rustネイティブアプリケーションを採用する](0001-Rustネイティブアプリケーションを採用する.md)

Relates to [2. WindowsネイティブGUIとWSL実行バックエンドを分離する](0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md)

## Context

Review Sweeper は Windows ネイティブ GUI 上で、大量行の PR diff、内蔵 terminal、background 更新、日本語を含む text、keyboard 中心の操作を扱う。TASK-1 では GPUI 0.2.2 と eframe/egui 0.36.1 を中心に、Windows native build/run、100,000 行の virtual list、input、background 更新、UI Automation、font fallback、installed system font の runtime 切替を比較した。詳細な証拠と再現手順は TASK-1 および doc-5 に記録する。

GPUI は DirectWrite system font collection と system fallback を native に利用できる。Windows 実測では、用途別の primary/fallback を切り替えながら、日本語、非 BMP 漢字、combining、emoji/ZWJ を font 同梱なしで最も完全に表示できた。また `uniform_list`、小さい memory footprint、framework 統合 executor は、大量 diff と非同期処理を持つ本製品に適合する。GPUI main で進む AccessKit 統合と、Zed と共に editor 向け基盤として継続開発されている点も、公開版との差を管理する前提で将来性があると判断した。

eframe/egui は公開版で selection/copy と意味的 UIA tree を実証できる強みがある。一方、egui 単体には system font discovery がなく、installed family から font file/TTC face を解決し、file read/cache、runtime の `FontDefinitions` 再登録、app-wide style 適用までアプリ側が担う。日本語と emoji sequence の完全な fallback を同梱なしで成立させる実装コストを含め、今回は第二候補とする。

## Decision

Windows ネイティブ UI framework に GPUI を採用する。eframe/egui は第二候補かつ fallback とし、下記の採用撤回条件に該当した場合に再評価する。

font はアプリへの同梱を前提にしない。次の 3 role を別設定とし、installed system font の primary family と順序付き fallback family list をユーザーが選択可能にする。

- UI proportional
- diff/editor monospace
- terminal monospace

無効、削除済み、または CJK glyph が不足する font を silent success にしない。適用前 validation、解決結果と blocking 日本語 corpus の preview、coverage 警告を提供し、失敗時は last-known-good、順序付き fallback、role 別の安全な system default の順で復旧することを必須設計条件とする。terminal の East Asian Width、combining、emoji cluster、cell 幅、cursor 位置は font 設定ではなく VT/grid engine の責務とする。

この spike では製品の公開 settings API、永続化 schema、settings UI、font picker の詳細を確定しない。UI 非依存の state/intent/effect と framework adapter の境界を維持し、GPUI 固有型を domain/application の公開 API に出さない。

次を既知制約および採用撤回条件として後続実装で検証する。

- crates.io GPUI 0.2.2 は content の UIA/AccessKit tree を公開せず、window 1 node のみだった。main の AccessKit 実装を公開版の能力として扱わず、公開版と main/AccessKit の差を pin と検証で管理する。
- custom diff/editor には grapheme-aware selection/copy、syntax span、wrap、side-by-side 同期、focus、accessibility metadata/action の実装が必要である。
- Microsoft 日本語 IME の preedit、変換、commit、candidate 位置を custom input surface で実測する必要がある。
- font family の locale 差、無効/削除設定、CJK coverage を validation と preview で扱う必要がある。
- terminal frontend は cell 幅、ambiguous width、combining/emoji cluster、selection/copy、cursor alignment、IME、accessibility を検証する必要がある。
- GPUI main の AccessKit 対応を利用する場合は crates.io 公開物との API・behavior 差と安定性を明示する。

これらを満たせず、特に accessibility、diff の基本 selection/copy、IME、terminal cell/cursor のいずれかを製品要件内で実現できない場合は、eframe/egui を fallback として再評価する。後続作業の候補はここに記録するが、この決定だけを根拠に新しい Backlog task を自動作成しない。

## Consequences

- DirectWrite system collection/fallback を利用し、font file をアプリへ同梱または全量 load せず、用途別の system font 選択を実装しやすい。
- `uniform_list`、GPUI の memory 特性、executor を大量 diff と background 更新へ活用できる。
- 日本語表示の実測品質と GPUI/AccessKit の将来性を優先できる。
- 公開 GPUI 0.2.2 の content UIA 不足を補う custom accessibility 実装と、main/公開版の差を継続管理する負担が生じる。
- diff/editor の selection/copy と input、Microsoft 日本語 IME、terminal grid/accessibility は製品実装で明示的に作り、Windows 実機で検証する必要がある。
- eframe/egui の公開 selection/copy/UIA の利点は fallback 評価の基準として残るが、system font discovery/file/TTC/cache/app-wide 適用のアプリ側コストは採用しない。
