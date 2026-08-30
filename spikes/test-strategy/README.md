# TASK-8 test strategy spike

外部network、GitHub account、WSL、Windows desktopに依存せず、TASK-8の代表的なfixture契約とfailure oracleを検証する。

```bash
cargo test --manifest-path spikes/test-strategy/Cargo.toml
bash spikes/test-strategy/verify.sh
```

`fixtures/manifest.json`はproduct schemaではなく、fixtureの分類漏れとschema driftを検出するversioned test contractである。GitHub live APIおよびWindows native UI/ConPTY/IME/UIAはこのharnessで代替せず、test strategy文書で別laneとして扱う。
