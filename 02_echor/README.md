# echor

Rust 版の `echo` コマンドの実装です。
[Command-Line Rust](https://github.com/kyclark/command-line-rust) の第2章の演習として作成されました。

## 目次 (TOC)

- [echor](#echor)
  - [目次 (TOC)](#目次-toc)
  - [機能](#機能)
  - [ファイル構成](#ファイル構成)
  - [使用方法](#使用方法)
    - [ビルドと実行](#ビルドと実行)
  - [開発](#開発)
    - [テスト](#テスト)
    - [Lint (Clippy)](#lint-clippy)
    - [カバレッジ](#カバレッジ)

## 機能

標準の `echo` コマンドと同様に、引数として渡されたテキストを標準出力に表示します。

- 複数の引数をスペース区切りで連結して表示
- `-n` オプションによる末尾の改行抑制

## ファイル構成

主要なファイル構成は以下の通りです。

```text
.
├── Cargo.toml      # プロジェクト設定・依存関係
├── mk-outs.sh      # テスト用の期待値ファイルを生成するスクリプト
├── src/
│   └── main.rs     # ソースコード本体 (引数解析とロジック)
└── tests/
    ├── cli.rs      # 統合テストコード
    └── expected/   # テストの期待値ファイル群
```

## 使用方法

### ビルドと実行

```bash
cargo run -- "Hello World"
# 出力: Hello World

# 改行なしオプション
cargo run -- -n "Hello World"
# 出力: Hello World%
```

## 開発

### テスト

`cargo test` を使用して、単体テストおよび統合テストを実行できます。
統合テスト (`tests/cli.rs`) では、`assert_cmd` を使用してバイナリを実際に実行し、振る舞いを検証しています。

```bash
cargo test
```

### Lint (Clippy)

Rust の標準 Linter である Clippy を使用して、コードの品質をチェックできます。

```bash
cargo clippy
```

### カバレッジ

`cargo-llvm-cov` を使用して、テストのカバレッジを計測できます。

```bash
# カバレッジの概要を表示
cargo llvm-cov --summary-only

# カバレッジの詳細レポート (HTML) を生成
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```
