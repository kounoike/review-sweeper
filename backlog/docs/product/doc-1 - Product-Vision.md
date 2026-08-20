---
id: doc-1
title: Product Vision
type: specification
created_date: '2026-08-21 03:40'
updated_date: '2026-08-20 18:49'
---
# Review Sweeper — Product Vision

## 概要

**Review Sweeper** は、GitHub Pull Request のレビューを高速かつ確実に行うための、レビュー作業に特化したネイティブクライアントである。

GitHub 上でレビュー依頼を受けてから、変更内容を理解し、必要に応じてローカル環境や AI を使って検証し、レビューコメントを書き、最終的な判断を GitHub に返すまでの一連の作業を、できるだけ少ないコンテキストスイッチで完了できることを目指す。

Review Sweeper はコードを書くための IDE ではない。

**Pull Request を理解し、検証し、判断するための Review Workbench である。**

---

## 解決したい問題

Pull Request のレビューでは、実際には GitHub の Diff 画面を見るだけでは済まない。

レビュー中には頻繁に次のような作業が発生する。

* 自分にレビュー依頼されている Pull Request を探す
* Pull Request の概要や変更ファイルを確認する
* Diff を読む
* 変更されたコードの周辺を含めてファイル全体を読む
* 前回レビュー以降に追加された変更だけを見る
* CI / GitHub Checks の状態を確認する
* CI 失敗の原因を調べる
* ローカルに Pull Request を checkout する
* 依存パッケージなどを準備する
* IDE でコードを詳しく調べる
* 必要に応じてコードを一時的に変更して挙動を確認する
* AI にレビューや調査を依頼する
* レビューコメントを整理する
* Approve / Request Changes / Comment を送信する

現在はこれらの作業が GitHub、IDE、ターミナル、AI ツールなど複数の場所に分散している。

Review Sweeper は、この一連の流れを **1つの Review Session** として扱う。

---

## Product Goal

Review Sweeper の最も重要な目標は、

> **レビュー依頼を受けてから、十分な根拠を持ってレビュー結果を返すまでの時間と認知負荷を最小化すること**

である。

単純に画面表示を高速化することだけが目的ではない。

レビュー開始前に必要な準備を自動化し、レビュー中に必要となる情報へすぐアクセスでき、レビュー後の再確認も容易にすることで、レビュー工程全体を高速化する。

---

## 基本ワークフロー

Review Sweeper が想定する基本的なレビューの流れは次の通りである。

```text
Review requested
        ↓
Preparation
 ├─ Fetch PR metadata
 ├─ Fetch diff
 ├─ Create worktree
 ├─ Setup workspace
 └─ AI pre-review
        ↓
Ready for Review
        ↓
Understand
 ├─ PR description
 ├─ Changed files
 └─ AI findings
        ↓
Inspect
 ├─ Diff
 ├─ Full source
 ├─ Previous review → HEAD
 └─ Local workspace changes
        ↓
Verify
 ├─ GitHub Checks
 ├─ External editor
 └─ Local experiments
        ↓
Review
 ├─ Draft comments
 ├─ AI-assisted investigation
 └─ Review progress
        ↓
Decision
 ├─ Approve
 ├─ Comment
 └─ Request Changes
```

---

## Review Inbox

Review Sweeper の起点は通知一覧ではなく、**Review Inbox** である。

Review Inbox は単純な GitHub Notifications のコピーではない。

レビュー担当者の視点から Pull Request を整理する。

想定する状態には以下がある。

* Needs Review
* Preparing
* Ready
* Updated Since Review
* Waiting for Author
* Checks Failed
* Reviewed

ユーザーが Review Inbox を見れば、

> 「今、自分がレビューすべきものは何か」

が分かることを目指す。

---

## Preparation

Pull Request を開いてから準備を始めるのではなく、レビュー依頼を受けた時点からバックグラウンドで準備できることを重要な機能とする。

Preparation では、ユーザー設定に応じて以下を実行できる。

### Minimal

* Pull Request metadata の取得
* Diff の取得とキャッシュ

### Workspace

* Git repository の取得・更新
* Pull Request 用 Worktree の作成
* Workspace setup

### AI Review

* Pull Request diff を利用した AI pre-review

### Full

* Worktree の準備
* Workspace setup
* AI review

可能な処理は並行して実行する。

Preparation の目的は、自動でレビューを完了することではない。

> **ユーザーがレビューを始めた瞬間に、必要な材料がすでに揃っている状態を作る**

ことが目的である。

---

## AI の位置づけ

Review Sweeper は AI にレビュー判断を委任するアプリではない。

AI はレビュー担当者のための **investigation assistant** として扱う。

AI は例えば以下を行う。

* Diff の一次レビュー
* 問題になりそうな変更の抽出
* 関連コードの探索
* 関連テストの探索
* 過去の変更履歴の確認
* GitHub Checks の失敗原因の整理
* レビューコメント案の作成

AI が生成した結果は、GitHub に直接投稿しない。

AI finding は一度ローカルのレビュー情報として保存し、ユーザーが確認した上で GitHub review comment に変換する。

最終的なレビュー判断は常にユーザーが行う。

---

## Worktree の位置づけ

各 Pull Request には独立した Git Worktree を作成できる。

Worktree は以下の用途に利用する。

* ファイル全体の参照
* AI による repository-level investigation
* 外部エディタからのコード確認
* ローカルでのテストや調査
* 必要に応じた一時的なコード変更

複数の Pull Request を並行してレビューしても checkout が競合しないことを重視する。

---

## External Editor

Review Sweeper 自身にはコード編集機能を実装しない。

コード編集、LSP、リファクタリング、複雑なターミナル操作などは既存の開発環境に委譲する。

Worktree はユーザーが設定した外部ツールで開ける。

例:

* Visual Studio Code
* Visual Studio Code Insiders
* Cursor
* Zed
* JetBrains IDE
* その他カスタムコマンド

Review Sweeper から特定ファイル・行を外部エディタで直接開けることも想定する。

---

## Local Workspace Changes

外部エディタで Worktree が変更される可能性があるため、Review Sweeper はローカル状態を常に観測する。

例えば以下を UI 上で明確に表示する。

* Clean
* Modified
* Staged
* Untracked files
* Conflicted
* HEAD differs from PR HEAD

ただし、

> **Local workspace state must never be silently incorporated into the PR review target.**

を原則とする。

通常のレビュー対象は GitHub 上の Pull Request HEAD であり、ローカル変更は別の状態として明示する。

---

## 差分の考え方

Review Sweeper では「Diff」を1種類として扱わない。

少なくとも以下を区別する。

### Pull Request Diff

```text
Base → PR HEAD
```

通常の Pull Request レビュー。

### Re-review Diff

```text
Last Reviewed Revision → PR HEAD
```

前回レビュー以降に変更された箇所だけを確認する。

### Local Diff

```text
PR HEAD → Local Working Tree
```

外部エディタなどで行ったローカル変更を確認する。

特に **Re-review Diff** を Review Sweeper の重要な機能とする。

---

## Review Progress

レビュー作業の進捗を明示的に管理する。

例えば、

```text
12 files

✓ 8 reviewed
● 2 changed since reviewed
○ 2 not reviewed
```

のように状態を表示する。

一度確認済みにしたファイルでも、その後 Pull Request に変更が追加された場合には再レビュー対象として扱えるようにする。

---

## 成功の定義

Review Sweeper が成功している状態とは、単に GitHub Web UI より高機能であることではない。

レビュー担当者が、

* レビュー待ちを見つける
* Pull Request を理解する
* 周辺コードを調べる
* AI の助けを借りる
* CI を確認する
* 必要ならローカルで検証する
* コメントを書く
* 再レビューする
* 最終判断を返す

という一連の流れを、迷わず高速に処理できることである。

究極的には、

> **Review Inbox を安全に、確実に、素早く空にできること**

を Review Sweeper の価値とする。
