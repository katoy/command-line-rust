# catr - Rust implementation of `cat`

「Command Line Rust」の第3章で作成する `cat` コマンドの Rust 実装です。
標準の `cat` コマンドの主要な機能をサポートしています。

## 目次

- [概要](#概要)
- [ファイル構成](#ファイル構成)
- [インストールと実行](#インストールと実行)
- [標準 cat コマンドとの違い](#標準-cat-コマンドとの違い)
- [開発](#開発)
  - [Lint](#lint)
  - [Format](#format)
  - [Test & Coverage](#test--coverage)

## 概要

`catr` は、ファイルの内容を標準出力に連結して表示するコマンドラインツールです。
標準入力からの読み込みや、行番号の表示などのオプションをサポートしています。

## ファイル構成

- `src/main.rs`: アプリケーションのメインロジック。
- `tests/cli.rs`: 統合テスト。`assert_cmd` を使用して CLI の挙動をテストします。
- `tests/inputs/`: テスト用の入力ファイル。
- `tests/expected/`: テストの期待される出力ファイル。

## インストールと実行

Rust がインストールされている環境で、以下のコマンドで実行できます。

```bash
# ヘルプの表示
cargo run -- --help

# ファイルの表示
cargo run -- tests/inputs/fox.txt

# 行番号を表示 (-n)
cargo run -- -n tests/inputs/fox.txt

# 標準入力からの読み込み
echo "hello" | cargo run -- -
```

## 標準 cat コマンドとの違い

本実装 (`catr`) と標準的な `cat` コマンド (GNU cat や BSD cat) には、以下の仕様上の差異があります。

- **行番号のリセット**:
  - `catr`: 行番号オプション (`-n`, `-b`) を使用した際、**ファイルごとに行番号がリセット (1から再開) されます**。
  - 標準 `cat`: 複数のファイルを指定した場合、行番号はファイル間で通し番号となります。

この挙動は、本プロジェクトの仕様 (およびテストケース) に基づくものです。

## 開発

### Lint

コードの静的解析には `clippy` を使用します。

```bash
cargo clippy
```

### Format

コードのフォーマットチェックと整形には `rustfmt` を使用します。

```bash
# チェックのみ
cargo fmt -- --check

# 整形を適用
cargo fmt
```

### Test & Coverage

テストの実行は標準の `cargo test` を使用します。

```bash
cargo test
```

カバレッジ計測を行いたい場合は、`llvm-cov` などのツールを使用することが推奨されます（追加のインストールが必要です）。

```bash
# (例) cargo-llvm-cov がインストールされている場合
cargo llvm-cov
```
