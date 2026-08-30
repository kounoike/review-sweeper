# Rust workspace の責務境界

TASK-10 は製品機能を含まない Windows native アプリ基盤だけを定義する。依存方向は
外側の adapter から application、application から domain への一方向とし、GPUI や
OS 固有型を内側へ公開しない。`review-sweeper-architecture-tests` が Cargo metadata を
検査し、この規則への意図しない依存追加を検出する。

| Path | 責務 | 後続の主な拡張先 |
| --- | --- | --- |
| `apps/review-sweeper` | composition root、logging 初期化、起動 error と終了コード | adapter の組み立て |
| `crates/domain` | 外部技術から独立した純粋な domain 型 | Review Inbox、diff、review draft |
| `crates/application` | use case、port、UI 非依存 state/effect | Review Inbox application service |
| `crates/integrations` | GitHub、git、AI の adapter | TASK-14 以降の外部連携 |
| `crates/execution` | Windows native と将来の WSL2 execution adapter | TASK-13、TASK-27 |
| `crates/ui-gpui` | GPUI 固有の window、view、event-loop adapter | 後続 UI task |

現時点では `integrations` と `execution` に public trait を置かない。TASK-7 の prototype
は責務と検証結果であり、production の同期/async API や process library は TASK-13 で
再決定する。GitHub、git、AI、Review Inbox、diff、terminal、WSL の具体機能もこの基盤には
実装しない。
