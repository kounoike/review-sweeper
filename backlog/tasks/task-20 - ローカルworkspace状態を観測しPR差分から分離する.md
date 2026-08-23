---
id: TASK-20
title: ローカルworkspace状態を観測しPR差分から分離する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - worktree
  - mvp
milestone: m-1
dependencies:
  - TASK-4
  - TASK-19
  - TASK-15
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
  - adrs/0008-ソースworktreeを観測可能なローカル状態として扱う.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 20
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
外部ツールで変更され得るPR worktreeを観測し、PRの正規差分、前回レビュー以降の差分、ローカルworkspace変更を混同せず表示する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 clean、modified、staged、untracked、conflicted、missing/broken、HEAD-divergedを追跡・表示する
- [ ] #2 git status更新をデバウンスし、複数worktreeを過剰に走査しない
- [ ] #3 ローカルHEADとPR HEADの相違を表示し、ローカル実験をPR HEADと比較できる
- [ ] #4 ローカル差分は出所を保持し、明示的選択なしにPRレビューまたはAIレビューへ含めない
- [ ] #5 外部編集、コミット、force-push、worktree削除中の状態遷移をfixtureで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
