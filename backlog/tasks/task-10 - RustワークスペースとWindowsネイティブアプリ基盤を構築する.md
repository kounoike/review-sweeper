---
id: TASK-10
title: RustワークスペースとWindowsネイティブアプリ基盤を構築する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-23 00:53'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-1
  - TASK-3
  - TASK-7
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
type: chore
ordinal: 10
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0001に基づき、選定したUIフレームワークを使うRustワークスペースと、Windowsで起動できる最小のネイティブアプリ基盤を構築する。GitHub、git、AI、UI、実行環境の関心事を後続タスクで分離できる構成にし、製品機能は含めない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Rustワークスペース内でアプリケーション、ドメイン、外部連携、UI、実行バックエンドの責務境界が定義されている
- [ ] #2 選定したUIフレームワークでWindowsネイティブの最小ウィンドウを起動できる
- [ ] #3 mise経由でfmt、lint、test、check、buildを再現できる
- [ ] #4 Windows固有の開発依存と起動手順が文書化されている
- [ ] #5 GitHub、git、AI、UIの具体的な製品機能が基盤タスクへ混在していない
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
