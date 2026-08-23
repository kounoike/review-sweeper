---
id: TASK-5
title: ローカル状態・永続化・キャッシュ方式を調査する
status: To Do
assignee: []
created_date: '2026-08-20 18:06'
updated_date: '2026-08-23 00:53'
labels:
  - project-setup
milestone: m-0
dependencies: []
references:
  - adrs/0008-ソースworktreeを観測可能なローカル状態として扱う.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
type: spike
ordinal: 5
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
共有リポジトリ、PR worktree、レビュー進捗、差分キャッシュ、AI指摘、設定をどこへ保存し、どのように更新・破棄するかを決める。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 アプリ設定、認証情報、PRキャッシュ、レビューセッション、worktree情報、固定された実行バックエンド識別子の保存先を分類する
- [ ] #2 再起動、複数PR、force-push、古いキャッシュ、破損データ、backend利用不能時の動作を検証する
- [ ] #3 ファイル形式またはデータベース方式を比較し、採用案を決める
- [ ] #4 ユーザーデータを失わない更新、削除、マイグレーション方針を定義する
- [ ] #5 Windowsと将来のWSLでパス・環境・Git状態を混在させない永続化境界を定義する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
