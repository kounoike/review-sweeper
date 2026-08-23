---
id: TASK-1
title: Windows向けRust UIフレームワークを技術検証する
status: To Do
assignee: []
created_date: '2026-08-20 18:06'
updated_date: '2026-08-23 00:49'
labels:
  - project-setup
milestone: m-0
dependencies: []
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
type: spike
ordinal: 1
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0001の第一候補であるGPUIを中心に、WindowsネイティブGUIとしてReview Sweeperの初期実装に採用できるかを最小プロトタイプで検証する。製品コードの大部分が依存する前に、採用または代替案を確定する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GPUIを含む候補をWindows対応、ビルド環境、非同期処理、描画性能、アクセシビリティ、保守性で比較する
- [ ] #2 Windows上でウィンドウ、基本入力、スクロール可能な大量行表示、バックグラウンド処理からの更新を最小プロトタイプで確認する
- [ ] #3 採用案、見送った案、既知の制約、再検討条件をADRまたは技術メモに記録する
- [ ] #4 UIフレームワークを交換可能に保つ境界を後続の基盤タスクから参照できる
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
