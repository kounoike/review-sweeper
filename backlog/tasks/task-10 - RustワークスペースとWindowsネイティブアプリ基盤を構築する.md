---
id: TASK-10
title: RustワークスペースとWindowsネイティブアプリ基盤を構築する
status: Done
assignee:
  - '@kounoike'
created_date: ''
updated_date: '2026-08-30 21:02'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-1
  - TASK-3
  - TASK-7
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
modified_files:
  - .gitignore
  - Cargo.toml
  - Cargo.lock
  - mise.toml
  - apps/review-sweeper/Cargo.toml
  - apps/review-sweeper/src/main.rs
  - crates/domain/Cargo.toml
  - crates/domain/src/lib.rs
  - crates/application/Cargo.toml
  - crates/application/src/lib.rs
  - crates/integrations/Cargo.toml
  - crates/integrations/src/lib.rs
  - crates/execution/Cargo.toml
  - crates/execution/src/lib.rs
  - crates/ui-gpui/Cargo.toml
  - crates/ui-gpui/src/lib.rs
  - crates/architecture-tests/Cargo.toml
  - crates/architecture-tests/src/lib.rs
  - docs/architecture/rust-workspace.md
  - docs/development/windows.md
  - backlog/tasks/task-10 - RustワークスペースとWindowsネイティブアプリ基盤を構築する.md
type: chore
ordinal: 10
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0001に基づき、選定したUIフレームワークを使うRustワークスペースと、Windowsで起動できる最小のネイティブアプリ基盤を構築する。GitHub、git、AI、UI、実行環境の関心事を後続タスクで分離できる構成にし、製品機能は含めない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Cargo workspaceでcomposition root、domain、application、external integration adapter、GPUI UI adapter、execution backend adapterを別crateにし、cargo metadataと依存関係のテストで境界を確認できる
- [x] #2 x86_64-pc-windows-msvc上でGPUI 0.2.2の製品binaryをbuildし、Windows native processとして最小ウィンドウを起動できる
- [x] #3 repository rootからmise run fmt、lint、test、check、buildを製品workspaceに対して再現でき、各commandが成功する
- [x] #4 Windows 11、VS 2022 C++ toolchain、Windows SDK、Rust MSVC target、miseを含む開発依存とnative build/run手順、UNC checkout時のtarget directory制約が文書化されている
- [x] #5 GitHub、git、AI、Review Inbox、diff、terminal、WSLの具体的な製品機能を実装せず、後続taskがadapter crateを拡張できる責務表を文書化している
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. TASK-1/3/7のspike・技術メモとADR-0001/0002/0003/0010から、GPUI pin、Windows MSVC条件、ExecutionBackend責務を製品workspaceへ引き継ぐ。
2. root Cargo workspaceと、composition root、domain、application、integrations、GPUI UI、execution adapterの最小crateを作成し、依存方向をmanifestとarchitecture testで固定する。
3. GPUI 0.2.2で製品名を表示する最小Windows native window、起動時logging、構造化された起動errorを実装する。
4. mise taskを製品workspace向けに更新し、crate責務、後続拡張点、Windows native開発・build・run手順を文書化する。
5. hostでfmt/lint/test/check/buildとBacklog/ADR/diff検証を行い、Windows MSVCでfmt/clippy/test/buildおよびnative起動を再検証する。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-31 着手確認: 作業ツリーはclean。依存TASK-1、TASK-3、TASK-7はDone。ADR-0001/0002/0003/0010はAccepted、mise run adr-list/adr-doctorは成功。TASK-10外の製品機能や公開API、release方式は確定せず、既存spikeのGPUI 0.2.2とWindows x64 MSVC基準を製品workspaceへ移す。

2026-08-31 実装: root Cargo workspaceにcomposition root、domain、application、integrations、execution、GPUI UI、architecture testの7 packageを追加した。GPUI 0.2.2はcfg(windows) dependencyに限定し、Windows以外は構造化UnsupportedPlatform errorを返すため、Windows-only GUIという現スコープを保ちながらhost quality gateを実行可能にした。起動時はtracing subscriberを初期化し、window作成errorをLaunchErrorからcomposition rootの終了code 1へ伝播する。crate責務と後続拡張点はdocs/architecture/rust-workspace.md、Windows依存・mise手順・UNC target制約はdocs/development/windows.mdに記録した。

2026-08-31 検証: WSL hostでmise run fmt、lint、test、check、buildがすべて成功し、application test 1件とCargo metadata依存方向test 1件が成功した。git diff --check、mise run backlog-check、mise run adr-doctor、mise tasks --json（15 task）も成功した。Windows 11 10.0.22631、rustc 1.98.0 x86_64-pc-windows-msvc、VS 2022 MSVC 14.39でmise taskと同一のcargo fmt/clippy -D warnings/test/dev build/release build command bodyが成功した（Windows hostにはmise.exe未導入のためtask schemaはWSL miseで検証）。release EXEは10,936,832 bytes、SHA-256 89758ACE6C350F2D784A33EDFB77BAE5C87F699E42A283448B9E439B6F994843。Windows native processを起動し、Responding=true、MainWindowTitle=Review Sweeper、non-zero HWND=28714310を確認し、CloseMainWindowでexit code 0を確認した。既知事項はGPUI依存proc-macro-error2 2.0.1のfuture-incompatibility warningで、clippy warningではない。

2026-08-31 ユーザーがPR #9を承認。全AC・DoD、WSL quality gate、Windows MSVC buildおよびnative window起動の記録を確認し、merge前にDoneへ更新した。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
ADR-0001/0002/0003/0010に従い、GPUI 0.2.2をWindows限定adapterに閉じ込めたRust workspace、composition root、domain/application/external integration/execution境界、loggingと構造化起動error、Cargo metadataによる依存方向testを構築した。Windows開発依存・mise task・UNC checkout制約とcrate責務を文書化し、GitHub、git、AI、Review Inbox、diff、terminal、WSLの具体機能は後続taskへ残した。WSL hostのmise run fmt/lint/test/check/buildとBacklog/ADR/diff検証、Windows 11・Rust 1.98.0・MSVC 14.39のfmt/clippy/test/dev/release buildが成功し、release EXEのReview Sweeper windowをnon-zero HWNDで起動して正常終了を確認した。ユーザー承認後、全AC・DoDと検証記録を再確認してDoneへ更新した。
<!-- SECTION:FINAL_SUMMARY:END -->
