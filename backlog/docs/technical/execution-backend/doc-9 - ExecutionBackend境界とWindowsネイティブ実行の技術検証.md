---
id: doc-9
title: ExecutionBackend境界とWindowsネイティブ実行の技術検証
type: specification
created_date: '2026-08-29 03:58'
updated_date: '2026-08-30 08:11'
tags:
  - execution-backend
  - windows
  - spike
---
# 目的と判断

TASK-7ではADR-0002に従い、GUI/domain層からprocess起動を隔離するReview Sweeper独自の`ExecutionBackend`境界を定義し、初期prototypeを`std::process`ベースのWindows native経路とする。process containmentの第一候補には`process-wrap 10.0.0`のWindows Job Object / POSIX process group wrapperを採用して最小実装を行った。

このspikeでTokio採用やproduction品質のprocess-tree supervisionを確定しない。`command-group`は`process-wrap`の前身、`processkit`はTokio前提の高機能な将来候補として比較結果を残し、採否はTASK-13で再決定する。WSL adapterの責務だけを定義し、検出、distro選択、path変換、WSL内部process管理を含む実装はTASK-27へ残す。

# 共通契約

prototypeは`spikes/execution-backend/src/lib.rs`に次の境界を置く。

- `ExecutionBackend`: `kind`、安定`identifier`、host pathからbackend pathへの明示変換、command実行を提供する。GUI、git、Worktree、workspace setupはOSのprocess APIへ直接依存せず、この境界を使う。
- `CommandRequest`: shell文字列ではなく`program`と`argv`を分離し、backendに束縛された`cwd`と親環境に対する`EnvironmentDelta`（追加・上書き・削除）を持つ。shell展開が必要な機能は呼び出し側が暗黙に有効化せず、将来別契約として明示する。
- `LogEvent`: `Stdout`/`Stderr`、UTF-8を仮定しないbyte chunk、単調増加`sequence`を持ち、callbackで完了前から逐次通知する。各stream内の順序とbackendが観測した到着順は維持するが、OS pipe間の厳密な同時刻順序は保証しない。表示層がdecode/redaction方針を決め、backendはstdoutとstderrを結合しない。
- `Completion`: 起動に成功したprocessの終端結果であり、backend identifier、PID、`Exited { code }`または`Cancelled`を返す。exit codeが非zeroでも実行errorにせず正常なCompletionとして扱う。signal等でportableなexit codeがない場合は`None`を許す。
- `CancellationToken`: thread-safeなcancel要求である。prototypeは要求検出後、containment単位へhard killし、reap完了後に`Cancelled`を返す。graceful stop、timeout、cancel escalation、drop時保証のproduction仕様はTASK-13で決める。
- `ExecutionError`: `BackendMismatch`、`PathConversion`、`UnsupportedHost`、`Launch`、実行中の`Io`を区別する。起動後の非zero exitや利用者cancelをerrorへ混ぜない。errorはUIが解決方法を提示できる構造化variantとし、表示文字列のparseを要求しない。

prototypeはstdout/stderrを別threadで同時にdrainする。`process-wrap`のstd `wait_with_output`はWindowsでstdoutを先に、stderrを後に読む制約が文書化されているため、両pipeが満杯になるdeadlockを境界実装側で避ける。

# backend identifierとWorktree固定

永続化するWorktree execution bindingは少なくとも次を一組で保持する。

```text
WorktreeExecutionBinding {
  worktree_id,
  backend_kind,
  backend_identifier,
  configuration_revision
}
```

`backend_kind`は`windows-native`、`wsl`、`macos-native`、`linux-native`を区別する。`backend_identifier`は設定instanceを安定識別し、Windows native prototypeは`windows-native:v1`、WSLは将来`wsl:<distro-stable-id>:v1`相当とする。表示名や現在のdistro名をidentifierの代用にしない。schemaの最終形とdistro stable IDの取得方法は各実装taskで決定する。

Worktree作成時にbindingを明示選択して固定し、画面、診断情報、command履歴にkindとidentifierを表示する。保存済みidentifierが現在利用不能、設定と不一致、またはpathの所有backendと不一致の場合は実行を拒否する。別のnative backendやdefault WSLへの暗黙fallbackは禁止し、再選択または明示的移行を要求する。同一Worktreeのgit状態、setup成果物、environmentを異なるbackendから暗黙に混在させない。

# path境界

Windows GUIが所有するhost pathと実行backend内pathは別の型とする。prototypeの`HostPath`と`BackendPath`は相互に代入できず、`BackendPath`は作成元の`BackendIdentifier`を保持する。`CommandRequest.cwd`は`BackendPath`だけを受け取る。

変換は`ExecutionBackend::host_path_to_backend`だけが担当し、absolute path、backend利用可否、対象filesystem、存在要件（操作ごとに必要な場合）を検証する。変換不能は`PathConversion`、別backendのpathを使った実行は`BackendMismatch`として明示的に失敗する。Windows nativeは同じpath表現を返せる場合も型を分けたままにし、identity conversionを理由に境界を省略しない。WSLのdrive mount、UNC、`\\wsl$`、symlink、case sensitivity等の具体変換はTASK-27で実測して決める。

# process library比較と実測

2026-08-29にcrate metadata、公式docs/source、prototypeで次を確認した。

| 候補 | 確認結果 | TASK-7での扱い |
| --- | --- | --- |
| `process-wrap 10.0.0` | `std`と`tokio1` frontendを選べ、Windows `JobObject`とUnix `ProcessGroup`をcomposable wrapperとして提供する。Windowsではsuspended起動後にJobへ関連付けてresumeし、cancel時の`kill`をjob/process groupへ伝播する。Linux/WSL上のprototype testに加え、Windows 11上のx86_64-pc-windows-msvc binaryで6 integration testとpath型compile-fail testを実行し、Windows Job Object分岐のruntime動作を確認した。 | std prototypeの第一候補。I/O streaming、error分類、cancel policyはReview Sweeper側の境界に保持する。 |
| `command-group 5.0.1` | `std::process::Command`拡張としてWindows Job Object / POSIX process groupを提供するが、公式に`process-wrap`がより柔軟でcomposableな後継と案内されている。 | 前身として記録し、新規prototypeには採用しない。既存利用資産が生じた場合のmigration比較対象。 |
| `processkit 3.3.4` | Tokio必須で、kill-on-drop、streaming、timeout/cancellation、pipeline、readiness、supervision、mock seam、Windows Job/Linux cgroup/POSIX group等を高水準で提供する。機能範囲と依存・runtime判断がTASK-7の最小spikeを超える。 | Tokio採用と高度なtree supervisionをTASK-13で決める際の将来候補。 |

一次資料:

- process-wrap: https://docs.rs/process-wrap/10.0.0/process_wrap/
- command-group: https://docs.rs/command-group/5.0.1/command_group/
- processkit: https://docs.rs/processkit/3.3.4/processkit/

process-wrap採用をproduction決定とみなさない。Windows nested Job制約、既存Job内起動、Ctrl-Break等のgraceful停止、childがcontainmentから離脱する場合、owner異常終了、長時間大量I/O、encoding、backpressure、resource limitをWindows実機で評価し、必要ならprocesskitまたはWindows API直接実装をTASK-13で再比較する。

# prototype検証範囲

`spikes/execution-backend`は次を自動化する。

- 正常終了とexit code 0
- nonzero exit 23をExecutionErrorではなくCompletionとして返す
- 存在しないprogramをLaunch errorとして返す
- 実行中cancelでcontained processを終了してCancelledを返す
- stdout/stderrを別LogEventとして分離し、sequenceを付ける
- relative host pathの変換拒否とbackend不一致の拒否

2026-08-30に同branchのUNC pathをWindows PowerShellから開き、Windows側MSVC Rust toolchainでruntime検証した。環境はMicrosoft Windows NT 10.0.22631.0、Windows PowerShell 5.1.22621.6133、rustc 1.98.0（host: x86_64-pc-windows-msvc）である。integration testはWindows上で`WindowsNativeBackend`を直接生成するため、`process-wrap::std::JobObject`を組み込む分岐を実行する。

```powershell
Set-Location "\\wsl.localhost\Ubuntu-24.04\home\kounoike\orca\workspaces\review-sweeper\task-7-execution-backend"
$env:CARGO_INCREMENTAL = "0"
$env:CARGO_TARGET_DIR = "$env:LOCALAPPDATA\Temp\review-sweeper-task7-target"
cargo test --manifest-path "spikes\execution-backend\Cargo.toml" --target x86_64-pc-windows-msvc -- --nocapture
cargo clippy --manifest-path "spikes\execution-backend\Cargo.toml" --target x86_64-pc-windows-msvc --all-targets --all-features -- -D warnings
```

Windows runtimeでは正常終了、nonzero exit、存在しないprogramのlaunch失敗、cancel、stdout/stderr分離、stable identifier/path境界の6 integration testと1 compile-fail doc testがすべて成功し、clippyもwarningなしで成功した。UNC path直下の既定`target`ではWindowsのincremental session lock作成が`os error -2147024895`となるため、上記のとおりincrementalを無効化しWindows local filesystem上の一時`CARGO_TARGET_DIR`を使う。これはtest対象sourceを変えず、生成物だけをWindows local filesystemへ置く再現条件である。

# WSL adapterと将来native backend

WSL backendは一つの巨大なprocess wrapperにせず、少なくとも次の責務を分離する。

1. Windows側adapterは選択済みdistro identifierの検証、host/backend pathの明示変換、`wsl.exe --distribution ... --exec ...`へのshell-freeなargv構築、Windows側launch/transport error、cancel要求の入口を所有する。
2. WSL内部process管理はLinux側PID/process group、stdout/stderr transport、exit/cancel完了、orphan防止を所有する。Windows側の`wsl.exe`だけをJob Objectへ入れることがLinux descendant全体の終了保証になるとは仮定しない。
3. 両者のprotocolはlaunch failure、transport/IO failure、Linux command exit、cancel acknowledgementを区別し、WSL path/environment/Git状態をWindows nativeへfallbackしない。

このadapter契約への`WslBackend`追加点だけをTASK-7で定義し、実装はTASK-27へ残す。macOS/Linux nativeは同じ`ExecutionBackend`、`CommandRequest`、`BackendPath`、`LogEvent`、`Completion`を再利用し、platform固有containmentとpath変換だけをadapter内へ閉じ込める。

# 認証・secret境界

ADR-0011とTASK-2の境界を維持する。GitHub access/refresh token、authorization code、PKCE verifier等はWindows GUI/`CredentialStore`が所有し、`CommandRequest.argv`、`EnvironmentDelta`、cwd、LogEvent、診断情報へ注入しない。`ExecutionBackend`はcredentialを取得するAPIを持たず、GitHub API操作は原則Windows側で行う。将来、外部commandがsecretを必要とする用途はこの汎用env差分へ暗黙に渡さず、redaction、最小権限、lifetime、transportを含む別の明示的Decisionを要求する。

# TASK-13での再検討条件

次のいずれかが必要になった時点でTokioとprocess libraryを再決定する。

- GUI runtimeと統合した非blocking I/O、bounded queue、backpressure
- graceful shutdownからhard killへの期限付きescalation
- owner drop/異常終了時を含む強いprocess-tree保証
- timeout、複数process/pipeline、readiness、restart supervision
- Windows nested Job、WSL内部process、PTY/interactive terminal
- resource limits、process member観測、test double/recording runner

prototypeの同期callback APIは境界の意味を検証するもので、production APIの同期/async形を固定しない。これらの要件とGUI runtimeが判明するTASK-13で、process-wrap継続、processkit採用、またはOS API直接実装を比較する。
