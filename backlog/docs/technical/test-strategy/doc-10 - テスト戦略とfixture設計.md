---
id: doc-10
title: テスト戦略とfixture設計
type: specification
created_date: '2026-08-30 20:15'
updated_date: '2026-08-30 20:16'
---
# テスト戦略とfixture設計

## 目的と設計原則

Review SweeperのGitHub連携、構造化差分、worktree観測、`ExecutionBackend`、永続化、非同期UIを、外部サービスがなくても決定的に検証できる境界を定める。既存のADR-0002/0006/0007/0010/0011/0012とTASK-1/2/4/5/7の契約をtest oracleとし、新しい製品APIや実装frameworkは確定しない。

- pureなdomain規則を最も厚くし、adapter contract、hermetic integration、Windows native system testの順に環境依存を増やす。
- mockが証明できるのはアプリ側のrequest、response解釈、状態遷移までである。GitHub、git、ConPTY、Credential Manager、GPUI、IME、UI Automationの実挙動をmock成功で代替しない。
- 時刻、乱数、retry、scheduler、filesystem、process、HTTPをport化し、testではfake clock、seed固定random、scripted transport、一時directoryを注入する。
- fixtureは非secret、versioned、最小、由来と期待結果が明示されたものにする。recordしたresponseを無審査で正本にせず、redactionとschema validationを通す。
- failureは「画面全体の失敗」に潰さず、fresh/stale/partial/unauthorized/deleted、available/unavailable、mappable/unmappableなどdomain状態でassertする。

## テスト層と責務

| 層 | 主対象とoracle | 実行環境・頻度 | 代替しないもの / 失敗時の切り分け |
| --- | --- | --- | --- |
| unit | parser、座標、value object、state reducer、retry分類、generation gate、backend ID一致、digest | 全OS、各commit、millisecond | I/Oは含めない。失敗はalgorithm/domain invariant |
| domain scenario | PR snapshotとlocal source分離、draft lifecycle、force-push、worktree state、backend binding、cancel競合 | 全OS、各commit、fake ports | adapter serializationは別contract test。失敗はuse-case/state transition |
| API contract | GitHub REST/GraphQL request、pagination、partial data、rate header、review/Checks payload、redaction | Linux通常CI、各commit、fixture/scripted HTTP | live GitHub semanticsはnightly/manual。失敗はclient mappingまたはfixture drift |
| fixture contract | envelope schema、version、必須case、digest、secret scan、期待state | Linux通常CI、各commit | product schemaではない。失敗はfixture自体またはmigration不足 |
| hermetic integration | temp git repository、SQLite/file cache、mock HTTP server、host-native process、application service結合 | Linux通常CI、各commit | Windows固有API、WSL実体は代替しない。失敗はadapter間契約 |
| Windows native integration | MSVC build、Win32 path、Credential Manager sandbox、Job Object、ConPTY、GPUI起動 | managed Windows runner、PR必須範囲 + scheduled拡張 | desktop interactionは次層。失敗はWindows adapter/runtime |
| Windows native UI/E2E | real GPUI window、keyboard/focus、selection/copy、font fallback、Microsoft IME、UIA/Narrator、terminal cell/cursor | interactive Windows test host、release candidate/manual + 安定部分のみscheduled | screenshotだけでsemantic/a11y/inputを代替しない。artifactにOS/framework/GPU条件を添付 |

unit/domain/API/fixture/hermetic integrationはnetwork、GitHub account/token、WSL、Windows desktopを要求しない。Windows専用codeはportのcontract testを通常CIで通したうえで、実adapterをWindows laneで検証する。

## 共通fixture envelope

fixture setは次のlogical fieldを持つ。物理形式は現prototypeではJSONだが、製品の公開schemaではない。

```text
FixtureEnvelope
  schema_version
  fixture_set
  source: synthetic | sanitized_capture | generated_repository
  input
  expected_domain_state
  expected_effects
  nondeterminism: { clock, random_seed, scheduler }
  provenance: { contract, captured_at?, redaction_review? }
```

規則:

1. unknown `schema_version`は黙って読まずfixture contract testを失敗させる。
2. SHA、repository ID、account ID、backend ID、generation IDを固定値で明示し、現在時刻やhost pathを期待値に混ぜない。
3. token、Authorization、cookie、client secret、実account、private repository contentを格納しない。sanitized captureはheader/body allowlist、secret scan、人手確認を行う。
4. successだけでなくpartial、unavailable、corrupt、unknown outcomeを同じfixture familyに置く。
5. golden updateは専用commandで生成し、reviewでsemantic差分を確認する。test失敗時の自動上書きを禁止する。
6. fixtureの期待結果はUI screenshotだけにせず、domain state/effect/event列を正本にする。

`spikes/test-strategy/fixtures/manifest.json`はschema version 1の代表fixture catalogueであり、`verify.sh`が分類漏れ、secretらしいfield、worktree実状態を検証する。

## GitHub差分・review・Checks fixture

### 差分

TASK-4のfixtureを再利用し、GitHub review snapshotとlocal comparisonを別sourceとして生成する。modified/add/delete/rename/copy/type-change、binary、patch欠落、no-final-newline、large/truncated、3,000 files上限、whitespace、two-dot/three-dotを含める。

oracleは次の通り。

- GitHub sourceだけが`github_anchor`を持てる。local/whitespace表示から送信座標を生成しない。
- contextはold/new、deletionはold、additionはnew lineを持ち、markerはlineを持たない。
- patch欠落、head mismatch、parse errorは近傍行へfallbackせず`Unmappable(reason)`または`Partial(reason)`になる。
- rename前後path、base/head SHA、snapshot sourceをparse後も保持する。

### review lifecycle

local draft、pending create、comment追加、submit、pending delete、outdated、422、timeout後outcome unknownをfixture化する。mutation timeoutは同じPOSTを即retryせず、remote review/commentをGETしてcorrelation ID、head SHA、remote IDでreconcileする期待effectをassertする。submitted reviewをcancel処理が削除しないこと、force-push後のdraftが`NeedsRemap`になることも検証する。

### Checks

check suite/runについてqueued、in_progress、success、failure、cancelled、timed_out、neutral、skipped、action_required、unknown enum、同名runの再実行、古いhead SHA、pagination、partial responseを含める。集約表示は個別stateを失わず、古いheadの成功をcurrent headの成功として扱わず、unknown stateをsuccessへfallbackしない。

live GitHub testにだけ残す項目は、GitHubの実validation境界、secondary rate limit、pending review association、multi-line LEFT/RIGHT、large diffの実上限である。専用test repositoryと最小権限accountを使い、通常PRのblocking gateにはしない。契約差分を検知した場合はsanitized fixtureへ反映するPRを作る。

## worktree fixture

generated temp repositoryで次を作り、`git status --porcelain=v2`、HEAD、index、merge stateを構造化adapterへ入力する。

| 状態 | 生成方法 | 期待state |
| --- | --- | --- |
| clean | commit直後 | `Clean` |
| modified | tracked fileを変更 | `Modified` |
| staged |変更をindexへ追加 | `Staged` |
| untracked | 未追跡fileを追加 | `Untracked` |
| mixed | staged/unstaged/untrackedを併存 | 各成分を保持し単一dirtyへ潰さない |
| conflicted | 競合branchをmerge | `Conflicted`、破壊的自動修復なし |
| head-diverged | observed remote refとahead/behind | `HeadDiverged`、PR snapshotとは別state |
| broken | invalid HEAD、missing gitdir、permission/I/O error | `Broken(reason)`、cleanへfallbackしない |

pathにはspace、日本語、non-UTF-8（対応OSのみ）、long path、symlink/junction境界を含める。watch eventはburst、rename、delete/recreate、overflowをscripted clockで流し、debounce後の再scanが最終状態へ収束することをassertする。testは一時directoryだけを変更し、ユーザーrepositoryを対象にしない。

## ExecutionBackend fixture

`backend_id`、kind、instance（WSL distro等）、backend-native path、command、argv、environment delta、scripted stdout/stderr chunk、terminationをfixtureに持つ。secret環境変数はfixtureに入れない。

- Windows native success/nonzero/launch error/cancel、UTF-8でないchunk、stdout/stderr順序、child process cleanup。
- WSL selected distro、未導入、distro不存在、停止中、path変換不能、`wsl.exe` nonzero。
- unsupported hostでは`Unavailable`を返し、別backendへ暗黙fallbackしない。
- worktree bindingとrequest/path/resultの`backend_id`がすべて一致するまでeffectを許可しない。
- Windows pathとWSL path、environment、git stateを同一requestへ混在させない。mismatchは起動前に検出する。

通常CIはTASK-7のhost-native harnessとfake backendで共通contractを検証する。Windows runnerはJob Object/process tree、path quoting、non-Unicode byte boundaryを、WSL laneは導入済み専用runnerでdistro identity、path、cancelを検証する。WSL laneはMVP通常CIの必須条件にせず、WSL backend実装タスクで導入時にgateを再評価する。

## 非同期・cancel・retry・corruption matrix

| fault injection | oracle |
| --- | --- |
| generation Nの後にN-1が完了 | N-1の結果/effectを破棄しNを維持 |
| cancelとsuccessが同tick | 一度確定したterminal stateだけをpublishし二重effectなし |
| cancel後に次generation開始 | 新generationだけacceptし旧completionを無視 |
| UI/window drop後にcompletion | callbackが破棄済みUIを参照せずresource cleanup |
| query 5xx/timeout | fake clockで1/2/4秒相当 + seeded jitter、上限後停止 |
| mutation timeout | outcome unknown、照合前の自動retryなし |
| `Retry-After` / rate reset | server hint優先、clock前進までrequestなし |
| GraphQL data + errors | 成功fieldとpartial reasonを併存 |
| truncated JSON / unknown enum | parse error/unknownを明示しlast-known-goodを保持 |
| cache digest mismatch / missing blob | `Corrupt`/`Missing`として再取得候補、structured stateを捏造しない |
| SQLite migration中断 / orphan blob | transaction rollbackまたはrepair report、secret fallbackなし |
| backend ID mismatch | process/filesystem effectの前にreject |

scheduler interleavingは少なくとも「refresh→cancel→late response」「old refresh→new refresh→old completion」「mutation timeout→reconcile→already applied」「watch burst→scan中に次event」を固定sequenceで検証する。property testを追加する場合もseedを失敗artifactへ記録し再現可能にする。

`spikes/test-strategy/src/lib.rs`はgeneration/cancel、bounded query retry、mutation reconciliation前retry禁止、digest破損、backend identity混在拒否の最小oracleをunit testする。具体的なproduction schedulerやretry crateの選定は後続実装タスクに残す。

## Windows native UI test matrix

Windows testはbuild/run smokeと実desktop interactionを分離する。

1. managed runner: MSVC locked build、unit/contract、Win32 adapter、process/ConPTY smoke。headless成功は描画・IME・a11y成功を意味しない。
2. interactive host: GPUI実windowでfocus traversal、keyboard shortcut、pointer、100,000行virtual diff、selection/copy、background更新/cancelを検証する。
3. text/input: 日本語、非BMP、combining、emoji/ZWJ、RTLを用途別system fontで表示し、Microsoft IMEのpreedit/候補位置/commitとclipboardを確認する。
4. accessibility: Accessibility Insights/UIA/Narratorでrole/name/value/focus/selection/actionを確認する。pixel screenshotだけをoracleにしない。
5. terminal: ConPTYへ固定VT corpusを流し、wide/ambiguous/combining/emoji cell、cursor、selection/copy、resize、10,000行scrollback、cancel/process cleanupを確認する。

artifactにはWindows build、Rust/GPUI version、GPU/driver、DPI、locale、font解決結果、release/debug、seed、event log、UIA tree、screenshotを含める。screenshot比較はDPI/font/GPU差に弱いため領域toleranceを限定し、domain snapshotとsemantic UIA assertionを主oracleにする。IME/Narratorの完全自動化が不安定な項目はrelease checklistとして明示し、flakyな無限retryでgreenにしない。

## CI laneと品質ゲート

| lane | trigger | 内容 | gate |
| --- | --- | --- | --- |
| `hermetic` | 全PR | fmt、lint、unit/domain、fixture contract、temp git、mock HTTP、host-native backend | 必須 |
| `windows-native` | 全PRまたはWindows関連path | MSVC build/test、Win32/Job Object/path、ConPTY smoke | Windows基盤導入後に必須 |
| `windows-ui` | scheduled/release、安定caseはPR | real GPUI/UIA/input/performance | release blocking、PR範囲は段階導入 |
| `github-live` | scheduled/manual | 専用repoでAPI drift/validation/rate behavior | 通常PR非blocking、driftはtriage必須 |
| `wsl` | WSL backend変更時/scheduled | 複数distroのidentity/path/process/cancel | WSL実装導入時に必須範囲を決定 |

通常CIではnetwork accessを前提にせず、dependency download後のtest自体はofflineで完結させる。各testは独立temp directory、固定locale/timezone/clock/seedを使い、順序依存と共有GitHub stateを避ける。flaky retryは最大1回を診断目的に限定し、初回失敗をartifactとmetricへ残す。quarantineはowner、理由、期限、関連taskを必須とし、必須gateから黙って除外しない。

## prototypeの再現手順

```bash
cargo fmt --manifest-path spikes/test-strategy/Cargo.toml --check
cargo clippy --manifest-path spikes/test-strategy/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path spikes/test-strategy/Cargo.toml
bash spikes/test-strategy/verify.sh
```

このprototypeはnetwork、GitHub credential、WSL、Windows desktopを使用しない。`verify.sh`は一時git repositoryでclean/modified/staged/untracked/conflicted/head-diverged/brokenを生成し、終了時に一時directoryを削除する。

## 後続実装への適用

- TASK-10/11はcrate構成とCI workflowへ本境界を配置するが、lane名やprovider固有設定はそのタスクで確定する。
- GitHub client、diff、worktree、backend、storage、UIの各実装タスクは、自層のfixtureを共通envelope規則へ追加する。
- live test用repository/account、Windows runner調達、WSL distro matrix、performance budgetの具体値は運用・architecture判断を伴うため、このタスクでは確定しない。
- fixture schemaをproduct persistence/API schemaとして再利用しない。両者のcompatibility policyはそれぞれの実装タスクで決める。
