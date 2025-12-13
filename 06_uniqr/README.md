# uniqr

Rust による `uniq` コマンドの実装です。隣接する重複行をフィルタリングまたは報告します。

## 目次

- [概要](#概要)
- [インストール](#インストール)
- [使い方](#使い方)
  - [引数](#引数)
  - [オプション](#オプション)
  - [実行例](#実行例)
- [テスト](#テスト)
- [開発](#開発)
  - [Lint (Clippy)](#lint-clippy)
  - [カバレッジ計測](#カバレッジ計測)

## 概要

`uniqr` は Unix の `uniq` ユーティリティを模倣したコマンドラインツールです。入力テキストから重複した行を検出し、処理します。

## インストール

Rust の環境が必要です。以下のコマンドでビルドできます。

```bash
cargo build --release
```

## 使い方

基本的な使い方は以下の通りです。

```bash
cargo run -- [OPTIONS] [IN_FILE] [OUT_FILE]
```

または、ビルドされたバイナリを直接実行します。

```bash
./target/release/uniqr [OPTIONS] [IN_FILE] [OUT_FILE]
```

### 引数

- `IN_FILE`: 入力ファイル。指定しない場合や `-` の場合は標準入力から読み取ります。
- `OUT_FILE`: 出力ファイル。指定しない場合は標準出力に出力します。

### オプション

- `-c`, `--count`: 各行の出現回数を行の先頭に出力します。
- `-h`, `--help`: ヘルプメッセージを表示します。
- `-V`, `--version`: バージョン情報を表示します。

### 実行例

標準入力から読み込み、標準出力へ書き出す:
```bash
$ echo -e "apple\napple\nbanana" | cargo run
apple
banana
```

ファイルから読み込み、カウントを表示する:
```bash
$ cargo run -- -c input.txt
   2 apple
   1 banana
```

## テスト

プロジェクトのテストを実行するには以下のコマンドを使用します。

```bash
cargo test
```

## 開発

### Lint (Clippy)

コードの品質チェックには `clippy` を使用します。

```bash
cargo clippy
```

### カバレッジ計測

テストカバレッジの計測には `cargo-llvm-cov` を使用します。

まずツールをインストールします（初回のみ）。

```bash
cargo install cargo-llvm-cov
```

カバレッジを計測してサマリーを表示するには以下を実行します。

```bash
cargo llvm-cov --summary-only
```