---
id: TASK-4
title: 差分とGitHubレビューAPIを技術調査する
status: Done
assignee:
  - '@kounoike'
created_date: '2026-08-20 18:06'
updated_date: '2026-08-29 03:50'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-2
references:
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
  - adrs/0007-構造化された差分モデルを使う.md
documentation:
  - doc-7
modified_files:
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
  - adrs/0007-構造化された差分モデルを使う.md
  - backlog/docs/technical/github-integration/doc-7 - 差分・GitHubレビューAPI技術調査.md
  - backlog/tasks/task-4 - 差分とGitHubレビューAPIを技術調査する.md
  - spikes/diff-review-api/fixtures/github-pr-files.json
  - spikes/diff-review-api/fixtures/review-payloads.json
  - spikes/diff-review-api/verify.sh
type: spike
ordinal: 4
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GitHub差分とローカル差分を扱い、インラインコメントを正しい位置へ対応付けるための差分形式、行番号、レビューAPIの制約を検証する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 再現可能なfixtureと小さな実証により、GitHubのPR diff/API patchとlocal git diffについて、取得経路、commit基準、rename/add/delete/binary/改行・空白、欠落・truncationを比較し、GitHubをreview位置の正本、local gitを補助比較として扱う際の制約が記録されている
- [x] #2 file、hunk、context/addition/deletion行、old/new line、side、commit identity、GitHub review comment位置を表す構造化モデルと、不変条件・位置解決不能時の扱いが定義されている
- [x] #3 同一の構造化行からunified/side-by-side表示を導出し、whitespace除外を別比較として扱い、前回review時commitから最新headまでの差分とoutdated commentを混同しない変換・照合手順がfixtureで検証されている
- [x] #4 GitHub公式一次情報に基づき、draft review作成、pending comment追加、event送信・破棄までのAPI、line/sideとsubject_type=file、commit_id、validation/permission/conflict/rate-limit/network/partial failureのerror処理・再試行契約が定義されている
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. GitHub公式REST/GraphQLドキュメントとgit公式仕様を調査し、差分取得・review comment位置・pending review lifecycleの制約を出典付きで整理する。
2. 一時Git repositoryからrename/add/delete/binary/空白/改行を含むfixtureを生成し、git diffのhunkとold/new lineを機械的に検証する。GitHub側は公式API例を固定fixture化し、取得制約とlocal差分との差異を確認する。
3. file/hunk/line/position/commit identityの候補モデル、unified/side-by-side、whitespace除外、前回review差分/outdated mappingを文書化し、重要なarchitecture判断は選択肢・推奨・tradeoffをOrca askで承認確認する。
4. draft review作成からpending comment追加、送信・破棄までのrequest/response/error matrixと安全なretry/idempotency境界を文書化する。
5. fixture実証、git diff --check、mise run backlog-check、mise run adr-doctor等を実行し、AC/DoD、notes、final summaryをBacklog CLIで更新して関連変更だけをcommitする。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-29 調査結果: GitHub公式仕様でPRがthree-dot diff、review commentのpositionが廃止方向でline/sideとsubject_type=fileを使うこと、PENDING reviewはevent省略で作成しevents endpointでsubmit、PENDINGのみdelete可能であることを確認した。local fixtureではrename、add/delete、binary、no-final-newline、old/new line計数、-w、two-dot/three-dotの差を再現し、verify.shが成功した。

Architecture判断: GitHub review diff snapshotとlocal comparisonを別sourceとして保持し、共通File/Hunk/Lineへparseしてsource/commit/座標を保持するADR-0006/0007案を推奨する。単一normalized diffは誤座標リスク、GitHub-onlyはlocal/whitespace/前回review比較が弱い。Orca Dispatchがagent_prompt_stalledで失効し、2回のorchestration askがactive Dispatchなしとして拒否されたため、ADRはProposedのまま承認待ちとした。

検証済み: bash -n spikes/diff-review-api/verify.sh、spikes/diff-review-api/verify.sh、jq empty fixtures/*.json、git diff --check、mise run backlog-check、mise run adr-doctorはすべて成功。live GitHub integration testとしてlarge/3000 files境界、.gitattributes、pending comment association、multi-line range、timeout reconcileを後続実装で確認する。

2026-08-29 承認反映: ユーザーが推奨案を承認したためADR-0006/0007をAcceptedへ変更した。GitHub review snapshotとlocal comparisonは別sourceのまま共通構造化モデルへparseし、source、revision・commit identity、GitHub座標を保持する。local差分やwhitespace除外表示からGitHub座標を推測・逆変換して送信しない。local修正→Suggested Changesはmain上のTASK-29へ分離されており、本タスクはmodel boundaryと調査結果までとした。

最終検証: bash -n spikes/diff-review-api/verify.sh、spikes/diff-review-api/verify.sh、jq empty spikes/diff-review-api/fixtures/*.json、git diff --check、mise run backlog-check、mise run adr-doctorがすべて成功した。live GitHub integration境界は後続実装で検証する。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
GitHub PR差分とlocal git差分の取得・座標・欠落制約をfixtureで検証し、両者を別sourceとして共通構造化モデルへparseする契約を確定した。ADR-0006/0007をAcceptedとし、GitHub comment座標はGitHub review snapshot由来に限定、local/whitespace差分からは推測しない。pending review lifecycleとerror/retry契約、TASK-29との境界を文書化し、fixture検証、JSON検証、git diff --check、backlog-check、adr-doctorの成功を確認した。
<!-- SECTION:FINAL_SUMMARY:END -->
