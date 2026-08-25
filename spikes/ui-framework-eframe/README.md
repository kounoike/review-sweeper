# eframe/egui Windows 技術検証プロトタイプ

TASK-1でGPUI 0.2.2と同一条件のWindows desktop検証を行うための隔離prototypeであり、製品基盤ではない。

- `ScrollArea::show_rows`による100,000行の可視範囲生成
- pointer clickと上下keyによる選択
- background threadからの`Context::request_repaint`
- standard `TextEdit`へのUnicode入力とAccessKit/UI Automation観測

Windows 11のDeveloper PowerShellから次を実行する。

```powershell
cargo run --manifest-path .\spikes\ui-framework-eframe\Cargo.toml
```

このprototypeはdiff syntax、可変高row、ConPTY、terminal frontendを実装しない。Narrator音声、IME composition/candidate window、GPU性能は別途実機で確認する。
