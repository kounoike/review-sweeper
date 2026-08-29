---
id: TASK-3
title: Windows初期ターゲットのビルド方式を選定する
status: Done
assignee:
  - '@kounoike'
created_date: '2026-08-20 18:13'
updated_date: '2026-08-29 03:39'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-1
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0003-ツール管理にMiseを使う.md
  - adrs/0010-windows-ui-gpui.md
documentation:
  - doc-7
modified_files:
  - mise.toml
  - backlog/docs/technical/windows-build/doc-7 - Windows初期ターゲットのビルド方式.md
  - backlog/tasks/task-3 - Windows初期ターゲットのビルド方式を選定する.md
type: spike
ordinal: 3
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0001/0002に基づくRustネイティブアプリをWindowsで再現可能に開発・CI・配布ビルドする方式を選定する。将来のmacOS/Linux追加を妨げない制約は整理するが、初期タスクで全OS対応を確約しない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Windows向けの開発用、CI用、配布用ビルド経路を比較する
- [x] #2 Windowsの対象CPUアーキテクチャ、Rust target、UIのネイティブ依存、成果物形式を整理する
- [x] #3 将来のmacOS/Linux対応とクロスコンパイルについて、現時点の制約と再検討条件を記録する
- [x] #4 mise経由の再現可能なビルド手順と採用理由を技術メモへ記録する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. ADR-0001/0002/0003/0010、TASK-1のWindows実証、Rust/GPUI/Microsoft/miseの公式一次情報を照合し、開発・CI・配布build候補を比較する。
2. 初期build基準をWindows native x86-64、x86_64-pc-windows-msvc、VS 2022/Windows SDK、GPUI pinに定め、ARM64/macOS/Linux/cross compileの制約と再検討条件を技術メモへ記録する。
3. GPUI evidence crateを対象にwindows-build-dev、windows-check、windows-build-releaseをmise.tomlへ追加し、将来の製品workspaceへ移すtarget契約を示す。
4. Windows native MSVCでfmt、clippy、test、release buildを実証し、PE header、runtime dependency、SHA-256を確認する。設定/文書はmise tasks validate、git diff --check、backlog-check、adr-doctorで検証する。
5. TASK-3の成果物契約をrelease x64 PEとprovenance/dependency inventory/SHA-256までに限定し、GitHub Releases、installer/package、署名、公開、更新の最終方式は既存TASK-12に残す。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-29 着手: git status --shortはclean。TASK-1がDone、ADR-0001/0002/0003/0010がAccepted、adr-doctor成功を確認した。AC #1は開発/CI/配布のコマンド・実行host・toolchain・出力を比較表で検証、#2はCPU/target/native依存/成果物を明記、#3はmacOS/Linux/cross compileの非保証範囲と再検討条件を明記、#4はmise taskの実行結果と採用理由を技術メモへ記録することで判定する。重要なarchitecture/配布方式の最終決定はコーディネーター承認まで保留する。

2026-08-29 調査: Rust公式ではx86_64-pc-windows-msvcはTier 1 with host toolsで、non-Windows hostからMSVC targetへのcross compileはunsupported。Microsoft一次情報ではMSIX一般配布はtrusted署名が必要、Store提出MSIXはStoreが再署名する。GitHub Actionsはx64 windows-2022を明示でき、VS 2022/複数Windows SDKを含む。初期基準をWindows native x64 MSVC、CIをpinned windows-2022、cross compileをcompile-only補助、ARM64/macOS/Linuxを条件付き再検討としてdoc-7へ整理した。配布packageはportable ZIP推奨案、Store MSIX、direct signed MSIXを比較し、質問msg_7345dc4ac80cで判断待ち。

2026-08-29 scope確認: TASK-12がTASK-3/TASK-11に依存し、GitHub Releases、installer/package manager、成果物命名、署名/checksum/secret、公開範囲、更新を明示的に担当する。質問msg_7345dc4ac80cは2回合計20分timeoutし未回答。このためTASK-3では一般配布方式を確定せず、Windows native release x64 PEとprovenance/dependency inventory/SHA-256までをbuild成果物契約とし、公開packageの最終決定をTASK-12へ残す。portable ZIPは非公開CI/review artifact候補に留める。

2026-08-29 検証: Windows Rust 1.98.0/MSVC 14.39でcargo fmt --check、x86_64-pc-windows-msvc向けclippy -D warnings、test（4 passed）、release buildが成功。11,101,696 byteのx64 PE、SHA-256 0d6d001cc999137164f9ab5f5e9dd7c76bbcd759aa5568fac32ec28a9cd0ba05を確認。dumpbinでCUI subsystem（製品crateでGUI化が必要）とVCRUNTIME140/Universal CRT/system DLL依存を確認した。mise tasks validateは14 task/error 0/warning 0、git diff --check、backlog-check、adr-doctorも成功。Windows側mise.exeは未導入のためmise process自体はLinux側schema/依存graph検証、Windows側ではtaskと同一cargo command bodyを直接実行した。生成したWindows Temp build cacheは記録後に削除した。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Windows初期build基準をWindows native x86-64 / x86_64-pc-windows-msvc / VS 2022・Windows SDK / pinned GPUI・Cargo.lockに定め、開発・windows-2022 CI・release buildと将来OS/cross compile制約をdoc-7へ記録した。mise.tomlへwindows-build-dev/windows-check/windows-build-releaseを追加。Windows Rust 1.98.0/MSVC 14.39でfmt、clippy、test 4件、release buildを成功させ、11,101,696 byte x64 PE、SHA-256、VCRUNTIME140/Universal CRT依存を確認した。mise task schema/graph、git diff --check、backlog-check、adr-doctorも成功。一般配布package/署名/公開/更新は既存TASK-12のスコープとして未確定のまま保持し、このdispatchの指示どおりpush/PR/mergeは行わない。
<!-- SECTION:FINAL_SUMMARY:END -->
