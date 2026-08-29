# 2. WindowsネイティブGUIとWSL実行バックエンドを分離する

Date: 2026-08-20

## Status

Accepted

Relates to [10. WindowsネイティブUIフレームワークにGPUIを採用する](0010-windows-ui-gpui.md)

Relates to [11. GitHub認証とsecretをWindowsネイティブ境界で所有する](0011-github-secret-windows.md)

## Context

Review Sweeperの最終的な対応OSはWindows、macOS、Linuxとする。初期ターゲットはWindowsであり、GUIはWindowsネイティブアプリケーションとして提供する。

Pull Requestのレビューでは、git、Worktree操作、workspace setupなどのコマンドを実行する必要がある。WindowsにはGit for Windowsなどのネイティブ実行環境がある一方、Linux向けの開発ツールやリポジトリのセットアップではWSLを利用したいケースがある。

WSLを必須依存にすると導入の負担が増え、WSLを使わないユーザーが基本的なレビュー機能を利用できなくなる。一方で、WSLを単なる外部コマンドとして扱うと、Windows側とWSL側のパス、環境変数、Git状態、Worktreeの扱いが混在しやすい。

## Decision

GUIとコマンド実行環境を分離し、WindowsネイティブGUIから実行バックエンドを選択できる構造にする。

初期ターゲットではWindowsネイティブ実行を基本バックエンドとし、WSLは必須依存ではなく、MVP後のできるだけ早い段階で追加する選択可能な実行バックエンドとする。

実行バックエンドはWorktree単位で明示・固定する。バックエンドは少なくとも次を区別する。

- Windowsネイティブ
- WSL2上の指定Linuxディストリビューション

バックエンドはgit、Worktree操作、workspace setupなどのコマンド実行に適用する。Windows側とWSL側のパス、環境変数、Git状態を暗黙に混在させない。

WSLが利用できない場合でも、Review Inbox、Diff確認、ソース確認、レビューコメント作成・送信などの基本レビュー機能は利用可能にする。将来のmacOS/Linux対応では、各OSのネイティブ実行環境を同じバックエンド抽象化の下で扱う。

## Consequences

- WindowsユーザーはWSLを導入せずに基本的なレビュー機能を利用できる。
- Linux向けツールチェーンやリポジトリのセットアップが必要な場合に、WSLを選択できる。
- GUIの実装とコマンド実行環境を分離でき、macOS/Linux対応時の拡張点が明確になる。
- WSL2とLinuxディストリビューションの検出、バックエンドの選択・表示、パス変換、環境変数、権限、エラー通知を実装・テストする必要がある。
- 同一WorktreeをWindowsとWSLから無秩序に操作しない運用と、バックエンド状態のUI表示が必要になる。
