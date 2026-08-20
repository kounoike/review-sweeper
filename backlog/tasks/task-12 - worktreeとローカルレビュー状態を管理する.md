---
id: task-12
title: worktreeとローカルレビュー状態を管理する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-20 17:37'
labels:
  - worktree
milestone: m-1
dependencies:
  - TASK-1
  - TASK-7
type: feature
---

## Description
<!-- SECTION:DESCRIPTION:BEGIN -->
共有リポジトリ、PRごとのworktree、ユーザー編集、PRの改訂を、ローカル作業を失わずに観測可能にする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] PRごとに独立してcloneするのではなく、共有ローカルリポジトリとPRごとのworktreeを使う。
- [ ] PRの終了、ブランチ更新、force-push、古いworktree、ガベージコレクションのライフサイクル規則を定義する。
- [ ] 明示的な確認なしに、ユーザー変更のあるworktreeをリセットまたは削除しない。
- [ ] PRの変更、前回レビュー以降の変更、ローカルworkspaceの変更を区別する。
- [ ] staged、unstaged、untracked、conflicted、clean、missing/broken、HEAD-divergedのworkspace状態を追跡する。
- [ ] worktree状態を監視し、git statusの更新をデバウンスする。
- [ ] ローカルHEADがPR HEADと異なる場合に表示する。
- [ ] ローカルの実験をPR HEADと比較できるようにする。
- [ ] 明示的に選択されない限り、ローカルworkspaceの変更をPRレビュー対象から除外する。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
