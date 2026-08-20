---
id: TASK-10
title: GitHub Actions CIとAIエージェント向け品質ゲートを構築する
status: To Do
assignee: []
created_date: '2026-08-20 18:13'
updated_date: '2026-08-20 18:16'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-8
  - TASK-9
type: chore
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
フォーマット、lint、テスト、ビルドをGitHub Actionsで自動検証し、プルリクエストとmainブランチの品質ゲートを整備する。あわせて、AIエージェントが作業完了前に同じ品質チェックを実行する仕組みを構築する。`mise`タスク、GitHub Actionsの必須チェック、`AGENTS.md`や必要に応じたSkillの役割を整理し、指示漏れやローカル環境差異だけに依存しない運用にする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 対応環境ごとのCIマトリクスと実行トリガーを決める。
- [ ] #2 `mise`タスクを利用してfmt、lint、test、buildを実行するWorkflowを追加する。
- [ ] #3 キャッシュ、失敗時のログ、必要な権限、実行時間の制約を定義する。
- [ ] #4 プルリクエストをマージするための必須チェックを決める。
- [ ] #5 AIエージェントが作業前後に実行すべきfmt、lint、test、checkの手順をAGENTS.mdなどに明記する。
- [ ] #6 AIエージェント向けのSkillや自動チェックが必要かを検討し、採用する仕組みをリポジトリに追加する。
- [ ] #7 GitHub Actionsの必須チェックで、エージェントや人間がローカルチェックを省略しても品質ゲートを通過できないようにする。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
