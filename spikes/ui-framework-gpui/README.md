# GPUI Windows 技術検証プロトタイプ

TASK-1 の判断材料だけを得るための使い捨てプロトタイプであり、製品基盤ではない。
`src/lib.rs` は UI 非依存の状態と `UiIntent` に加え、PR diff の可視範囲・選択・
増分 invalidation、および terminal frontend と backend の port を示す。
`src/main.rs` は GPUI アダプターである。

確認対象は次のとおり。

- Win32 ウィンドウの起動
- マウスによる行選択と、上下キーによる選択変更
- `uniform_list` による 100,000 行の仮想化表示とスクロール
- `BackgroundExecutor` で実行した処理の完了後に UI 状態を更新
- diff 更新時に可視範囲との共通部分だけを再描画対象にする pure Rust の契約
- `WindowsConPty` と `Wsl { distribution }` を明示的に分ける `TerminalBackendTarget`

diff の syntax highlighting、gutter、comment、fold/search/copy と terminal の VT engine、
ConPTY/WSL transport、IME、accessibility は、このプロトタイプでは実装しない。
これらを framework widget の状態にせず、snapshot/intent/transport port の背後で実装するための
境界だけを検証対象とする。

## Windows での再現

Windows 11、DirectX 11 対応 GPU、Visual Studio 2022 Build Tools の MSVC/C++
ツールを用意し、リポジトリルートの PowerShell から実行する。

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo run --manifest-path .\spikes\ui-framework-gpui\Cargo.toml
```

次を手動で確認する。

1. ウィンドウが表示され、サイズ変更できる。
2. 行のクリックと上下キーで選択行が変わる。
3. ホイールまたはスクロール操作で先頭から 100,000 行目付近まで移動でき、操作中に固まらない。
4. `R` または右上のボタンを押すと「更新中…」になり、約 500 ms 後に generation が増える。
5. Narrator または Accessibility Insights for Windows で、ボタン、選択行、行一覧の名前・role・状態が公開されるかを観察する。

項目 5 は GPUI 0.2.2 の公開 API にアクセシビリティツリーの登録機能を確認できないため、
未対応である可能性を明示的に検証する。WSL2 上の Linux ビルドや Windows 向け
`cargo check` は、Windows の入力、描画性能、スクリーンリーダー連携の代替にはならない。
