# commr

`commr` は、Rustで記述された `comm` コマンドのクローンです。2つのソートされたファイルの共通または一意な行を見つけるために使用されます。

## 目次

- [必要な環境](#必要な環境)
- [ビルド](#ビルド)
- [インストール](#インストール)
- [使用方法](#使用方法)
- [テスト](#テスト)
- [コード品質 (Clippy)](#コード品質-Clippy)
- [カバレッジ計測](#カバレッジ計測)

## 必要な環境

- Rust (バージョン 1.70 以上)
- Cargo (Rustに付属)
- cargo-llvm-cov (カバレッジ計測用)

`cargo-llvm-cov` は以下のコマンドでインストールできます。

```bash
cargo install cargo-llvm-cov
```

## ビルド

プロジェクトをビルドするには、以下のコマンドを実行します。

```bash
cargo build
```

## インストール

`commr` をシステムにインストールするには、以下のコマンドを実行します。

```bash
cargo install --path .
```

## 使用方法

`commr` コマンドは、2つのファイルを比較し、共通または一意な行を出力します。

```bash
commr [OPTIONS] <FILE1> <FILE2>
```

**例:**

```bash
# file1.txt と file2.txt を比較し、デフォルトの出力を行う
commr file1.txt file2.txt

# file1.txt と file2.txt を比較し、コラム1の行を非表示にする
commr -1 file1.txt file2.txt

# file1.txt と file2.txt を比較し、大文字小文字を区別しない
commr -i file1.txt file2.txt

# 標準入力からファイル1を読み込み、ファイル2と比較する
commr - file2.txt
```

オプション:
- `-1`: 最初のファイルにのみ存在する行の出力抑制
- `-2`: 2番目のファイルにのみ存在する行の出力抑制
- `-3`: 両方のファイルに存在する行の出力抑制
- `-i`, `--insensitive`: 大文字と小文字を区別しない比較
- `-d <DELIMITER>`, `--output-delimiter <DELIMITER>`: 出力区切り文字の指定 (デフォルトはタブ)

## テスト

ユニットテストと統合テストを実行するには、以下のコマンドを使用します。

```bash
cargo test
```

## コード品質 (Clippy)

コード品質をチェックし、一般的な間違いや非イディオムなコードを検出するには、Clippyを使用します。

```bash
cargo clippy
```

## カバレッジ計測

コードカバレッジを計測するには、`cargo-llvm-cov` を使用します。サマリーのみを表示するには、以下のコマンドを実行します。

```bash
cargo llvm-cov --summary-only
```

詳細なHTMLレポートを生成するには、以下のコマンドを実行します。

```bash
cargo llvm-cov --html
```
HTMLレポートは `target/llvm-cov/html/index.html` に出力されます。
