---
id: TASK-8
title: テスト戦略とfixtureを設計する
status: Done
assignee:
  - '@codex'
created_date: '2026-08-20 18:06'
updated_date: '2026-08-30 20:30'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-1
  - TASK-2
  - TASK-4
  - TASK-5
  - TASK-7
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - doc-10
modified_files:
  - backlog/docs/technical/test-strategy/doc-10 - テスト戦略とfixture設計.md
  - spikes/test-strategy/.gitignore
  - spikes/test-strategy/Cargo.lock
  - spikes/test-strategy/Cargo.toml
  - spikes/test-strategy/src/lib.rs
  - spikes/test-strategy/fixtures/manifest.json
  - spikes/test-strategy/verify.sh
  - spikes/test-strategy/README.md
type: spike
ordinal: 8
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GitHub連携、差分解析、worktree状態、準備パイプライン、UI状態を再現可能に検証するためのテスト層とfixtureを設計する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 単体・domain・API contract・fixture contract・integration・Windows native UI/E2Eの責務、代替可能範囲、実行頻度、失敗時の切り分けをテスト戦略文書に定義する
- [x] #2 GitHub PR差分・review thread/comment lifecycle・Checks、clean/dirty/untracked/conflict/corrupt等のworktree状態、ExecutionBackendのWindows native/WSL/unsupported/errorを再現するversioned fixture schemaと代表fixtureを定義し、自動検証できる
- [x] #3 通常CIはnetwork・GitHub account/token・WSL・Windows desktopに依存せずLinuxで決定的に完結し、Windows native UI/ConPTY/IME/UIAを別laneで検証する実行条件と品質ゲートを定義する
- [x] #4 非同期順序逆転・cancel競合・retry/backoff・partial/corrupt data・fixture schema drift・backend identity混在を検出するmatrixとoracleを定義し、代表ケースを再現可能なprototypeで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. 既存ADR・TASK-1/2/4/5/7の成果物とfixture/test慣例を調査し、既存契約を抽出する。
2. テスト層、fixture schema、非同期・失敗・backend混在matrix、CI laneを一つの技術文書として設計する。
3. network/WSL不要で代表fixtureと境界条件を検証する小さな再現可能prototypeを追加する。
4. prototypeと文書・Backlog整合性を検証し、AC/DoD・notes・final summaryを更新してcommitする。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-31 調査・設計: ADR-0002/0006/0007/0010/0011/0012およびTASK-1/2/4/5/7の成果物をtest oracleとして整理した。doc-10にunit/domain/API contract/fixture/hermetic integration/Windows native integration/UI-E2Eの境界、GitHub差分・review・Checks、worktree、ExecutionBackend、async/cancel/retry/corruption/backend混在のmatrix、hermetic/windows/github-live/wslのCI laneを記録した。live GitHub semantics、Windows IME/UIA/ConPTY、runner調達、performance budgetはmockで代替せず後続タスクの判断として残した。

2026-08-31 検証: test-strategy crateのcargo fmt --check、clippy -D warnings、test（6件）、check、build、fixture verifyが成功した。関連regressionとしてTASK-4 diff/review fixture、TASK-7 ExecutionBackend test（6件+doctest）、TASK-5 persistence/cache test（13件）が成功し、git diff --check、mise run backlog-check、mise run adr-doctorも成功した。rootにはCargo.tomlがないためrootのmise run fmt/lint/test/check/buildではなく各manifestを明示した。Windows native UI/IME/UIA/ConPTYおよびGitHub live APIは設計上別laneであり、このLinux hermetic prototypeでは未実行・成功扱いにしていない。

2026-08-31 ユーザーがPR #8を承認。全AC・DoDと記録済み検証結果を確認し、merge前にDoneへ更新した。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
unit/domain/API contract/fixture/hermetic integration/Windows native UIの責務とCI laneをdoc-10へ定義し、GitHub差分・review・Checks、worktree、ExecutionBackend、async/cancel/retry/corruption/backend混在をversioned fixtureとmatrixへ整理した。network・GitHub credential・WSL・Windows desktop不要のprototypeを追加し、6 unit testsと一時git repositoryによる7状態のfixture検証、関連TASK-4/5/7 regression、Backlog/ADR検査の成功を確認した。live GitHub semantics、Windows IME/UIA/ConPTY、runnerおよびperformance budgetの具体化はmockで代替せず後続実装の範囲として残す。
<!-- SECTION:FINAL_SUMMARY:END -->
