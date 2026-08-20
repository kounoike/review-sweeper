# アーキテクチャ決定記録

アーキテクチャ決定記録（ADR）は[`adrs`](https://github.com/joshrotenberg/adrs)で管理する。既存のADRは、同ツールのadr-tools互換クラシック形式を使用している。

## よく使うコマンド

```sh
mise install
mise run adr-list
mise run adr-doctor
adrs new "Decision title"
adrs edit 1
adrs status 1 accepted
adrs search "worktree"
adrs generate toc > adrs/README.md
```

このディレクトリの番号付きMarkdownファイルがADRリポジトリである。決定の作成、編集、検索、関連付け、検査にはCLIを使い、索引を手作業で管理しない。機能仕様は`backlog/docs/`、実装作業は`backlog/tasks/`で管理する。
