---
id: TASK-7
title: 実行バックエンド境界とWindowsネイティブ経路を技術検証する
status: Done
assignee:
  - '@kounoike'
created_date: '2026-08-23 00:49'
updated_date: '2026-08-29 04:04'
labels:
  - project-setup
  - worktree
milestone: m-0
dependencies: []
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - doc-3
  - doc-9
modified_files:
  - >-
    backlog/docs/technical/execution-backend/doc-9 -
    ExecutionBackend境界とWindowsネイティブ実行の技術検証.md
  - backlog/tasks/task-7 - 実行バックエンド境界とWindowsネイティブ経路を技術検証する.md
  - spikes/execution-backend/.gitignore
  - spikes/execution-backend/Cargo.toml
  - spikes/execution-backend/Cargo.lock
  - spikes/execution-backend/src/lib.rs
  - spikes/execution-backend/src/bin/execution_backend_fixture.rs
  - spikes/execution-backend/tests/backend.rs
type: spike
ordinal: 7
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0002に基づき、GUIからコマンド実行環境を分離する共通境界と、初期ターゲットであるWindowsネイティブ実行経路を実装前に検証する。WSL固有機能は実装せず、後から同じ境界へ追加できることを確認する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 技術メモでgit・Worktree操作・workspace setupに共通なCommandRequest/CommandOutcome相当の契約を定義し、argv、cwd、env差分、cancel、stdout/stderr log、exit/launch/IO errorの境界とsecret非注入原則を明示する
- [x] #2 技術メモでWorktreeごとにbackend kindと安定identifierを固定して永続化・表示するモデル、不一致時に暗黙fallbackせず再選択を要求する動作を定義する
- [x] #3 Rustの最小prototypeと自動テストでWindows native processの正常終了、非zero終了、起動失敗、cancel、stdout/stderr分離取得を検証し、Windows実機未実行項目は再現コマンドと制約を明記する
- [x] #4 Windows host pathとbackend内pathを別型として扱い、暗黙変換を禁止するAPI境界と、backendが責任を持つ明示変換・検証・変換不能errorを技術メモおよび型/テストで示す
- [x] #5 WSL2、macOS native、Linux nativeを既存境界へ追加する拡張点、ADR-0002とTASK-2/ADR-0011の認証・secret境界を維持する責務、再検討条件を技術メモに記録し、WSL固有実装を含めない
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. ADR-0002、ADR-0011、TASK-2/TASK-13/TASK-27を境界として、承認済み修正版A案の対象・非対象を固定する。
2. Review Sweeper独自のExecutionBackend、CommandRequest、BackendPath、LogEvent stream、Completion、cancel/error、backend identifier、secret非注入契約をRust型と技術メモで定義する。
3. stdベースのWindowsNativeBackend最小prototypeを作り、process-wrapのWindows Job Object/POSIX process groupを第一候補として正常終了・非zero・起動失敗・cancel・stdout/stderr分離・path型を自動テストする。
4. command-groupを前身、processkitをTokio前提の将来候補として比較し、Tokio採用と高度なprocess-tree supervisionはTASK-13、WSL実装はTASK-27へ残す。WSL adapter責務とmacOS/Linux native拡張点、認証・secret境界を技術メモへ記録する。
5. fmt/lint/test/check/build、Windows target check、git diff --check、backlog-check、adr-doctorを実行し、客観的証拠でAC/DoD・notes・final summaryを更新してTASK-7関連変更だけをcommitする。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-29 承認済み修正版A案を反映: Review Sweeper独自のExecutionBackend境界を正本とし、stdベースWindowsNativeBackend prototypeではprocess-wrap 10.0.0のWindows Job Object/POSIX process groupを第一候補として採用した。command-group 5.0.1は前身、processkit 3.3.4はTokio前提の高機能な将来候補として比較し、Tokio採用、graceful cancel、高度なprocess-tree supervisionはTASK-13へ残した。WSLはWindows側wsl.exe adapterとWSL内部process管理の責務を分離し、実装はTASK-27へ残した。

契約結果: CommandRequestはshell-freeなprogram/argv、backend-bound cwd、非secretのenv差分を持つ。stdout/stderrはbyte LogEvent streamとして分離し、nonzero exit/cancelのCompletionとlaunch/IO/path/backend errorを分離した。Worktreeはkindとstable identifierを固定し、不一致時は暗黙fallbackせず再選択を要求する。HostPathとBackendPathは別型で、明示変換・absolute検証・backend mismatch errorをprototypeとcompile-fail testで確認した。ADR-0011のsecretをargv/env/logへ注入しない境界を維持した。

検証結果 (2026-08-29):
- mise exec -- cargo test --manifest-path spikes/execution-backend/Cargo.toml --all-features: 成功。6 integration test（正常終了、nonzero、launch失敗、cancel、stdout/stderr分離、stable identifier/path mismatch）と1 compile-fail doc test（HostPath/BackendPath分離）が成功。
- host: manifest指定のcargo fmt --check、clippy --all-targets --all-features -D warnings、check、buildが成功。
- Windows cross target: x86_64-pc-windows-msvc向けcargo checkとclippy -D warningsが成功し、JobObject分岐のcompileを確認。現在はWSL2/Linux環境でMSVC binaryをlink/runできないためWindows Job Object runtimeは未実測。doc-9にWindows実機のcargo test/clippy再現commandと制約を記録。
- git diff --checkおよびgit diff --cached --check: 成功。
- mise run backlog-check: 成功。
- mise run adr-doctor: 成功 (No issues found)。
- rootのmise run fmt/lint/test/build/checkはrepository rootにCargo.tomlがなく失敗した。root Rust workspace構築はTASK-10のscopeであり、TASK-7 spikeはmanifest指定の同等commandですべて成功した。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Review Sweeper独自のExecutionBackend契約とstdベースWindowsNativeBackend prototypeを追加し、process-wrapのWindows Job Object/POSIX process groupを第一候補としてCommandRequest、BackendPath、stdout/stderr LogEvent、Completion、cancel/error、stable identifier、secret非注入を具体化した。技術メモではcommand-group/processkit比較、Worktree固定とno-fallback、WSL adapter責務分離、macOS/Linux拡張点、TASK-13/TASK-27への残課題を記録した。Linux/WSL上の6 integration testと1 compile-fail test、host fmt/clippy/check/build、Windows MSVC target check/clippy、git diff check、backlog-check、adr-doctorに成功した。Windows Job Object runtime実測はWindows実機用再現commandを記載して残した。
<!-- SECTION:FINAL_SUMMARY:END -->
