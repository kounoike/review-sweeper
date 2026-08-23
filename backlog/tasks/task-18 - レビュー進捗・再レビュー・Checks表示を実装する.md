---
id: TASK-18
title: レビュー進捗・再レビュー・Checks表示を実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - mvp
milestone: m-1
dependencies:
  - TASK-14
  - TASK-15
  - TASK-17
references:
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 18
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ファイル単位のレビュー進捗、前回レビュー以降の変更、GitHub Checksをまとめ、再レビュー対象を短時間で判断できるようにする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ファイルを未確認、確認済み、確認後に変更ありとして追跡・表示する
- [ ] #2 PR Diffと前回レビュー時点からPR HEADまでの差分を切り替えられる
- [ ] #3 force-push、rename、削除、前回revision不明時の進捗動作を定義して検証する
- [ ] #4 選択中PRのChecksを成功、失敗、進行中、skipped、cancelledとして表示し、GitHub詳細を開ける
- [ ] #5 レビュー送信後とPR更新後に進捗・レビュー状態・Checksが矛盾なく更新される
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
