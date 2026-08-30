---
id: doc-7
title: 差分・GitHubレビューAPI技術調査
type: specification
created_date: '2026-08-29 03:09'
updated_date: '2026-08-29 03:50'
tags:
  - github
  - diff
  - review-api
  - spike
---
# 差分・GitHubレビューAPI技術調査

## 目的と結論

Review Sweeperで、GitHub上のPull Request差分、local repositoryで再計算する差分、表示座標、review comment送信座標を混同せず扱うための技術契約を整理する。

ユーザー承認に基づき、ADR-0006/0007を`Accepted`として次のarchitecture契約を確定する。

- GitHub review用の差分snapshotとlocal comparisonを別sourceとして保持する。
- 両sourceを共通の`DiffFile` / `DiffHunk` / `DiffLine`へparseするが、`source`、比較revision・commit、GitHubのreview対象`head_sha`とcomment座標を失わない。
- review comment送信座標はGitHub sourceから得た`path + line + side + commit_id`だけから生成する。local差分やwhitespace除外表示から推測・逆変換して送信しない。
- unified/splitは同じ構造化行の表示projectionとする。whitespace除外は別comparisonであり、review座標を上書きしない。
- 前回reviewとの差分は`last_submitted_review.commit_id..current_head_sha`という別comparisonとし、PR全体のthree-dot diffやoutdated commentの位置とは別に表示する。

## GitHub PR差分とlocal git diff

GitHubのPull Requestはthree-dot comparisonを使い、base/headのmerge baseとhead tipを比較する。localでの基本形は`git diff <base>...<head>`だが、GitHubのPull Request pageとcompare pageはmerge baseの選び方が異なる場合があるため、local再計算結果をGitHub review座標の正本にはしない。

| 目的 | 取得経路 | 特性・制約 |
| --- | --- | --- |
| GitHub PR unified diff | `GET /repos/{owner}/{repo}/pulls/{pull_number}`、`Accept: application/vnd.github.diff` | review対象snapshotとしてparseする。欠落・上限・head更新を検知したら送信不可にする。 |
| GitHub file metadata/patch | `GET /repos/{owner}/{repo}/pulls/{pull_number}/files` | `filename`、`previous_filename`、status、任意の`patch`。最大3,000 files。`patch`欠落だけからbinary/truncated/too-largeの理由を断定しない。 |
| local PR相当 | `git diff --find-renames <base>...<head>` | local objectと現在のmerge baseに依存し、GitHub PR pageとの完全一致を保証しない。 |
| 任意commit間 | `git diff <old>..<new>` | 前回reviewからlatest headなど、明示した2 endpointの比較。 |
| working tree/index | `git diff`、`git diff --cached` | local専用source。GitHub review comment座標には使わない。 |
| whitespace除外 | `git diff -w ...`等 | hunkと行対応が変わり得る別comparison。GitHub座標へ逆変換しない。 |

Git unified formatはold/new path、`@@ -old_start,old_count +new_start,new_count @@`、context (` `)、deletion (`-`)、addition (`+`)からなる。rename/copyはextended header、binaryはtext hunk以外になり得る。`\ No newline at end of file`はcontent行ではない。

`spikes/diff-review-api/verify.sh`は一時repositoryでmodified/add/delete/rename/binary/no-final-newline、old/new line計数、whitespace差、two-dot/three-dot差を再現する。GitHub JSONは公式response schemaを固定したcontract fixtureであり、live integration testではない。

## 構造化差分モデル

```text
DiffSnapshot
  snapshot_id
  source: GitHubReview | LocalCommitRange | LocalIndex | LocalWorkingTree
  repository_id
  comparison:
    GitHubReview { pull_number, base_sha, head_sha, fetched_at, etag? }
    CommitRange { old_sha, new_sha, merge_base_sha?, options }
    LocalState { head_sha, index_tree?, worktree_fingerprint, options }
  completeness: Complete | Partial(reason) | Unavailable(reason)
  files: [DiffFile]

DiffFile
  old_path?: RepoPath
  new_path?: RepoPath
  status: Added | Modified | Deleted | Renamed | Copied | TypeChanged | Unmerged
  old_blob_oid?: Oid
  new_blob_oid?: Oid
  is_binary
  patch_availability
  hunks: [DiffHunk]

DiffHunk
  old_start, old_count, new_start, new_count
  header_text
  lines: [DiffLine]

DiffLine
  kind: Context | Addition | Deletion | NoNewlineMarker
  raw_text
  old_line?: PositiveInt
  new_line?: PositiveInt
  github_anchor?: { line: PositiveInt, side: LEFT | RIGHT }
```

不変条件:

- contextは`old_line`と`new_line`の両方、deletionは`old_line`だけ、additionは`new_line`だけを持つ。
- `NoNewlineMarker`はold/new lineを持たない。
- GitHub `side=LEFT`はdeletion側、`side=RIGHT`はadditionまたはcontext側。
- file-level commentは`subject_type=file`を使い、`line`/`side`を送らない。
- `github_anchor`は`source=GitHubReview`かつsnapshot headがcurrent PR headと一致し、GitHub patchで解決できる場合だけ生成する。
- rename前後のpathを保持し、local rename推測で送信pathを置き換えない。
- raw patch欠落、partial response、parse error、head mismatch、diff外の行は`Unmappable(reason)`とし、近傍行へfallbackしない。

```text
ReviewDraftComment
  draft_id
  snapshot_id
  expected_head_sha
  path
  subject: File | Line { line, side } | Range { start_line, start_side, line, side }
  body
  state: Local | PendingRemote(review_id, comment_id?) | Submitted | NeedsRemap(reason)
```

送信直前にcurrent head SHAを再取得し、`expected_head_sha`と一致しないdraftは`NeedsRemap(HeadChanged)`として、GitHub diff上でユーザーが再確認するまで送信しない。

## unified / side-by-side / whitespace表示

- unifiedは`DiffLine`順を1列へ投影し、old/new gutterを併記する。
- side-by-sideはdeletion/addition runを左右に配置する。空セルは表示用で、実在行や行番号を作らない。
- comment/finding/read-stateは`(snapshot_id, file, line identity)`へ結び、画面row indexには結び付けない。
- mode切替は同じ構造化行から表示rowだけを再計算する。
- whitespace除外は`options.whitespace=IgnoreAll`を持つ別local snapshotとして計算する。そこからGitHub座標を推測・逆変換せず、新規inline commentはcanonical GitHub snapshot上で再選択した行だけ送信できる。

## 前回reviewとの差分とoutdated comment

「前回review以後」はcurrent userの最新submitted reviewの`commit_id`を起点、current `head_sha`を終点にするtwo-endpoint comparisonである。PENDINGやdismissedの扱いは明示し、PR canonical diffを置き換えない。commit objectを取得できなければ「比較不能」とし、force-push後の履歴関係を推測しない。

review comment responseの`commit_id` / `original_commit_id`、`line` / `original_line`、`position` / `original_position`を保持する。current位置がなくoriginalだけ残るcommentはoutdatedとし、current lineへ自動再配置しない。

## review comment位置

deprecatedの`position`は新規commentに使わない。

| 対象 | request |
| --- | --- |
| addition/context | `path`, `line=<new_line>`, `side=RIGHT`, `commit_id=<expected head>` |
| deletion | `path`, `line=<old_line>`, `side=LEFT`, `commit_id=<expected head>` |
| 複数行 | 終端`line`,`side`と`start_line`,`start_side` |
| file全体 | `path`, `subject_type=file`, `commit_id`; `line`/`side`なし |
| thread reply | `in_reply_to`; 位置parameterなし |

rangeの許可組合せなど422になり得る境界はlive API testで固定し、別行への自動fallbackはしない。

## pending review lifecycle

1. 送信直前にPR head SHAを再確認する。
2. `POST /pulls/{pull_number}/reviews`へ`commit_id`とcommentsを送り、`event`を省略して`PENDING` reviewを作る。
3. 可能ならcreate requestへcommentsをまとめる。追加commentとpending reviewのassociationはlive testで確認する。
4. 明示的ユーザー操作で`POST /pulls/{pull_number}/reviews/{review_id}/events`へ`event=APPROVE | REQUEST_CHANGES | COMMENT`を送りsubmitする。
5. cancelはPENDINGに限り`DELETE /pulls/{pull_number}/reviews/{review_id}`で破棄する。

MVPは編集をlocal draftとして保持し、明示的送信操作内でPENDING作成からsubmitまで進める。POST後の通信断ではpending review一覧を取得してreconcileする。

## error・retry契約

mutating requestは一律自動retryしない。

| 状況 | 動作 |
| --- | --- |
| 401 | token refresh/re-authへ渡し、secretをlogに出さない。 |
| 403 | permission、review rule、rate limitをresponse/headerで分類する。rate limit以外は再送しない。 |
| 404 | resource消失とprivate resourceの認可不足を再確認し、無限pollしない。 |
| 409相当/head変更 | headを再取得し`NeedsRemap`へ。古い座標で再送しない。 |
| 422 | line/side/path/range/event/body、pending state等を表示し、自動fallbackしない。 |
| 429/rate-limit 403 | `Retry-After`、`X-RateLimit-*`を優先する。mutationはreconcile後だけ再実行する。 |
| 5xx/timeout | outcome unknownとしてreview/comments/stateをGET照合する。照合不能なら「送信結果不明」とする。 |
| partial failure | local draft、expected head、remote IDs、完了stepを保持し、pending deleteか再開を選べる。submitted reviewは自動削除しない。 |

mutationはserial化し、診断logはcorrelation ID、endpoint class、status、GitHub request ID、state transitionへ制限する。

## fixtureと残るlive検証

`spikes/diff-review-api/verify.sh`で、rename/binary/no-final-newline/line計数、two-dot/three-dot、whitespace、optional `patch`、review payloadのline/file不変条件、pending create/submit/deleteを検証する。

live integration testに残す項目:

- large diff、3,000 files境界、binary、`.gitattributes`による欠落
- pending reviewへの追加comment associationと複数pending validation
- multi-line rangeのLEFT/RIGHT境界、rename/delete/outdated headの422 body
- timeout reconcileとsecondary rate limit header

## 参照した一次情報

- [GitHub Docs: Branches / three-dot and two-dot comparisons](https://docs.github.com/en/pull-requests/reference/branches)
- [GitHub Docs: REST API endpoints for pull requests](https://docs.github.com/en/rest/pulls/pulls)
- [GitHub Docs: REST API endpoints for pull request review comments](https://docs.github.com/en/rest/pulls/comments)
- [GitHub Docs: REST API endpoints for pull request reviews](https://docs.github.com/en/rest/pulls/reviews)
- [GitHub Docs: GraphQL pull request types](https://docs.github.com/en/graphql/reference/pulls)
- [GitHub Docs: Best practices for using the REST API](https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api)
- [GitHub Docs: Rate limits for the REST API](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)
- [Git: git-diff Documentation](https://git-scm.com/docs/git-diff)
- [Git: diff format](https://git-scm.com/docs/diff-format)

## architecture判断と後続タスク境界

承認済みADR-0006/0007に従い、GitHub review snapshotとlocal comparisonは別sourceのまま共通モデルへparseし、source、revision・commit identity、GitHub座標を保持する。local差分やwhitespace除外表示からGitHub座標を推測して送信しない。

local修正をGitHub Suggested Changesへ変換する機能はmain上のTASK-29へ分離されている。TASK-4は`LocalWorkingTree`等を表現できるmodel boundaryと、local sourceには送信用`github_anchor`を付与しないという調査契約までとし、suggestion生成・GitHub送信・UI実装は行わない。
