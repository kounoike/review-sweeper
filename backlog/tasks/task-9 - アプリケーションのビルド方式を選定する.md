---
id: TASK-9
title: アプリケーションのビルド方式を選定する
status: To Do
assignee: []
created_date: '2026-08-20 18:13'
updated_date: '2026-08-20 18:14'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-3
type: spike
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Review Sweeperを各開発環境で再現可能にビルドし、利用者へ配布するためのビルド方式を選定する。Rustのターゲット、依存するネイティブライブラリ、成果物形式、クロスコンパイル、署名の前提を整理する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 開発用・CI用・配布用のビルド経路を比較する。
- [ ] #2 対応OSとCPUアーキテクチャ、ネイティブ依存、クロスコンパイルの制約を整理する。
- [ ] #3 成果物の形式と再現可能なビルド手順を決め、理由を記録する。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
