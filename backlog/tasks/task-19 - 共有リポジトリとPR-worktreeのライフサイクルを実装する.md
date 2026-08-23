---
id: TASK-19
title: 共有リポジトリとPR worktreeのライフサイクルを実装する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-23 00:50'
labels:
  - worktree
  - mvp
milestone: m-1
dependencies:
  - TASK-9
  - TASK-5
  - TASK-13
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0008-ソースworktreeを観測可能なローカル状態として扱う.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 19
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
共有ローカルリポジトリとPRごとのworktreeをWindowsネイティブ実行バックエンドで安全に作成・更新・追跡する。ローカル編集状態の継続監視と差分表示は別タスクとする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PRごとのcloneではなく共有リポジトリと独立したworktreeを作成・再利用できる
- [ ] #2 WorktreeにPR識別子、PR HEAD、ローカルHEAD、パス、固定された実行バックエンドを関連付ける
- [ ] #3 PR終了、ブランチ更新、force-push、missing/broken、古いworktreeのライフサイクル規則を実装する
- [ ] #4 ユーザー変更またはHEAD divergenceがあるworktreeを明示的確認なしにreset・削除しない
- [ ] #5 複数PRの並行操作と失敗からの再試行をfixtureで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
