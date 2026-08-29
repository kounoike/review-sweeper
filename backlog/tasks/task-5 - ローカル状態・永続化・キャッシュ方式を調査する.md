---
id: TASK-5
title: ローカル状態・永続化・キャッシュ方式を調査する
status: Done
assignee:
  - '@codex'
created_date: '2026-08-20 18:06'
updated_date: '2026-08-29 03:41'
labels:
  - project-setup
milestone: m-0
dependencies: []
references:
  - adrs/0008-ソースworktreeを観測可能なローカル状態として扱う.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0012-secret-cache-hybrid.md
  - adrs/0011-github-secret-windows.md
modified_files:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0011-github-secret-windows.md
  - adrs/0012-secret-cache-hybrid.md
  - spikes/persistence-cache/.gitignore
  - spikes/persistence-cache/Cargo.lock
  - spikes/persistence-cache/Cargo.toml
  - spikes/persistence-cache/README.md
  - spikes/persistence-cache/src/lib.rs
  - spikes/persistence-cache/src/storage.rs
  - backlog/tasks/task-5 - ローカル状態・永続化・キャッシュ方式を調査する.md
type: spike
ordinal: 5
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
共有リポジトリ、PR worktree、レビュー進捗、差分キャッシュ、AI指摘、設定をどこへ保存し、どのように更新・破棄するかを決める。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 設定・PRキャッシュ・レビューセッション・worktree情報・固定backend identifierの保存先、寿命、正本、secret/non-secret境界が分類され、ADR-0011に従い認証secretがWindows Credential Manager以外へ保存されない
- [x] #2 再起動、複数PR、force-push、stale cache、corruption、backend unavailableの各シナリオについて期待動作と客観的なfixture検証結果が記録される
- [x] #3 ファイル方式と組み込みDB方式を、atomicity、concurrency、query、corruption recovery、migration、運用性の観点で比較し、重要な採用判断はユーザー承認を得て記録される
- [x] #4 schema version、transaction/atomic write、backup/recovery、retention、明示削除、worktree削除防止を含む非破壊の更新・削除・migration方針が定義される
- [x] #5 Windows nativeとWSL distroごとのpath/backend/Git状態を混在させないnamespaceとboundaryが定義され、Windows pathとWSL pathのfixtureで識別子衝突がないことを検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. 承認済みHybrid方式（LocalAppData内SQLite + content-addressed file cache、secretはWindows Credential Managerのみ）をADRとして記録する。
2. SQLiteの実storage fixtureに再起動、複数PR/force-push、transactional migration rollback、corruption検知・backup recovery、backend unavailable時のno-fallbackを実装する。
3. file cacheのdigest検証、missing/corrupt参照修復、未参照blobのorphan GCとdurable state非破壊を検証する。
4. READMEとTASK-5へ承認内容・客観的検証結果を反映し、AC/DoD/final summaryを更新する。
5. Rust検証、Backlog/ADR検証、diff確認後、TASK-5関連変更だけをcommitする（push/PR/mergeは対象外）。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-29 調査: ADR-0011のsecret境界（GitHub secretはWindows Credential Managerのみ、平文file/DB/WSLへのfallback禁止）、ADR-0002のworktree単位backend固定、ADR-0006/0008のremote PRとlocal Git状態分離を永続化境界の前提とした。Windows machine-local stateはFOLDERID_LocalAppDataに集約し、pathはbackend-tagged identityとしてWindows/WSL/distro間で暗黙変換しない。Microsoft WSL資料とSQLite公式資料（atomic commit/WAL/Backup API/corruption/integrity_check）を一次資料としてspikes/persistence-cache/README.mdに比較・障害・migration/delete方針を記録した。

Decision gate: Hybrid（SQLite structured state + content-addressed file cache、推奨）/SQLite only/Files onlyを選択肢とした。Hybridはtransaction整合性とcache evictionを両立するがorphan GCが必要、SQLite onlyはDB肥大とbackup/VACUUM cost、Files onlyは複数file atomicity/locking/migrationを自前実装する。Orca Dispatchがagent_prompt_stalledで失効しaskを送れなかったため、coordinator Runへ同内容をstatus送信しユーザー判断を依頼した。採用は未確定。

Fixture検証: `cargo fmt --manifest-path spikes/persistence-cache/Cargo.toml`成功、`cargo clippy --manifest-path spikes/persistence-cache/Cargo.toml --all-targets -- -D warnings`成功、`cargo test --manifest-path spikes/persistence-cache/Cargo.toml`は7 tests成功、`git diff --check`成功。検証範囲は複数account/PR、force-push revision分離、corrupt cache検知とreview session保持、Windows/WSL/distro path namespace、非UTF-8 WSL path保持、backend unavailable時のno-fallback。architecture承認待ちのため、実storageの再起動/transactional migration/corruption recovery検証、ADR、AC/DoD/final summary、commitは未実施。

2026-08-29 ユーザー承認: Hybrid方式に限定する。非secret structured stateはWindows FOLDERID_LocalAppData配下のSQLite、大容量diff/blobはcontent-addressed file cache、secretはWindows Credential Managerのみとし、orphan GCを設計・検証対象に含める。

2026-08-29 承認反映・実storage検証: Accepted ADR-0012としてHybrid方式を確定し、ADR-0002/0011へ相互linkした。`rusqlite` bundled SQLiteとtemporary filesystemを用いたfixtureを追加し、close/reopen後の複数account/PR・force-push revision保持、v1→v2 migration失敗時のDDL/`user_version`同時rollback、SQLite Backup API backupからの破損DB復元・破損原本quarantine、backend binding再起動保持、SHA-256 content-addressed cacheの未参照blob・欠損index・digest不一致file GC、およびGC後のreview draft保持を検証した。`cargo fmt --manifest-path spikes/persistence-cache/Cargo.toml`、`cargo clippy --manifest-path spikes/persistence-cache/Cargo.toml --all-targets -- -D warnings`、`cargo test --manifest-path spikes/persistence-cache/Cargo.toml`（12 tests）、`git diff --check`、`mise run adr-doctor`、`mise run backlog-check`はいずれも成功。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
ユーザー承認済みHybrid方式をAccepted ADR-0012へ記録し、非secret structured stateをLocalAppData内SQLite、大容量diff/blobをSHA-256 content-addressed file cache、secretをWindows Credential Managerだけに置く境界を確定した。実SQLite/file fixtureで再起動、複数PR/force-push、transactional migration rollback、破損DBのbackup recovery、backend unavailable時のno-fallback、orphan/missing/corrupt cache GCとdurable draft保持を12 testsで検証した。fmt、clippy -D warnings、test、git diff --check、adr-doctor、backlog-checkは成功し、push/PR/mergeは依頼どおり未実施のためタスク全体はPR作成待ち。
<!-- SECTION:FINAL_SUMMARY:END -->
