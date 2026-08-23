---
id: TASK-3
title: Windows初期ターゲットのビルド方式を選定する
status: To Do
assignee: []
created_date: '2026-08-20 18:13'
updated_date: '2026-08-23 00:53'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-1
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
type: spike
ordinal: 3
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0001/0002に基づくRustネイティブアプリをWindowsで再現可能に開発・CI・配布ビルドする方式を選定する。将来のmacOS/Linux追加を妨げない制約は整理するが、初期タスクで全OS対応を確約しない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Windows向けの開発用、CI用、配布用ビルド経路を比較する
- [ ] #2 Windowsの対象CPUアーキテクチャ、Rust target、UIのネイティブ依存、成果物形式を整理する
- [ ] #3 将来のmacOS/Linux対応とクロスコンパイルについて、現時点の制約と再検討条件を記録する
- [ ] #4 mise経由の再現可能なビルド手順と採用理由を技術メモへ記録する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
