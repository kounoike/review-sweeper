# 12. 非secret構造化状態と大容量cacheにHybrid永続化を採用する

Date: 2026-08-29

## Status

Accepted

Relates to [2. WindowsネイティブGUIとWSL実行バックエンドを分離する](0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md)

Relates to [11. GitHub認証とsecretをWindowsネイティブ境界で所有する](0011-github-secret-windows.md)

## Context

Review Sweeperは、設定、PR metadata、review session、AI指摘、worktreeとbackend bindingなどの非secret構造化状態を再起動後も保持する。一方、diffやblobは大容量になり、再取得可能で、容量・期限による独立evictionが必要である。複数PR、force-push、並行更新、schema migration、破損復旧をfile群だけで整合させると、複数file transaction、locking、query、version管理をアプリケーション側で再実装することになる。

SQLiteだけに大容量blobも格納する案はtransaction境界を単純にできるが、database、backup、VACUUMの肥大とcache evictionの運用costが大きい。structured stateとfile cacheを分けるHybrid案では、transaction整合性と独立evictionを両立できる一方、DB indexとblob fileのorphanを修復する必要がある。

ADR-0011はGitHubのaccess token、refresh token、認証code、PKCE verifierなどのsecretをWindows Credential Managerだけに保存し、SQLite、file、WSL等へのfallbackを禁止している。本Decisionはそのsecret境界を変更しない。

## Decision

非secret構造化状態と大容量cacheにHybrid永続化を採用する。

- Windows MVPでは、非secret構造化状態を`FOLDERID_LocalAppData`配下のlocal NTFS上に置く単一SQLite databaseへ保存する。WSL filesystem、UNC path、network filesystemへdatabaseを置かず、Windows native processだけがdatabaseとWALを所有する。
- 設定、PR metadata/cache index、review session/draft、AI指摘の状態、worktree record、immutable backend bindingをSQLite transactionで更新する。account、repository、PR、head SHA、app生成worktree ID、backend binding IDを明示的にnamespace化する。
- 大容量diff/blobはSHA-256 digestを名前とするcontent-addressed file cacheへ保存する。temporary fileへwrite、flush、publishした後にSQLite indexから参照し、digest不一致や欠損は再取得可能なcache障害として扱う。
- 起動時または低優先度保守でorphan GCを行う。未参照file、欠損index、digest不一致fileを修復するが、review session/draft、設定、worktree bindingなどのdurable stateはcache GCで削除しない。
- SQLite schemaは`user_version`で管理し、migration前にSQLite Backup APIで整合backupを作成する。migrationは単一transactionとし、失敗時はrollbackする。起動時に`quick_check`、保守・診断時に`integrity_check`を使い、破損databaseは削除や空DB上書きをせずquarantineして有効なbackupから復元する。未知の将来schemaではread-only診断と明示的な復旧を要求する。
- force-push後のhead SHAは別revisionとして作成し、旧revisionのreview stateを暗黙にcarryしない。cacheはstale化してretention後に回収できるが、ユーザー生成draftは明示discardまたは定義済みretentionまで保持する。
- worktree削除はcache evictionと分離し、実在、所有record、dirty/staged/untracked/conflicted、process利用中を再確認する。filesystem削除とDB更新は一つのtransactionとみなさずintent/resultを段階記録し、変更済みworktreeを自動削除しない。
- GitHub認証secretはADR-0011どおりWindows Credential Managerだけに保存する。SQLiteとcacheにはopaque account IDやcredential target identifierだけを保存でき、secret store利用不能時はfail closedとする。

## Consequences

- 複数のstructured recordをtransactionで整合させ、query、concurrency、migrationをSQLiteの保証へ委ねられる。
- 大容量cacheをdurable user stateと独立して容量・期限で回収でき、database backupとVACUUMの負荷を抑えられる。
- database、backup、cache directoryの配置と権限をWindows側で一元管理でき、Windows/WSL間のpath・backend・Git状態の混在を避けられる。
- DB indexとblob publishは単一filesystem transactionにならないため、idempotentなpublishとorphan GC、欠損・破損時の再取得が必須になる。
- backup generation、retention、quarantine、recovery UI、GC schedulingと容量上限は製品実装時に具体化する必要があるが、本Decisionの非破壊境界を変更してはならない。
