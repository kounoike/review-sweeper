# ローカル状態・永続化・キャッシュ調査

このspikeはTASK-5の調査記録と、identity/state boundaryおよび承認済みHybrid方式の実storage fixtureである。製品実装ではないが、SQLite transaction、再起動、backup recovery、content-addressed cacheとorphan GCを実際のfilesystem上で検証する。

## 既決の境界

- ADR-0011に従い、access token、refresh token、認証code、PKCE verifierはWindows Credential Managerだけに保存する。SQLite、cache file、設定file、WSL、環境変数、process引数へのfallbackは禁止する。
- ADR-0002に従い、実行backendはworktree作成時に`windows-native`または`wsl:<app-generated-binding-id>:<distribution>`へ固定する。利用不能時に別backendへ暗黙fallbackせず、review機能を継続しつつlocal準備だけをblockedとして明示的なrebindを求める。
- ADR-0006/0008に従い、GitHub由来のPR revisionとsource worktreeのGit状態は別の正本とする。local変更をremote diff/cacheへ混入させず、変更済みworktreeをcache cleanupと一緒に削除しない。

## 保存対象の分類

| 対象 | 正本・保存先 | identity | 寿命と破棄 | secret境界 |
| --- | --- | --- | --- | --- |
| アプリ設定 | Windowsの`FOLDERID_LocalAppData`配下のdurable state | installation/profile | 明示resetまたは互換migrationまで保持 | token値を含めない。credential lookup用のopaque account IDだけ可 |
| GitHub認証情報 | Windows Credential Manager | account ID + credential target | logout、失効、refresh置換で削除/更新 | 唯一のsecret保存先。利用不能時はfail closed |
| PR metadata/cache index | durable structured state | account ID + repository database ID + PR number + head SHA | refreshで更新。head変更時は旧revisionをstale化 | response header/bodyにsecretを残さない |
| diff/blob cache | 再取得可能なcache領域 | content digest + source revision | LRU/容量/期限で削除可。欠損・checksum不一致は再取得 | Authorization header、signed URL等を保存しない |
| review session | durable structured state | PR identity + head SHA | 明示discard/submit後のretention policyまで保持 | draft本文は非secretだがユーザー生成データとしてcacheより強く保護 |
| AI指摘 | revision-scopedな準備結果 | PR identity + head SHA + model/prompt/tool version | force-pushまたは生成条件変更でstale化。再生成可能だがユーザーが採否を記録した結果はsession側へ保持 | source抜粋を含み得るためtelemetry/logへ流さない |
| worktree record | durable structured state | app生成worktree ID | 実在確認後にtombstone。dirty/brokenなら自動削除禁止 | credentialを含めない |
| backend binding | worktree recordのimmutable field | tagged `windows-native` / `wsl:<app-generated-binding-id>:<distribution>` | 明示rebindまたはworktree削除まで保持 | backendへGitHub tokenを配らない |
| observed Git state | refresh可能なsnapshot | worktree ID + observed HEAD + generation | 起動時/イベント後に再観測し、古いsnapshotを表示しない | 非secret |

Windowsのmachine-local stateは`FOLDERID_LocalAppData`に置く。Microsoftもmachine-specific app dataの保存先として同known folderを案内している。WSL内にはアプリDB/cacheを複製せず、Windows native processだけが永続化を所有する。

SQLiteを選ぶ場合、databaseとjournal/WALはWindows native processからlocal NTFS上で扱い、WSL filesystemのUNC pathやnetwork filesystemへ置かない。SQLiteのWALは同一host上のshared memoryを前提とし、databaseだけを別backendから直接開く設計にしない。

## 障害シナリオと期待動作

| シナリオ | 期待動作 | 保護する状態 |
| --- | --- | --- |
| 再起動 | schema/checksumを検証してdurable stateを再開し、worktree/Git/backend availabilityは再観測する | review progress、draft、binding |
| 複数PR/account | `(account_id, repository_id, pr_number)`でnamespace化し、revisionはさらに`head_sha`で分ける | 別PR・別accountの進捗 |
| force-push | 新headを別revisionとして作成し、旧cacheをstale化する。閲覧済み・draftを新headへ暗黙carryしない | 旧revisionのdraftと監査可能性 |
| stale/missing cache | cacheだけを再取得する。review sessionやworktreeを削除しない | durable user state |
| checksum不一致 | blobをquarantine/削除して再取得する。DB indexと実体のorphanを起動時/低優先度GCで修復する | durable user state |
| structured state破損 | read-onlyで退避し、直近の整合backupから復元を試みる。空DBで上書きしない | 原本と復旧可能性 |
| backend unavailable | PR/diff/draft閲覧は継続し、local準備をblocked表示する。自動fallback/パス変換しない | backend/path identity |
| WSL distro差替え | 同名pathでもbackend IDが違えば別物として扱い、明示rebindとGit/worktree再検証を要求する | Git状態の混同防止 |

## 保存方式の比較と採用判断

| 案 | atomicity/concurrency/query | corruption/migration | cache運用 | 主なtradeoff |
| --- | --- | --- | --- | --- |
| Hybrid（推奨） | non-secret structured stateを単一SQLite transactionで更新 | `user_version`とtransactional migration、起動時`quick_check`、SQLite Backup APIを利用 | 大容量diff/blobはcontent-addressed fileとして独立eviction | DB indexとblobのorphan GCが必要 |
| SQLite only | 全状態を一transactionで扱いやすい | 同上 | blobもbackup/VACUUM対象になりDBが肥大 | cache削除・backup costが大きい |
| Files only | 単一fileのtemp+fsync+renameは可能だが複数file transaction/locking/queryを自前実装 | versioned documentごとのmigrationと破損判定を自前実装 | directory単位のevictionは容易 | 複数PR・draft・index間のpartial update riskが最大 |

SQLiteはtransactionのatomic commitとcrash後のjournal recoveryを提供する。一方、database fileと`-wal`/`-shm`を実行中に別々にcopy/renameしてはならず、backupはSQLite Backup APIを使う。破損検知は起動時の軽量checkと保守時の`integrity_check`を使い、破損fileを即削除しない。

ユーザー承認によりHybrid方式を採用し、Accepted ADR-0012へ記録した。非secret structured stateは`FOLDERID_LocalAppData`配下のSQLite、大容量diff/blobはcontent-addressed file cache、secretはWindows Credential Managerだけに保存する。SQLite onlyとFiles onlyは上記tradeoffにより不採用とした。

## 更新・削除・migration policy

1. durable structured stateの全writeはtransactionで行い、schema versionを記録する。migration前のbackupは既存の正常世代を削除せず、同一directoryの一意temporary fileへSQLite Backup APIで完成させる。`quick_check`とfile syncが成功した後だけ固定backup名へreplace publishし、通常の失敗ではtemporaryをdrop cleanupする。Windowsでは通常の`rename`による既存destination上書きに依存せず、`MoveFileExW(MOVEFILE_REPLACE_EXISTING)`相当のreplace semanticsを使うため、publish前の失敗・crashでは最後の正常backupへ触れない。migrationは一transactionとし、失敗時はrollbackする。
2. cacheは再取得可能、review session/draft・設定・worktree bindingはdurable user stateとして扱う。容量圧迫時もcacheから削除し、durable stateをevictしない。
3. cache blobはSHA-256 digest検証後にpublishし、未参照blob、欠損index、digest不一致file、publish途中のtemporary fileを起動時または低優先度保守のorphan GCで修復する。GCはcache index/fileだけを対象とし、review session/draft、設定、worktree bindingを削除しない。force-pushは旧revisionを即削除せずstale/tombstoneとしてretention期限まで保持する。
4. worktree削除は実在、record ownership、dirty/staged/untracked/conflicted、process利用中を再確認し、ユーザー確認なしにdirty worktreeを削除しない。DB record削除とfilesystem削除を同一transactionとみなさず、intent/resultを段階記録する。
5. unknownな将来schema、migration失敗、DB corruptionでは新規の空stateで上書きせず、read-only診断とbackup復元を提示する。credential storeの障害はこの復旧経路と分離してfail closedにする。

## Windows / WSL path boundary

- pathは文字列だけでidentityにせず、必ず`BackendPath { backend_id, encoding, native_path_bytes }`として保存する。WindowsはWTF-8相当、WSL/Linuxはnative bytesをlosslessに保持し、UI用のlossy display stringを正本にしない。`C:\src\repo`と`/mnt/c/src/repo`を同一と推測しない。
- Windows pathはWindows API、WSL pathは固定distro内のLinux API/gitで解決・検証する。display/interop用変換値はderived cacheであり正本にしない。
- WSLの各distributionは独立したLinux filesystemを持つ。MicrosoftもLinux toolにはWSL filesystem、Windows toolにはWindows filesystemを使い、OSをまたぐtight I/Oを避けるよう案内している。
- WSL backendが停止・未登録・名称不一致の場合、保存pathをWindows UNCへ変換して続行せずunavailableとする。明示rebind時は新backendでrepository identity、worktree common dir、HEADを再検証する。
- WSLのdisplay nameだけをidentityにしない。明示bind時に生成したimmutable IDと、その時点のdistribution selectorを保存する。再import/rename/rebindでは新IDを発行し、同じpath文字列でも旧bindingのGit状態を流用しない。

## Fixture

`src/lib.rs`は保存方式に依存しない境界を、`src/storage.rs`は実SQLite/file storageをtestする。

```console
cargo test --manifest-path spikes/persistence-cache/Cargo.toml
```

検証対象は次のとおりである。

- 複数account/PRのnamespace、force-push後のrevision分離と旧draft保持
- Windows/WSLおよびWSL distro間のpath分離、非UTF-8 WSL pathのlossless保持
- backend bindingの再起動後保持と、backend unavailable時のno-fallback
- SQLite fileをclose/reopenする実再起動と、複数sessionの永続化
- schema v1からv2へのmigration途中の意図的失敗で、DDLと`user_version`が同時rollbackすること
- SQLite Backup APIで作成したbackupからの破損database復元と、破損原本のquarantine
- 新backupのpublish前に失敗させた場合も最後の正常backupがbyte単位で維持され、temporaryがcleanupされ、そのbackupから復元できること
- SHA-256 content-addressed blobのpublish、未参照blob、欠損index、digest不一致fileのorphan GC、およびGC後もreview draftが保持されること

## 一次資料

- [Microsoft: KNOWNFOLDERID](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid)
- [Microsoft: Windows app restore / machine-specific app data](https://learn.microsoft.com/en-us/windows/apps/develop/windows-app-restore)
- [Microsoft: Working across file systems](https://learn.microsoft.com/en-us/windows/wsl/filesystems)
- [Microsoft: WSL interop](https://learn.microsoft.com/en-us/windows/dev-environment/wsl-interop)
- [SQLite: Atomic Commit](https://sqlite.org/atomiccommit.html)
- [SQLite: Write-Ahead Logging](https://sqlite.org/wal.html)
- [SQLite: Backup API](https://sqlite.org/backup.html)
- [SQLite: How To Corrupt An SQLite Database File](https://sqlite.org/howtocorrupt.html)
- [SQLite: PRAGMA integrity_check](https://sqlite.org/pragma.html#pragma_integrity_check)
