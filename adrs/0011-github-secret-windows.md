# 11. GitHub認証とsecretをWindowsネイティブ境界で所有する

Date: 2026-08-29

## Status

Accepted

Relates to [2. WindowsネイティブGUIとWSL実行バックエンドを分離する](0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md)

## Context

Review SweeperはWindowsネイティブGUIからGitHub APIを利用し、必要に応じてagent、terminal、git、worktree、workspace setupをWindows nativeまたは選択したWSL distroで実行する。GitHub認証情報をWSLや各実行プロセスへ配布すると、secretの複製、失効漏れ、ログやプロセス引数への露出、実行backendごとの不整合が生じる。

またGitHub Notifications REST APIは採用するGitHub App user access tokenに対応しないため、GitHub Notifications互換の受信箱と最小権限のGitHub App認証は両立しない。MVPのReview Inbox semantics、認証フロー、token lifecycle、secret storeの失敗時動作を一つの信頼境界として確定する必要がある。

## Decision

Review InboxはGitHub Notificationsの未読・既読状態と互換にせず、review request/searchとPR metadataを正本とする。

MVPの認証にはGitHub App user access tokenを使う。WindowsネイティブGUIがsystem browserを開き、authorization code flow with PKCEで認証する。GitHub App private keyはdesktopへ配布しない。access token、refresh token、認証code、PKCE verifierなどのsecret lifecycleはWindows側が所有する。

MVPの永続secret storeにはWindows Credential Managerを使う。アプリケーション層には共通`CredentialStore`契約を置き、将来のmacOS/Linux native対応では各OSのsecret store backendを追加する。secret storeが利用できない、書き込みまたは更新に失敗した場合は平文file、database、registry、環境変数、プロセス引数へfallbackせずfail closedとし、accountを利用可能にしないか再認証を要求する。

agent、terminal、git、worktree、workspace setupの実行backendはWindows nativeを維持し、Worktreeごとに選択したWSL distroを`wsl.exe`経由で実行できる。実行backendはGitHub credentialを所有せず、WSLへtokenを恒久コピーしない。GitHub API操作は原則としてWindows側の認証境界で実行し、実行backendへはsecretではなく必要最小限の非secretデータと操作結果を渡す。

## Consequences

- Review InboxはGitHub Notificationsの既読・未読とは一致しないため、review request、レビュー履歴、PR更新時刻から製品独自の状態を算出し、その意味をUIで明示する必要がある。
- selected repositoryと最小permission、短命access tokenを利用できる一方、GitHub Appのinstallおよびorganization approvalが必要になる場合がある。
- GUI/Windows側に認証callback、single-flight refresh、失効・権限変更検知、logout、再認証、redaction、複数accountの責務が集約される。
- Windows Credential Managerが利用不能な環境ではGitHub機能を継続できないが、secretを安全性の低い保存先へ暗黙に移さない。
- WSLを含む実行backendを切り替えてもcredentialの正本は一つに保たれる。WSL内のツールがGitHub API tokenを直接必要とする用途は自動対応せず、別の明示的な設計判断が必要になる。
- macOS/Linux native対応では`CredentialStore`契約を再利用できるが、各OS backendの実装とfail-closed動作を個別に検証する必要がある。
