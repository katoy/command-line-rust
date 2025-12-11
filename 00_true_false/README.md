# 00_true_false

Unixの `true` と `false` コマンドをRustで再実装したプロジェクトです。

## プロジェクト構成

```
00_true_false/
├── Cargo.toml      # ワークスペース設定
├── true_cmd/       # true コマンド実装
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/
│   │   └── main.rs
│   └── tests/
│       └── cli.rs
└── false_cmd/      # false コマンド実装
    ├── Cargo.toml
    ├── README.md
    ├── src/
    │   └── main.rs
    └── tests/
        └── cli.rs
```

## ビルド

```bash
# ワークスペース全体をビルド
cargo build --release
```

## テスト

```bash
# 全テスト実行
cargo test

# 個別テスト
cargo test -p true_cmd
cargo test -p false_cmd
```

## Lint

```bash
cargo clippy --all-targets --all-features -- -W clippy::pedantic
```

## 各コマンドについて

| コマンド | 終了コード | 説明 |
|---------|-----------|------|
| `true_cmd` | 0 (成功) | 常に成功を返す |
| `false_cmd` | 1 (失敗) | 常に失敗を返す |

## 参考

- 書籍: [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/)
