---
id: TASK-12
title: Windows向け初回リリース方法を選定する
status: To Do
assignee: []
created_date: '2026-08-20 18:13'
updated_date: '2026-08-23 00:53'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-3
  - TASK-11
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
type: spike
ordinal: 12
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Windows向けRustネイティブアプリの初回成果物をユーザーへ届ける方法と、バージョニング、リリースノート、署名、公開範囲、更新方法を選定する。macOS/Linuxの配布方式は将来タスクの判断を拘束しない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Windows向けGitHub Releases、インストーラー、パッケージマネージャー候補を比較する
- [ ] #2 バージョン番号、タグ、リリースノート、Windows成果物命名の規約を決める
- [ ] #3 Windows成果物の署名、チェックサム、秘密情報の管理方法を決める
- [ ] #4 初回リリースまでの手動手順と将来自動化する範囲を記録する
- [ ] #5 将来のmacOS/Linux追加時に再検討する配布要件を明記する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
