---
id: TASK-2
title: GitHub API・認証境界・token lifecycleを設計する
status: Done
assignee:
  - '@kounoike'
created_date: '2026-08-20 18:06'
updated_date: '2026-08-29 02:34'
labels:
  - project-setup
milestone: m-0
dependencies: []
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0011-github-secret-windows.md
documentation:
  - doc-3
  - doc-6
modified_files:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0011-github-secret-windows.md
  - >-
    backlog/docs/technical/github-integration/doc-6 -
    GitHub-API・認証・トークン保管技術調査.md
  - backlog/tasks/task-2 - GitHub-APIと認証・トークン保管を調査する.md
type: spike
ordinal: 2
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
WindowsネイティブGUIを信頼境界として、Review Inbox、PRメタデータ、差分、コメント、Checks、レビュー送信に必要なGitHub API、GitHub App user access tokenの認証フロー、token lifecycle、secret保管を設計する。コマンド実行backendがWindows nativeまたはwsl.exe経由の選択済みWSL distroであっても、GitHub認証情報はGUI/Windows側が所有し、WSLへ恒久コピーしない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Review InboxはGitHub Notifications互換ではなくreview request/searchを正本とし、必要なREST/GraphQL API、query、権限、レート制限、ページング、およびNeeds Review・Updated Since Review・Reviewedへの分類根拠が整理されている
- [x] #2 MVPの認証方式がGitHub App user access token、system browser、authorization code flow with PKCEとして定義され、開始、callback、キャンセル、再認証、logoutのユーザー体験とエラー処理が整理されている
- [x] #3 tokenの取得、更新、期限切れ、失効、権限変更、logout、再認証を含むlifecycleをWindowsネイティブGUI側が所有し、secretをログ、診断情報、プロセス引数へ露出させない設計と検証方法が定義されている
- [x] #4 MVPではtokenをWindows Credential Managerへ保存し、将来のmacOS/Linux native対応では共通CredentialStore契約にOS別secret store backendを追加でき、secret store利用不能時は平文fallbackせずfail closedとなる契約が定義されている
- [x] #5 選択したWSL distroをwsl.exe経由で起動するbackendを含め、agent、terminal、git、worktree、workspace setupの実行backendとGitHub認証境界が分離され、WSLへtokenを恒久コピーせずGitHub操作を原則Windows側で行う責務・データフロー・エラー境界が定義されている
- [x] #6 APIエラー、権限不足、レート制限、ネットワーク障害、CredentialStore利用不能時のユーザー向け動作と、認証・secret境界を検証するテスト観点が整理されている
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. 現worktreeのTASK-2とdoc-6を正本として、task-orchestrator worktreeのTASK-2から承認事項と具体化した受け入れ条件を抽出する。
2. doc-6へReview Inbox分類、認証UXとtoken lifecycle、CredentialStore契約、Windows/WSL責務境界、fail-closed動作、検証観点を統合する。
3. 既存ADRとの重複を確認し、承認済みの認証・secret信頼境界を新規Accepted ADRとして記録してADR-0002へ関連付ける。
4. 文書を受け入れ条件ごとに確認し、検証結果と完了サマリーをBacklog CLIで記録する。
5. git diff --check、mise run backlog-check、mise run adr-doctorと文書整合性確認を成功させ、TASK-2をDoneにして対象変更だけをコミットする。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-25: GitHub公式一次情報を基に、API、permission、pagination、rate/secondary limit、conditional request、webhook、認証方式、Windows secret store、error/offline/partial data UXをdoc-6へ整理した。

2026-08-29 承認済み判断: Review InboxはGitHub Notifications互換ではなくreview request/search基準とする。MVP認証はGitHub App user access token + system browser + authorization code flow with PKCE、token保存先はWindows Credential Managerとする。GitHub認証、token lifecycle、secretはGUI/Windows側が所有し、agent、terminal、git、worktree、workspace setupのWindows native/選択WSL distro実行backendへtokenを恒久コピーしない。将来のmacOS/Linux native対応は共通CredentialStore契約にOS別secret-store backendを追加し、利用不能時は平文fallbackせずfail closedとする。

統合結果: doc-6にInbox分類、callback/cancel/re-auth/logout、single-flight refresh、revoke/permission変更、CredentialStore error契約、Windows/WSLデータフロー、ユーザー向けerror動作、secret境界の検証観点を具体化した。重要な信頼境界はAccepted ADR-0011として記録し、Accepted ADR-0002へRelates toで関連付けた。task-orchestrator worktreeは参照のみで、同worktreeの未コミット変更は変更・破棄していない。

実装時確認: callback URIの具体値、GitHub App登録運用、Windows Credential Manager用crate、Contents/Commit statuses permissionの要否は、ADR-0011の境界を変えない範囲でWindows実機・endpoint fixtureにより確定する。webhook relayはMVP外で、導入時は別Decisionを要する。

検証結果 (2026-08-29):
- AC #1: doc-6「API サーフェス」「Review Inboxの分類契約」「pagination、cache、rate limit」でREST/GraphQL、query、permission、pagination/rate limitと3分類の根拠を本文確認。
- AC #2: doc-6「認証フローとユーザー体験」でsystem browser開始、PKCE state/verifier、callback、拒否・cancel・timeout、再認証、logoutを本文確認。
- AC #3: doc-6の認証フロー、操作契約、memory/log/crash対策、検証観点でrefresh/revoke/permission変更とsecret非露出を本文確認。
- AC #4: doc-6「CredentialStore契約とOS backend」とADR-0011でWindows Credential Manager、将来OS backend、error分類、平文fallbackなしのfail closedを本文確認。
- AC #5: doc-6「Windowsと実行backendの信頼境界」とADR-0011でWindows native/選択WSL distro、wsl.exe、非secretデータフロー、WSLへのtoken恒久コピー禁止を本文確認。
- AC #6: doc-6「error・offline・partial data UX」「検証観点」でAPI、permission、rate、network、CredentialStore、認証・secret境界の動作を本文確認。
- git diff --check: 成功。
- mise run backlog-check: 成功。
- mise run adr-doctor: 成功 (No issues found)。
- 文書構造・承認状態・ADR相互リンク確認: 成功。必須section、承認待ち表現の不存在、ADR-0002/0011相互linkをコマンドで確認。
- mise run check: Cargo.tomlがまだ存在しないpre-scaffold repositoryのためfmt/lint/testが起動できず対象外。Rust実装変更はなく、文書向け検証は上記のとおり成功。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
承認済み方針を正本のTASK-2とdoc-6へ統合し、Review Inboxの分類、GitHub App user access token + system browser + authorization code flow with PKCE、token lifecycle、Windows Credential Managerと共通CredentialStoreのfail-closed契約、Windows/WSL実行backendとのsecret境界、API/error UXと検証観点を具体化した。重要判断はAccepted ADR-0011としてADR-0002へ関連付けた。git diff --check、backlog-check、adr-doctor、文書構造・承認状態・ADR相互リンク確認に成功。Rust変更はなく、Cargo.toml未作成のためmise run checkは対象外。
<!-- SECTION:FINAL_SUMMARY:END -->
