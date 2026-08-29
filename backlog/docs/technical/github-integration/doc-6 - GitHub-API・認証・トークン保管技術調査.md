---
id: doc-6
title: GitHub API・認証・トークン保管技術調査
type: other
created_date: '2026-08-25 05:44'
updated_date: '2026-08-29 02:33'
tags:
  - github
  - api
  - authentication
  - security
  - windows
---
# GitHub API・認証・トークン保管技術調査

## 位置づけ

TASK-2 の調査メモ。調査日は 2026-08-25。対象は GitHub.com を使う Windows native desktop app の Review Inbox、PR 閲覧、diff、reviews/comments、Checks、レビュー送信である。実 credential の生成・保存、GitHub App/OAuth App の登録、外部 account・repository・webhook の変更は行っていない。

本書のReview Inbox semantics、認証方式、secret保管、Windows/WSL境界は2026-08-29にユーザー承認済みであり、ADR-0011に記録する。GitHub Appの登録値、callback URIの具体値、crate選定などの実装詳細は、ここで定める信頼境界を変更しない範囲で後続実装時に検証する。

## 結論（承認済み方針）

MVPは **GitHub App の user access token（user-to-server）** を採用する。Windows desktop から system browser を開く authorization code + PKCE を基本とし、user access token expiration を有効にする。repository permission は `Pull requests: write`、`Checks: read`、source blob を GitHub API から読む場合のみ `Contents: read`、commit status を別 API で読む場合のみ `Commit statuses: read` とし、install 時に選択 repository へ限定する。desktop には GitHub App private key を絶対に含めず、user access token と refresh token だけを OS secret store に保管する。

ただし GitHub Notifications REST API は公式仕様上 personal access token (classic) の `notifications` または `repo` scope に限定され、GitHub App user/installation token、OAuth App token、fine-grained PAT は利用できない。このためReview Inboxは `review-requested:@me`、`team-review-requested:`、参加・更新時刻等によるsearchとPR metadataの集合として定義し、GitHub Notificationsの既読/未読同期は行わない。classic PATや二重credentialはMVPで採用しない。比較結果として検討した選択肢は次のとおり。

1. classic PAT で `notifications` または private PR 詳細も含む `repo` scope を受け入れる。現行の Notifications endpoint は OAuth App token にも対応すると仮定しない。
2. GitHub App を主認証に保ち、Notifications だけ optional classic PAT とする（二重 credential、権限説明、account 不整合が増えるため非推奨）。
3. GitHub Notifications 互換を MVP 外とし、Review Inbox を review request/search ベースと明示する（推奨）。

webhook は GitHub App の長所だが、desktop 単体に public HTTPS receiver と安定 endpoint はない。MVP は conditional polling と manual refresh を採り、webhook を使う場合は署名検証・再送・削除を担う relay service を別アーキテクチャとして承認してから実装する。

## API サーフェス

### Review Inbox と検索

| 目的 | 推奨 API | 必要 permission / scope | 制約と実装メモ |
| --- | --- | --- | --- |
| review request 中の open PR を横断取得 | REST `GET /search/issues?q=is:pr+is:open+review-requested:@me` または GraphQL `search(type: ISSUE, query: ...)` | GitHub App/fine-grained PAT は対象 repository の `Pull requests: read`、classic/OAuth は private repository に `repo` | Search は最大 1,000 results、認証時 30 requests/min、query timeout では `incomplete_results=true`。複数 repository のうち権限がないものは黙って欠落し得る。team review は `team-review-requested:ORG/TEAM` を別 query にする。 |
| GitHub notification thread・未読 | REST `GET /notifications`、`GET /notifications/threads/{thread_id}`、read/unsubscribe 系 | **classic PAT のみ**。`notifications` または `repo` scope | `Last-Modified` による conditional polling が前提。GitHub App token と fine-grained PAT は非対応。採用認証では使わない。 |
| repository 単位の open PR | REST `GET /repos/{owner}/{repo}/pulls` | `Pull requests: read` | default 30、最大 100/page。Inbox の全体 discovery より、既知 repository の再同期・fallback に使う。 |
| requested reviewers | REST `GET /repos/{owner}/{repo}/pulls/{number}/requested_reviewers`、GraphQL `reviewRequests` | `Pull requests: read` | review を提出した reviewer は requested list から消えるため、`reviews` と併せて状態を算出する。 |

### PR 詳細、diff、会話、Checks

| 目的 | REST | GraphQL / 補足 | permission |
| --- | --- | --- | --- |
| PR metadata | `GET /repos/{owner}/{repo}/pulls/{number}` | `PullRequest` の title/body/state/isDraft/mergeable/reviewDecision/headRefOid/baseRefOid/author/labels/reviewRequests 等をまとめて取得可能 | `Pull requests: read` または endpoint により `Contents: read` |
| changed files | `GET /repos/{owner}/{repo}/pulls/{number}/files` | `PullRequest.files` connection | `Pull requests: read`。REST は最大 3,000 files、default 30/max 100 per page。上限到達は完全 diff と表示しない |
| raw diff/patch | PR GET に `Accept: application/vnd.github.diff` / `application/vnd.github.patch` | GraphQL は構造化 metadata 向け。diff 本体は REST media type または local Git source とする | PR read。large diff の取得失敗・省略を扱い、ADR-0006/0007 の source 分離・構造化 model に渡す |
| commits | `GET /repos/{owner}/{repo}/pulls/{number}/commits` | `PullRequest.commits` | `Pull requests: read` / `Contents: read`。head SHA を送信前に再検証する |
| issue-level conversation | `GET /repos/{owner}/{repo}/issues/{number}/comments` | `PullRequest.comments` / timeline | `Pull requests: read` または `Issues: read`、投稿にはどちらかの `write`。PR は issue でもあるため endpoint は issue API だが、PR permission だけでも利用可能 |
| reviews | `GET /repos/{owner}/{repo}/pulls/{number}/reviews` | `PullRequest.reviews` | `Pull requests: read` |
| inline comments/thread | `GET /repos/{owner}/{repo}/pulls/{number}/comments`、review ごとの comments | `reviewThreads { isResolved, isOutdated, comments }` は thread 状態集約に有用 | `Pull requests: read` |
| Checks | `GET /repos/{owner}/{repo}/commits/{ref}/check-runs`、check suites | `statusCheckRollup` | `Checks: read`。fork 側 push では checks endpoint の `pull_requests` が空になり得るため PR head SHA を明示する。read は OAuth/PAT でも可、Checks write は GitHub App のみだが本製品は read のみ |
| legacy commit statuses | `GET /repos/{owner}/{repo}/commits/{ref}/status` | `statusCheckRollup.contexts` | `Commit statuses: read`。Checks と legacy status を統合表示する場合だけ要求 |

GraphQL は Inbox row と PR overview を少ない round trip で組み立て、`reviewThreads`、`reviewDecision`、`statusCheckRollup` の関連を取得する用途に向く。REST は raw diff、endpoint ごとの conditional request、明確な permission header、review 送信に向く。GraphQL だけに統一しない。

### Review Inboxの分類契約

取得集合は、個人review requestの`review-requested:@me`、所属teamごとの`team-review-requested:ORG/TEAM`、およびユーザーがreview済みのopen PRを追跡する参加履歴queryから作る。検索結果はrepository access範囲と`incomplete_results`を保持し、欠落可能性をUIで明示する。

分類優先順位は次のとおりとする。

1. **Needs Review**: openかつ明示的な個人またはteam review requestが現在存在する。再review requestもここへ戻す。
2. **Updated Since Review**: 現在の明示requestはなく、ユーザーの最新submitted reviewが存在し、そのreviewの`commit_id`と現在の`head.sha`が異なる。
3. **Reviewed**: 現在の明示requestはなく、ユーザーの最新submitted reviewの`commit_id`と現在の`head.sha`が一致する。

draft、closed、mergedはactive Inboxから除外し、履歴またはfilterで扱う。requested reviewers、reviews、current head SHAの取得に失敗した場合は推測分類せず`partial`として表示する。GitHub Notificationsのread/unreadは取得も更新もしない。

### 下書きと送信

| 操作 | REST | GraphQL | permission / 注意 |
| --- | --- | --- | --- |
| pending review 作成 | `POST /repos/{owner}/{repo}/pulls/{number}/reviews` で `event` を省略 | `addPullRequestReview` | `Pull requests: write`。`commit_id` は表示した head SHA を明示し、更新済みなら送信前確認へ戻す |
| inline thread/comment | pending review の `comments`、`POST .../pulls/{number}/comments` | `addPullRequestReviewThread`、`addPullRequestReviewComment` | `Pull requests: write`。新規実装は deprecated な diff `position` より `line`/`side`/`start_line`/`start_side` を優先する |
| reply | review comment reply endpoint | `addPullRequestReviewThreadReply` | `Pull requests: write` |
| submit review | `POST .../reviews/{review_id}/events` | `submitPullRequestReview` | `APPROVE` / `REQUEST_CHANGES` / `COMMENT`。送信は通知を発生させ、secondary/content-creation limit 対象 |
| thread resolve | REST endpoint の対応状況を実装時に再確認 | `resolveReviewThread` / `unresolveReviewThread` | mutation 可否を runtime permission で検証。MVP のレビュー送信必須範囲とは分離可能 |
| issue-level comment | `POST /repos/{owner}/{repo}/issues/{number}/comments` | `addComment` | `Pull requests: write` または `Issues: write`。PR review と一般会話を UI 上で区別する |

ADR-0009 に従い AI は読み取り専用で、人間の明示操作だけが mutation を実行する。送信 mutation は「timeout 後に GitHub 側で成功したか不明」という状態があるため自動 retry しない。review ID、head SHA、server 上の pending/submitted state を再取得し、重複しないことを確認してからユーザーに再送を提示する。

## permission の最小集合

GitHub App の初期候補:

- Repository metadata: GitHub App の必須 read（追加要求なし）。
- `Pull requests: write`: PR/read、review/thread/read-write を包含するため。閲覧専用 mode を提供するなら `read` app registration と送信用 registration の分離が必要になり、単一 GitHub App の install permission は動的縮小できない。まず単一 app の write permission を明示説明し、local UI で送信を human gate にする案。
- `Checks: read`: check runs/suites 表示。
- PR の issue-level comments は `Pull requests` permission で read/write できるため、PR 専用なら `Issues` permission は要求しない。通常 issue も対象に広げる場合だけ `Issues: read/write` を追加する。
- `Contents: read`: GitHub blob/source 取得が必要な場合だけ。PR files/diff と local Git で足りる設計では不要になり得るため実装前に endpoint fixture で確認する。
- `Commit statuses: read`: legacy status を Checks と併合する場合だけ。
- Account permission（email 等）は要求しない。

REST の permission 不足は `403` と `X-Accepted-GitHub-Permissions`、classic/OAuth scope は `X-Accepted-OAuth-Scopes` / `X-OAuth-Scopes` を診断に使う。`404` は private resource の非存在だけでなく visibility/permission 不足もあり得るため、repository access 一覧と app installation を確認する導線を出す。GraphQL は HTTP 200 でも `errors` と partial `data` を返し得るため、HTTP status だけで成功判定しない。

## pagination、cache、rate limit

- REST collection は response の `Link` header を唯一の next-page source とし、URL を手組みしない。多くの endpoint は default 30、`per_page=100` が上限。画面は先頭 page を先に表示し、cancel 可能な incremental pagination にする。
- GraphQL connection は `first`/`last` 1–100 が必須。`pageInfo.endCursor`/`hasNextPage` で cursor pagination し、1 call 500,000 nodes 上限を避けるため Inbox row と詳細 query を分ける。
- authenticated user の REST core は通常 5,000 requests/hour、GraphQL は 5,000 points/hour。search は別枠で authenticated 30 requests/min（code search は 10/min）。必ず response の `X-RateLimit-*` を保存・表示し、`GET /rate_limit` の常時 polling はしない。
- secondary limit は REST 900 points/min、GraphQL endpoint 2,000 points/min、REST+GraphQL 合計 concurrent request 100、content creation は概ね 80/min・500/hour 等だが値は変更・非公開要因がある。製品側は global concurrency 4 程度、mutation は直列かつ 1 秒以上間隔を初期値とし、server headers を優先する。
- `ETag` / `Last-Modified` を account + method + normalized URL + Accept/API version 単位で cache し、`If-None-Match` / `If-Modified-Since` を送る。正しく認証した conditional GET の `304` は primary rate limit を消費しない。Notifications を classic PAT で選んだ場合は特に `Last-Modified` polling を使う。
- API version header（現在の GitHub Docs が示す `X-GitHub-Api-Version`）を client adapter に固定し、version 変更は fixture/compatibility test を伴う明示更新にする。

## webhook の扱い

GitHub App を採る場合に候補となる event は `pull_request`、`pull_request_review`、`pull_request_review_comment`、`issue_comment`、`check_run`、`check_suite`、`installation`、`installation_repositories`、`github_app_authorization`。minimum event のみ subscribe し、delivery ID による deduplicate、署名検証、時刻・再送、installation/repository removal、authorization revoke を処理する。

ただし desktop 単体に webhook secret や GitHub App private key を配布してはならず、public receiver も成立しない。relay を採る場合は server secret store、payload 最小化、retention/delete、account mapping、offline reconnect、運用費と breach response が新たな scope になる。MVPはconditional pollingとmanual refreshを採用し、relay導入時は別Decisionとする。

## 認証方式の比較

| 方式 | 最小権限/private repo | SSO・組織管理 | token lifecycle | 配布時 secret | 評価 |
| --- | --- | --- | --- | --- | --- |
| GitHub App user access token | fine-grained repository permissionsとinstall時の選択 repository。user と app 両方の権限の共通部分。private repo は app installation/approval が必要 | active SSO session で authorize/install。別端末・再 login では SSO session を作って再認証が必要な場合あり。organization owner/admin approval が障壁 | access token 既定8時間、refresh 6か月、refresh 使用時に旧 access/refresh は失効。authorization revoke webhookあり | **private key は絶対に配布不可**。native public client は client secret を安全に保持できない。authorization code + PKCE でも client secret は app識別子相当と扱うか broker が必要 | 最小権限・短命tokenを理由に採用。Notifications API非対応はreview request/search基準で扱い、install frictionをUIで説明する |
| OAuth App | scope は粗い。private PR を扱うと `repo` が広範囲。repository 単位制限なし | active SSO session で authorize。organization OAuth app policy の承認対象 | expiring token を opt-in 可能（8時間/refresh 6か月）。従来は long-lived | native public client に secret を隠せない。PKCE を使用。device flow は secret 不要だが phishing リスクのため browser 利用可能なら非推奨 | 現行 Notifications endpoint には使えず、private repository では broad `repo` が最小権限要件に弱い |
| fine-grained PAT | resource owner と選択 repository、permission read/write、expiration を手動設定。private repo 可だが organization approval/policyあり | token 作成時に SSO authorization。既定で organization owner approval が必要な場合あり | refresh なし。expiry/rotation/revoke は user 操作 | app secret 不要。token 自体を paste して OS store へ | developer preview/advanced fallback に有用。consumer desktop の主 UX には手作業・expiry対応が重い。Notifications 非対応 |
| classic PAT | `repo`/`notifications` 等の粗い scope。private repo access が広い | organization ごと `Configure SSO`。organization policy で禁止可能 | refresh なし。expiry推奨。1年未使用や leak 検出等で revoke | app secret 不要。token paste | Notifications 互換の唯一の現実的 fallback だが過剰権限。既定推奨にしない |

public native client では client secret を本当の secret と見なせない。選択肢は (a) embedded client secret + PKCE を認証 endpoint の互換要件として受け入れ、自社 backend の認可根拠には決して使わない、(b) token exchange/refresh broker を運用する、(c) device flow で secret を不要にする、の三つである。GitHub 公式 best practice は browser が使える public client では authorization code + PKCE を device flow より優先し、device flow は phishing のため headless/constrained environment に限定する。MVPは (a) を採用する。配布物内のclient secretは秘匿できずpublic clientの識別子に過ぎないため、自社backendや追加機能の認可根拠には決して使わない。将来brokerを導入する場合は別のアーキテクチャ判断とする。

### 認証フローとユーザー体験

1. GUIはaccount追加ごとにランダムな`state`とPKCE verifierをmemory内で生成し、`S256` challenge付きauthorize URLをsystem browserで開く。verifier、authorization code、tokenをURL log、process argument、clipboard、永続一時fileへ出さない。
2. 登録済みcallback handlerは`code`と`state`をGUIへ戻す。`state`不一致、重複callback、期限切れ、ユーザー拒否はtoken交換せず終了し、再試行可能な理由を表示する。callback URIの具体値はGitHub App登録とWindows実機検証で固定する。
3. GUI/Windows側がcodeとverifierをtokenへ交換し、Credential Managerへの保存とread-back確認が成功した後だけaccountを利用可能にする。browserを閉じた、callback待機をキャンセルした、または時間切れの場合は短命state/verifierを破棄する。
4. access tokenは期限前にaccount単位のsingle-flightでrefreshする。401時は未試行の場合に一度だけrefreshし、成功後のrequestだけを再試行する。refresh pairの交換後に保存できない場合はfail closedで再認証を要求する。
5. revoke、`github_app_authorization`、permissionまたはinstallation変更を検知した場合は対象accountの同期を止め、cacheをstale/unauthorizedとして保持し、再認証またはrepository access確認を案内する。
6. logoutはremote revokeを試行してからlocal credentialとaccount cacheを削除する。offlineでlocal削除だけを選ぶ場合はremote tokenが有効な可能性を明示する。account removeとGitHub App uninstallは別操作とする。

GitHub App private keyはdesktopへ含めない。GitHubのtoken endpoint互換のため配布物に含めるclient secretはpublic clientでは秘匿できない識別子として扱い、それ単体を信頼根拠にしない。

## Windows secret store と Rust 候補

MVPはWindows Credential ManagerのWindows native credential storeを採用する。Rustは`keyring-core` + Windows専用`windows-native-keyring-store`をadapterの背後に置く候補とし、成熟度・thread ordering・Windows実機behaviorを実装spikeで検証する。直接制御が必要ならMicrosoft `windows` crateからWin32 Credential Management APIを呼ぶ案を比較する。Credential Managerが利用不能な場合、MVPはDPAPI fileを含む別保存先へ自動fallbackせずfail closedとする。平文file、SQLite plaintext、registry plaintext、environment variable、command line argument、clipboard常用は採用しない。

secret store 内は account ごとに versioned credential envelope を一件として保存する候補:

- non-secret index: host、GitHub numeric account ID、login 表示名、auth kind、expiry、credential locator、schema version、last sync/error class。通常 DB に保存可能。
- secret envelope: access token、refresh token（存在時）、それぞれの expiry、token type、必要最小限の rotation metadata。refresh で新旧 token が同時失効するため一件の置換で commit する。
- key は mutable な login ではなく `github.com + numeric account ID + auth kind` を基礎にし、同一 account の token generation は上書きする。複数 host/GHES 対応を将来追加しても衝突しない。

操作契約:

1. Sign-in: browser state/PKCE verifier は短命 memory のみ。token response を secret wrapper に直接取り込み、Credential Manager 書込成功後だけ account を利用可能にする。失敗時に plaintext fallback しない。
2. Refresh: expiry の数分前または 401 後一回だけ single-flight refresh。新しい pair を一件置換して read-back/parse 確認し、その後 memory の旧値を破棄する。交換成功後の local write 失敗は旧 refresh token が既に無効なので `再ログインが必要` とする。
3. Revoke/sign-out: online なら対応する GitHub revoke/authorization endpoint を先に試し、成功または既に無効を確認して local secret と cache を削除する。offline 時は `この端末からのみ削除（remote token は有効な可能性）` を明示選択させる。account remove と app uninstall を混同しない。
4. Migration: version N を read → memory で変換 → new key/version を write → read-back 検証 → old entry delete。失敗時は old を保持し、平文中間 file を作らない。
5. Delete: account secret、ETag/body cache、draft review、diagnostic metadata を列挙して削除し、失敗した item をユーザーに示す。uninstall cleanup も同じ列挙を使う。

### `CredentialStore`契約とOS backend

application層はOS APIを直接参照せず、account credentialを`put`、`get`、`delete`、`replace`できる共通`CredentialStore`契約に依存する。結果は`not_found`、`unavailable`、`access_denied`、`corrupt`、`write_failed`を区別し、secret値をerrorや`Debug`へ含めない。`replace`はaccess/refresh pairを一単位として扱い、成功後のread-back検証を必須にする。テストdoubleはmemory内だけで動作し、製品buildのfallback backendとして登録しない。

MVP backendはWindows Credential Managerだけとする。将来のmacOS nativeではKeychain、Linux nativeではOS secret service等のbackendを同じ契約へ追加できるが、利用不能時はいずれも平文fallbackせずfail closedとする。

### Windowsと実行backendの信頼境界

- WindowsネイティブGUIがGitHub API client、認証callback、token refresh、CredentialStore、redactionを所有する。GitHub API操作は原則Windows側で実行する。
- agent、terminal、git、worktree、workspace setupはWindows native backendを維持し、Worktreeごとに選択したWSL distroを`wsl.exe -d <distro> -- ...`相当で起動できる。backend選択、path変換、終了code、stdout/stderr、cancelはcommand execution境界で扱う。
- 実行backendへ渡すのはrepository path、commit SHA、diffやAPI結果など必要最小限の非secretデータとする。access token、refresh token、authorization code、PKCE verifierをWSL filesystem、WSL credential helper、環境変数、stdin、process argumentへ恒久または暗黙にコピーしない。
- WSL内のツールがGitHub credentialを要求する場合は認証済みとして偽装せず、unsupportedまたは再設計が必要な操作として明示する。Windows側API失敗とWSL command失敗は別error domainとして表示する。

### 検証観点

- 認証: `state`不一致、callback重複、cancel、timeout、browser拒否、code交換失敗でtokenが保存されず、再試行可能である。
- lifecycle: expiry前refreshのsingle-flight、401後一回だけのrefresh、rotation時のpair置換、revoke/permission変更/logoutでaccountが正しく停止または削除される。
- secret: log、診断bundle、panic/error、HTTP trace、process list、Windows/WSLのenvironmentとfilesystemにtokenやcode/verifierが現れない。
- store: Windows Credential Managerのput/get/replace/delete、read-back、access denied/corruption/unavailableでfail closedとなり、別保存先が作られない。
- backend: Windows nativeと選択WSL distroの両方で非secret commandを実行でき、WSL未導入、distro不存在、path変換、cancel、終了codeをGitHub認証errorと混同しない。
- API/Inbox: pagination、rate limit、GraphQL partial data、Search incomplete、repository permission欠落でstale/partial表示となり、不完全データから分類を推測しない。

memory/log/crash 対策:

- Rust の token type は `secrecy::SecretString` 等で `Debug`/`Display`/serde serialize を既定禁止し、drop 時 zeroize を best-effort で行う。zeroize は swap、allocator copy、OS crash dump まで保証しないことを明記する。
- `Authorization`、refresh token、OAuth code、device code、webhook secret を structured log field、URL query、panic text、error chain、telemetry、analytics、screenshot、clipboard に入れない。HTTP header は sensitive flag を立て、request/response dump middleware は header/body allowlist にする。
- `#[instrument(skip(...))]`、custom `Debug` redaction、token prefix すら通常 log に出さない。診断には GitHub request ID、status、endpoint template、rate headers、account の non-secret ID だけを残す。
- crash report は memory dump を既定収集せず、添付前 preview/redaction を行う。refresh response body と secret-store errors の生 bytes を保持しない。

## error・offline・partial data UX

| 状態 | 判定 | UX | retry |
| --- | --- | --- | --- |
| 401 / bad credentials | access token expiry/revoke、refresh failure | account 単位で `再認証が必要`。他 account と cached data は利用継続。refresh 可能なら single-flight で一回だけ試す | refresh 一回。失敗後の API loop は止める |
| 403 permission/SSO | `X-Accepted-GitHub-Permissions`、`X-GitHub-SSO`、app installation/policy | 不足 permission、SSO 再認証、organization approval、repository installation を区別した action を表示 | permission が変わるまで自動 retry しない |
| 404 | resource 削除または visibility/permission camouflage | `存在しないかアクセスできない`。repository access/install を確認する導線。cache は削除せず stale 表示 | manual refresh |
| 422 | stale diff line、validation、spam/content limit | field/path/head SHA と server message を redaction 後に表示し draft を保持 | 自動 retry しない。最新 diff に rebase/mapping 後に再送 |
| primary rate limit | 403/429 + remaining=0 | stale cache を read-only 表示し reset 時刻と残時間を示す。background refresh を停止 | `X-RateLimit-Reset` まで待つ |
| secondary limit | 403/429、`Retry-After` または message | account/global activity を減速し、送信 button を cooldown。繰返し時は明示 error | `Retry-After`、次に1分、以後 exponential backoff + full jitter、上限回数で停止 |
| 5xx / 502 / 504 | GitHub/server timeout | cached/partial data を保持し `GitHub 一時障害`。status page linkを提供可能 | GET/GraphQL query は 1s, 2s, 4s 程度 + jitter、最大3回。mutation は状態確認まで自動 retry なし |
| network/offline/TLS/DNS | transport error | offline banner、最終同期時刻、cache read-only。draft は local 保存。送信は queue せず disabled | connectivity 回復/manual refresh。短い GET のみ bounded retry |
| GraphQL partial data | HTTP 200 + `data` と `errors` | 成功 field を表示し、欠落 section に `取得不完全` と原因。完全取得を装わない | error path ごとに小さい query へ分割し bounded retry |
| Search incomplete/権限欠落 | `incomplete_results=true` または複数 repo の silent omission | Inbox に `検索結果が不完全` badge。対象 repository/access 範囲と再検索 action | query 分割（日付/repository）後に再試行、無限 loop 禁止 |
| files 3,000 上限/large diff | page 上限・diff response error | `変更ファイルの一部のみ` と明示し、local Git source への切替を提案 | 同じ REST call の反復はしない |

cache record は `fresh / stale / partial / unauthorized / deleted` を区別し、画面全体を一つの success/failure に潰さない。account ごとの request cancellation と generation ID を持ち、古い response が新しい refresh 結果を上書きしないようにする。

## 承認済み判断と実装時の確認事項

承認済み: Review Inboxはreview request/search基準、認証はGitHub App user access token + system browser + authorization code flow with PKCE、MVPの保存先はWindows Credential Manager、GitHub認証とsecret lifecycleはGUI/Windows側が所有しWSLへtokenを恒久コピーしない。実行backendはWindows nativeを維持しつつ、選択WSL distroを`wsl.exe`経由で利用可能にする。将来のOS backendも平文fallbackせずfail closedとする。

実装時には、最小permissionに`Contents: read`と`Commit statuses: read`を含める必要があるかをendpoint fixtureで削減確認する。callback URIの具体値、GitHub App登録運用、採用crateはWindows実機検証後に固定する。webhook relayはMVP外としconditional pollingを使い、導入時は別Decisionとする。

## 一次情報・候補実装

- [REST pull requests](https://docs.github.com/en/rest/pulls/pulls)
- [REST pull request reviews](https://docs.github.com/en/rest/pulls/reviews)
- [REST review comments](https://docs.github.com/en/rest/pulls/comments)
- [REST notifications](https://docs.github.com/en/rest/activity/notifications)
- [REST search](https://docs.github.com/en/rest/search/search)
- [REST checks](https://docs.github.com/en/rest/checks/runs)
- [GraphQL pull requests](https://docs.github.com/en/graphql/reference/pulls)
- [GraphQL pagination](https://docs.github.com/en/graphql/guides/using-pagination-in-the-graphql-api)
- [GraphQL rate/query limits](https://docs.github.com/en/graphql/overview/rate-limits-and-query-limits-for-the-graphql-api)
- [REST pagination](https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api)
- [REST rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)
- [REST best practices / conditional requests](https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api)
- [GitHub App と OAuth App の比較](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/differences-between-github-apps-and-oauth-apps)
- [GitHub App best practices](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/best-practices-for-creating-a-github-app)
- [GitHub App user access token](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
- [GitHub App token refresh](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/refreshing-user-access-tokens)
- [OAuth App best practices](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/best-practices-for-creating-an-oauth-app)
- [PAT 管理](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
- [token expiration/revocation](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/token-expiration-and-revocation)
- [SSO app authorization](https://docs.github.com/en/enterprise-cloud@latest/authentication/authenticating-with-single-sign-on/authorizing-an-app-for-single-sign-on)
- [PAT SSO authorization](https://docs.github.com/en/enterprise-cloud@latest/authentication/authenticating-with-single-sign-on/authorizing-a-personal-access-token-for-use-with-single-sign-on)
- [Microsoft DPAPI CryptProtectData](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata)
- [keyring-rs](https://github.com/open-source-cooperative/keyring-rs)
- [windows-native-keyring-store](https://docs.rs/windows-native-keyring-store/latest/windows_native_keyring_store/)
- [secrecy](https://docs.rs/secrecy/latest/secrecy/)
