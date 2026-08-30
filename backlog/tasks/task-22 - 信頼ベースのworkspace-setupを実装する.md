---
id: TASK-22
title: 信頼ベースのworkspace setupを実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
updated_date: '2026-08-30 20:35'
labels:
  - preparation
  - mvp
milestone: m-1
dependencies:
  - TASK-21
  - TASK-13
references:
  - adrs/0005-リポジトリセットアップ前に信頼を要求する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 22
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
リポジトリのセットアップ要件を副作用なく自動検出して提案内容を表示する。setupはrepositoryが信頼済みでも自動実行せず、ユーザーが明示的に「Setupを実行」を選び、初回またはsetup policy fingerprint変更時の承認がある場合だけ、Worktreeに固定された実行バックエンドで実行する。setup未実行・承認待ち・失敗でもGitHub差分によるレビューを継続可能にする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 mise.toml、.tool-versions、主要manifest・lockfileからセットアップ候補と必要性を副作用なしで自動検出する
- [ ] #2 検出した正規化コマンド、実行理由、対象worktree、固定実行バックエンド、repository trust、setup policy承認状態を実行前に表示する
- [ ] #3 信頼されていないリポジトリではコードやsetupコマンドを一切実行せず、repositoryが信頼済みでもユーザーの明示的な「Setupを実行」操作なしにsetupを自動実行しない
- [ ] #4 初回はrepository trustとsetup policyを承認し、fingerprintが同じ場合は手動実行操作後に追加確認なしで固定バックエンドからsetupを開始し、ログ、失敗、キャンセル、再試行を扱う
- [ ] #5 正規化コマンド、対象backend、制御file等のpolicy fingerprint変更時はtrustを維持しつつ差分を示して再承認を要求し、信頼の付与・取り消し、PR更新、別worktree・別backend、setup未実行・失敗でもレビューを継続できる境界をテストする
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
