---
id: doc-3
title: Scope and Roadmap
type: specification
created_date: '2026-08-20 18:45'
updated_date: '2026-08-20 19:07'
---
## MVP の目的

MVP の目的は、

> **GitHub Web UI より Pull Request のレビュー作業を快適かつ高速に行えることを実証する**

ことである。

MVP では IDE や汎用 GitHub クライアントを作ることを目指さない。

---

# Initial Scope

## Review Inbox

* 自分にレビュー依頼された Pull Request の一覧
* Repository / author / title / 更新時刻などの基本情報
* Review status
* Preparation status
* GitHub Checks の概要
* Pull Request の更新検知

想定する分類:

* Needs Review
* Preparing
* Ready
* Updated Since Review
* Reviewed

---

## Pull Request Overview

表示する情報:

* Title
* Description
* Author
* Repository
* Base branch
* Head branch
* Commits
* Changed files
* Additions / deletions
* Reviewers
* Current review status
* Checks

---

## Diff Viewer

必須機能:

* Unified diff
* Syntax highlighting
* Added / deleted line coloring
* File navigation
* Hunk navigation
* Line numbers
* Inline review comments
* Mark file as reviewed
* Whitespace handling

Split diff は 初期スコープ内または今後の優先機能として扱う。

---

## Full Source Viewer

* Pull Request HEAD のファイル全体表示
* Syntax highlighting
* Line number
* Find
* Text selection / copy
* Diff 位置との相互移動
* External editor で同じファイル・行を開く

ソース編集機能は実装しない。

---

## Review Draft

GitHub の pending review model に対応する。

* Inline comment draft
* Review body
* Comment
* Approve
* Request Changes
* Submit 前の確認
* Draft autosave

AI が生成したコメントは直接投稿せず、Review Draft に追加する。

---

## Review Progress

ファイル単位のレビュー状態を管理する。

例:

```text
✓ Reviewed
○ Not reviewed
● Changed since reviewed
```

レビュー済みファイルがその後変更された場合、再レビュー対象であることを表示する。

---

## Re-review

以下の Diff を切り替えられるようにする。

### PR Diff

```text
Base → PR HEAD
```

### Since Last Review

```text
Last Reviewed Revision → PR HEAD
```

前回レビュー以降に追加された変更を容易に確認できることを重視する。

---

## Worktree Management

Pull Request ごとに独立した Worktree を管理する。

機能:

* Repository fetch
* Worktree creation
* Worktree update
* PR HEAD tracking
* Workspace path 表示
* External editor で開く
* File manager で開く
* Path copy

Worktree lifecycle と cleanup policy は別途 ADR で決定する。

---

## Workspace Setup

Worktree 作成後に repository の開発環境を準備できる。

候補:

* mise
* pnpm
* npm
* yarn
* Cargo
* uv
* Custom command

自動検出と実行は分離する。

例:

```text
Detected:
  mise.toml
  pnpm-lock.yaml

Suggested:
  mise install
  pnpm install --frozen-lockfile
```

Repository trust policy に従って実行する。

---

## Local Workspace Status

外部エディタによる変更を検知する。

表示対象:

* Modified files
* Staged files
* Untracked files
* Conflicts
* Current HEAD
* PR HEAD
* HEAD divergence

Local Changes は PR Diff と明確に分離する。

---

## External Editor Integration

初期スコープでは最低限以下をサポートする。

* VS Code
* Custom command

可能なら以下も早期対応する。

* VS Code Insiders
* Cursor
* Zed

操作:

```text
Open Worktree
Open File
Open File at Line
```

---

## GitHub Checks

* Check status 一覧
* Success
* Failure
* Pending
* Skipped
* Cancelled
* Required / optional の識別が可能なら表示
* GitHub 上で詳細を開く

Check log の高度な解析は後続機能とする。

---

## Preparation Pipeline

レビュー依頼を受けた Pull Request に対して、レビュー開始前の準備を実行できる。

### Minimal

```text
Metadata
Diff
```

### Workspace

```text
Metadata
Diff
Worktree
Workspace setup
```

### AI

```text
Metadata
Diff
AI pre-review
```

### Full

```text
Metadata
Diff
Worktree
Workspace setup
AI review
```

ユーザーは global / repository 単位で policy を設定可能にすることを想定する。

---

## Preparation Scheduler

同時に多数の Pull Request が準備されることを考慮する。

初期スコープでは最低限、

* Preparation concurrency limit
* AI concurrency limit
* User-opened PR の優先

を扱う。

より高度な resource-aware scheduling は後続機能とする。

---

## AI Review

初期スコープの AI は原則として read-only とする。

利用可能な情報:

* Pull Request metadata
* Diff
* Changed files
* Full source
* Repository search
* Git history
* Review comments
* Checks information

AI output は構造化された Finding として扱う。

Finding の例:

```text
Severity
Category
File
Line
Title
Explanation
Suggested comment
```

操作:

* Dismiss
* Ask follow-up
* Convert to review comment

---

# Post-MVP

## Deeper AI Investigation

* Symbol-level investigation
* Related tests detection
* Git blame
* Historical regression investigation
* Previous review finding reconciliation
* Check failure analysis

---

## Check Log Integration

* GitHub Actions log表示
* Failed step 抽出
* AI summary
* Local reproduction command suggestions

---

## Advanced Search

* Search changed files
* Search entire repository
* Search review comments
* Search AI findings

---

## Navigation History

IDE のような Back / Forward navigation。

例:

```text
diff src/foo.rs:120
→ full source src/bar.rs:85
→ AI finding src/baz.rs:42
```

を戻れるようにする。

---

## Advanced External Tool Integration

Launcher profile を拡張する。

例:

* Open in Codex
* Open in Claude Code
* Open terminal
* Open custom review tooling

---

## Embedded Terminal

MVP では実装優先度を下げる。

必要性が確認された場合のみ追加する。

外部エディタの integrated terminal で十分な場合は実装しない可能性もある。

---

## Optional LSP Integration

MVP では対象外。

Source Viewer 内で Go to Definition や Find References が強く求められることが確認された場合に検討する。

基本方針として、高度なコード理解は外部 IDE に委譲する。

---

# Explicit Non-goals

少なくとも初期段階では以下を目的としない。

## Code Editor

Review Sweeper 内でソースコードを編集する機能は作らない。

---

## Full IDE

以下を独自実装することを目的としない。

* Debugger
* Refactoring
* Build system integration
* Full LSP client
* Language-specific IDE functionality

---

## Autonomous Reviewer

AI が自律的に Pull Request を Approve / Reject する機能を目的としない。

---

## Autonomous Code Modification

AI が Review Sweeper 内からコードを書き換え、commit / push することは初期スコープに含めない。

必要な場合は外部 coding agent に処理を渡す。

---

## General-purpose GitHub Client

以下のような GitHub 全機能の代替を目的としない。

* Issue management
* Project management
* Repository administration
* Releases
* Actions workflow authoring
* Organization management

Review Sweeper は Pull Request review workflow に集中する。

---

# MVP 成功条件

MVP 完成の判断は機能数ではなく、実際のレビュー体験で行う。

最低限、以下の流れが快適に完了できること。

```text
Review Inbox
   ↓
Open ready PR
   ↓
Read diff
   ↓
Inspect full source
   ↓
Check AI findings
   ↓
Check CI
   ↓
Open suspicious code in external editor
   ↓
Return to Review Sweeper
   ↓
Confirm local workspace state
   ↓
Write review comments
   ↓
Submit review
```

さらに再レビュー時に、

```text
Updated Since Review
   ↓
View only new changes
   ↓
Confirm previous comments
   ↓
Submit follow-up review
```

が GitHub Web UI より明確かつ高速に行えることを重要な成功条件とする。

---

## Windows / WSL 実行バックエンド

初期ターゲットはWindowsとする。GUIはWindowsネイティブアプリケーションとして動作させる。

Windowsネイティブ実行を基本経路とし、WSLは必須依存ではなく、MVP後のできるだけ早い段階で追加する任意の実行バックエンドとして扱う。

WSL対応では、Worktree単位で実行バックエンドを選択・表示できるようにし、git、Worktree操作、workspace setupなどのコマンド実行環境を一貫させる。Windows側とWSL側のパス、環境変数、Git状態を暗黙に混在させない。

WSLが利用できない場合でも、Review Inbox、Diff確認、レビューコメント作成・送信などの基本レビュー機能は利用できる。

将来的にはmacOS/Linuxを対象に追加し、各OSのネイティブ実行環境を同じ実行バックエンド抽象化の下で扱う。
