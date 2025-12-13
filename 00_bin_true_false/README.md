# true & false (Rust)

Rust による `true` および `false` コマンドのシンプルな実装です。
O'Reilly の書籍「Command-Line Rust」の学習プロジェクト (Chapter 2) です。

## 概要

UNIX 標準コマンドの `true` と `false` を模倣しています。

- **true**: 何もせず、常に終了ステータス `0` (成功) で終了します。
- **false**: 何もせず、常に終了ステータス `1` (失敗) で終了します。

## ディレクトリ構成

```
.
├── Cargo.toml      # プロジェクト設定
├── src
│   └── bin
│       ├── false.rs    # false コマンドの実装
│       └── true.rs     # true コマンドの実装
└── tests
    └── cli.rs      # 統合テスト (assert_cmd を使用)
```

## ビルド

```bash
cargo build
```

## テストとコード品質

`assert_cmd` クレートを使用した統合テストが含まれています。

```bash
cargo test
```

コードの品質と慣用的な記述をチェックするために `Clippy` を使用できます。

```bash
cargo clippy
```

## 実行

`cargo run` コマンドを使用して、それぞれのバイナリを実行できます。

### true コマンド

```bash
cargo run --bin true
# 終了ステータスの確認
echo $?
# 出力: 0
```

### false コマンド

```bash
cargo run --bin false
# 終了ステータスの確認
echo $?
# 出力: 1
```
