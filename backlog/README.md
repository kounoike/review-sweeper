# Review Sweeperのバックログ

プロジェクトのバックログは[Backlog.md](https://github.com/MrLesk/Backlog.md)で管理する。タスクは`backlog/tasks/`配下のMarkdownファイルであり、単一のチェックリストを手作業で管理せずCLIを使う。

## よく使うコマンド

```sh
mise install
mise run backlog
mise run backlog-browser
backlog task list --plain
backlog task create "New task"
backlog task edit TASK-1 --check-ac 1
backlog search "worktree"
```

現在のエージェント向けワークフローは`backlog instructions overview`で確認する。1タスクには一貫した作業単位をまとめ、コーディング前に受け入れ条件を具体化し、実装メモと完了サマリーをタスクに記録する。
