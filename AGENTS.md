# Agent instructions

## 基本方針

- ユーザーの依頼と、それに対応する単一のBacklog.mdタスクにスコープを限定する。複数タスクを同時に進めない。
- ユーザーとのコミュニケーション、およびこのリポジトリの文書は日本語で記述する。製品名、コマンド名、コード、識別子など、翻訳すると意味が変わる技術要素は原文のまま記載する。
- 既存の変更を勝手に破棄しない。作業開始時に`git status --short`を確認し、依頼と無関係な変更は保持する。
- 依頼の範囲を超える製品仕様、公開API、互換性、リリース方式、アーキテクチャの決定を、ユーザーの承認なしに確定しない。

## Backlog.mdの運用

このプロジェクトではBacklog.mdをタスク、受け入れ条件、作業メモ、完了サマリーの正本として使用する。

### 必須ワークフロー

- すべてのユーザー依頼について、作業や回答の前に`mise exec -- backlog instructions overview`を実行する。
- Backlogのタスクを作成・分割する前に`mise exec -- backlog instructions task-creation`を読む。
- タスクの計画、担当者や状態の変更、計画・実装メモの追加、実装開始の前に`mise exec -- backlog instructions task-execution`を読む。
- 受け入れ条件の確認、完了サマリーの記録、終端状態への変更の前に`mise exec -- backlog instructions task-finalization`を読む。
- 不慣れなBacklog CLIコマンドは、先に`mise exec -- backlog <command> --help`で引数と使用例を確認する。
- タスク、ドラフト、文書、Decision、マイルストーンのMarkdownを直接編集せず、状態、受け入れ条件、メモ、完了サマリーはBacklog CLIで更新する。
- 受け入れ条件を実装前に具体化し、作業中はチェックリストと状態を更新し、完了時には検証結果を含む完了サマリーを記録する。
- タスクを完了にするのは、受け入れ条件をすべて満たし、関連する検証が成功した後に限る。

Backlog CLIは`mise.toml`で管理しているため、直接`backlog`を実行せず、常に`mise exec -- backlog ...`を使う。初回またはツールが未導入の場合は`mise install`を実行してから利用する。こちらで定義済みの確認処理は`mise run backlog-check`を使ってよい。`mise`経由でもCLIが利用できない場合は、タスクMarkdownを直接編集して代替せず、制約をユーザーに報告する。

### タスクの境界

- 対応するタスクが存在しない場合は、タスク作成が必要かを判断し、必要なら作成手順を読んでから作成する。
- 既存タスクの受け入れ条件やスコープが依頼と一致しない場合、勝手に拡張せず、タスクを分割するかユーザーに確認する。
- 重要な調査結果、選択肢、トレードオフ、実装上の前提は、チャットだけでなく対象タスクのメモに残す。

## アーキテクチャとADR

- 大きな影響があり、または変更を戻しにくい設計判断は`adrs`で管理する。
- 作業前に`mise run adr-list`と`mise run adr-doctor`を実行し、関連する既存ADRと状態を確認する。
- Decisionを作成する前に既存のDecision/ADRを検索する。既存の受け入れ済み決定を新しい選択に合わせて書き換えない。置き換える場合は新しいDecisionを作成し、旧Decisionを`Supersedes`で明示する。
- ADRの作成、編集、状態変更、関連付け、索引生成には`adrs new`、`adrs edit`、`adrs status`、`adrs link`などのCLIを使い、索引を手作業で編集しない。
- ユーザーは重要な製品仕様、スコープ、公開API、互換性、アーキテクチャの最終決定権を持つ。エージェントは選択肢・根拠・トレードオフを整理して提示し、承認前に決定を確定したり依存実装を開始したりしない。

Decision本文をCLIで生成・更新できない場合に限り、CLIのバージョンと`mise exec -- backlog decision --help`および`mise exec -- backlog decision create --help`で制約を確認したうえで、Decisionの本文セクションだけを直接編集してよい。Frontmatterは変更せず、タスク等の他のMarkdownにはこの例外を適用しない。

## 実装と検証

- Rustの変更では、作業内容に応じて`mise run fmt`、`mise run lint`、`mise run test`、`mise run check`、`mise run build`を実行する。完了時に実行した検証と結果をタスクへ記録する。
- ドキュメントや設定のみの変更でも、`git diff --check`を実行する。
- 失敗した検証を成功したものとして報告しない。環境依存で実行できない場合は、コマンドと理由を明記する。
