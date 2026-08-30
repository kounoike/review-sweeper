# 6. GitHub差分とローカル差分を別のソースとして扱う

Date: 2026-08-21

## Status

Accepted

Relates to [7. 構造化された差分モデルを使う](0007-構造化された差分モデルを使う.md)

Relates to [8. ソースworktreeを観測可能なローカル状態として扱う](0008-ソースworktreeを観測可能なローカル状態として扱う.md)

## Context

GitHubの差分データは高速表示と、GitHubのレビューAPIでインラインレビューコメントを正しく配置するために有用である。ローカルのgit差分データは再計算、別の比較、空白処理、ローカル変更、前回レビューとの比較に有用である。

両方のソースを同一視すると、重要な動作上の差異が隠れてしまう。

## Decision

GitHubが提供するPR差分データと、ローカルで計算したgit差分データを別のドメイン概念として表現し、共通の構造化差分モデルへparseする。

各snapshotはsource、比較revision・commit identity、GitHub review comment座標を失わない。UIは共通の差分ビューアーコンポーネントで描画してよいが、review commentの送信座標はGitHub review snapshotからのみ生成し、local差分やwhitespace除外表示から推測・逆変換しない。

## Consequences

- インラインコメントの対応付けをGitHubの意味論に合わせ続けられる。
- ローカル比較をGitHubの正規PR差分と偽ることなく、より豊かなレビューに利用できる。
- local差分とGitHub review snapshotの対応関係は比較表示に利用できるが、GitHubレビューコメントAPIの送信座標には昇格させない。
