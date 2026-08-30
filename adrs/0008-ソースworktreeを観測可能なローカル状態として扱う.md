# 8. ソースworktreeを観測可能なローカル状態として扱う

Date: 2026-08-21

## Status

Accepted

Relates to [4. PR準備とレビューセッションを分離する](0004-PR準備とレビューセッションを分離する.md)

Relates to [6. GitHub差分とローカル差分を別のソースとして扱う](0006-GitHub差分とローカル差分を別のソースとして扱う.md)

## Context

ユーザーは外部エディターでPRのworktreeを開き、ファイル変更、変更のステージ、コミット作成、HEADの誤操作を行う可能性がある。レビューアプリはPRのリモート状態とローカルの実験を混同してはならない。

## Decision

ソースworktreeをPRのレビュー対象とは分離された、観測可能なローカル状態として扱う。

アプリはPRの変更、ユーザーの前回レビュー以降の変更、ローカルworkspaceの変更を区別する。ローカル変更をPRレビュー対象やAIレビュー対象へ暗黙に取り込んではならない。

## Consequences

- UIはclean、modified、staged、untracked、conflicted、broken、HEAD-divergedの状態を明確に表示する必要がある。
- worktree状態は軽量なgit status確認をデバウンスして更新する。
- 明示的な確認なしに変更済みworktreeをリセットまたは削除してはならない。
- ユーザーはPRに属するものを理解したまま、ローカル編集をレビューの実験に使える。
