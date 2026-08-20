---
id: doc-2
title: Product & Design Principles
type: specification
created_date: '2026-08-20 18:43'
updated_date: '2026-08-20 19:07'
---
## 1. Review First

Review Sweeper は汎用 GitHub クライアントではない。

Pull Request のレビューに直接関係しない機能は、必要性を慎重に判断する。

機能追加時には、

> この機能はレビュー開始からレビュー完了までの時間、判断品質、認知負荷のいずれかを改善するか

を判断基準とする。

---

## 2. Review Workbench, Not IDE

Review Sweeper は IDE を目指さない。

アプリ内で提供するコード表示機能はレビューに必要な範囲に限定する。

含めるもの:

* Syntax highlighted source viewer
* Diff viewer
* Line numbers
* Selection / Copy
* Find
* Comment gutter
* AI finding overlay

原則として含めないもの:

* Source editing
* Refactoring
* Debugger
* Language-specific IDE features
* 本格的な LSP integration

高度なコード操作は外部エディタに委譲する。

---

## 3. Keyboard First, Mouse Friendly

レビューを大量に処理する用途では、繰り返し操作のコストが重要である。

主要操作はキーボードのみでも完結できるようにする。

同時に、キーボード操作を知らなくても通常の GUI として利用可能にする。

主要操作は可能な限り Command としてモデル化し、以下から同じ操作を利用できるようにする。

* Keyboard shortcut
* Command palette
* Menu
* Context menu
* Button

---

## 4. Prepare Before the User Needs It

ユーザー操作を待ってから重い処理を開始しない。

レビュー依頼を受けた Pull Request は、設定に応じて事前に準備する。

対象には以下を含む。

* Metadata
* Diff
* Git fetch
* Worktree
* Workspace setup
* AI review
* Checks

ただし、自動化はユーザーの意思決定を代行しない。

---

## 5. Automation Stops Before Decision

バックグラウンド処理や AI が自動で行ってよいのは、

> ユーザーが判断できる状態を作るところまで

とする。

自動で行ってよい処理の例:

* Metadata fetch
* Diff fetch
* Worktree creation
* Workspace setup
* AI analysis
* Check status retrieval

ユーザー操作を必要とする処理:

* Review comment 投稿
* Approve
* Request Changes
* Review submission
* Commit
* Push

---

## 6. AI Assists, Human Decides

AI finding はレビュー結果ではない。

AI が発見した可能性のある問題は、ユーザーが確認するための情報として扱う。

Finding は例えば次の状態を持つ。

* Open
* Dismissed
* Converted to Comment
* Resolved

AI が生成したコメント案も、そのまま GitHub へ投稿しない。

レビュー担当者が確認・編集した後に review draft へ追加する。

---

## 7. Remote State and Local State Are Different

GitHub 上の Pull Request とローカル Worktree は別の状態として扱う。

### Remote canonical state

* Pull Request metadata
* Pull Request HEAD
* Submitted reviews
* Review comments
* GitHub Checks

### Local state

* Worktree
* Local modifications
* AI findings
* Draft comments
* Review progress
* Last reviewed revision
* UI state

両者を暗黙的に混在させない。

---

## 8. Local Changes Must Be Visible

外部エディタで Worktree が変更されることを正常なユースケースとして扱う。

そのため、Review Sweeper は Worktree の状態を UI 上で容易に確認できるようにする。

少なくとも以下を識別する。

* Clean
* Modified
* Staged
* Untracked
* Conflict
* HEAD diverged from PR

ローカル変更が存在する場合でも、通常の PR review target に暗黙的に含めない。

---

## 9. Re-review Is a First-class Workflow

レビューは一度で完了するとは限らない。

作者が修正 commit を追加した後の再レビューを、通常レビューとは別の重要なワークフローとして設計する。

Review Sweeper は、

```text
Base → PR HEAD
```

だけでなく、

```text
Last Reviewed Revision → Current PR HEAD
```

を容易に確認できるようにする。

前回指摘事項と新しい変更の関係も追跡可能にする。

---

## 10. Fast Path First

Pull Request を開くために Worktree や dependency installation の完了を要求しない。

例えば、

```text
Open PR
 ├─ immediately: metadata / cached diff
 └─ asynchronously: worktree / setup / deep AI review
```

とする。

ユーザーが最初の Diff を読み始めるまでの時間を最小化する。

---

## 11. Explicit Trust Boundaries

Worktree setup は任意コード実行につながる可能性がある。

例:

```text
mise install
pnpm install
npm install
cargo build
custom setup scripts
```

そのため、

* Repository trust
* Setup command policy
* AI command execution policy

を明確に設計する。

未知の Pull Request を表示しただけで、無条件に repository-controlled code を実行しない。

---

## 12. Graceful Degradation

すべての機能が利用可能でなくてもレビューは続行できるようにする。

例:

* Worktree作成失敗 → GitHub diff は閲覧可能
* Setup失敗 → Source viewer は利用可能
* AI API障害 → 手動レビューは可能
* Checks取得失敗 → Diff review は可能
* External editor未設定 → アプリ内閲覧は可能

一部機能の障害によって Review Session 全体をブロックしない。

---

## 13. Observable Background Work

Preparation はバックグラウンドで行うが、隠さない。

Review Inbox から例えば、

```text
Diff        ✓ Ready
Workspace   ◌ Creating
Setup       ◌ pnpm install
AI          ✓ 3 findings
Checks      ✓ Passing
```

のように状態を確認できるようにする。

ユーザーが必要なら、

* Cancel
* Retry
* Run now

などを選択できるようにする。

---

## 14. Prioritize Interactive Work

バックグラウンド Preparation より、ユーザーが現在操作している Pull Request を優先する。

例えば優先順位は、

1. Currently opened PR
2. Explicit user action
3. Requested review preparation
4. Background refresh

とする。

バックグラウンド処理によって UI responsiveness を犠牲にしない。

---

## 15. Persistent Review Sessions

レビュー途中の状態は可能な限り失わない。

少なくとも以下をローカルに保存する。

* Draft comments
* AI findings
* Review progress
* Last reviewed revision
* Workspace information

アプリ再起動後に Review Session を再開できることを目指す。

---

## 16. Semantic Themes

テーマは以下をサポートする。

* Light
* Dark
* System

コンポーネントは直接 RGB 値を持たず、semantic color を参照する。

例:

* Background
* Foreground
* Muted
* Border
* Accent
* Success
* Warning
* Error
* Diff Added
* Diff Deleted
* Selection
* AI Finding

Syntax highlighting theme と UI theme は内部的には分離可能な構造とする。

---

## 17. External Tools Are an Extension Point

外部エディタ連携を VS Code 固有機能として設計しない。

Launcher profile として抽象化し、将来的に以下のようなツールへ拡張可能にする。

* VS Code
* Cursor
* Zed
* JetBrains IDE
* Terminal
* Codex
* Other development agents
* Custom command

Review Sweeper はレビューの中心となり、必要に応じて適切な専門ツールへ処理を渡す。

---

## 18. Optimize the Whole Review Cycle

個々の機能のベンチマークより、レビュー全体の所要時間を重視する。

例えば Worktree 作成を10%高速化することより、

* 次の Pull Request を事前準備する
* 前回レビュー以降だけを表示する
* AI findings を事前生成する

方がユーザー体験を大きく改善する可能性がある。

常に Review Cycle 全体を最適化する。
---

## 実行環境の分離

Review Sweeperの初期ターゲットはWindowsとし、GUIはWindowsネイティブで提供する。

コマンド実行やworkspace setupは実行バックエンドを介して行い、Windowsネイティブ実行を基本とする。WSLは必須依存にせず、MVP後のできるだけ早い段階で選択可能なバックエンドとして追加する。

Worktreeごとに実行バックエンドを明示し、WindowsとWSLのパス、環境変数、Git状態を暗黙に混在させない。WSL未導入でも基本的なレビュー機能は利用可能にする。
