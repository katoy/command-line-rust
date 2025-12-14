# Fortuner - Rust版 Fortune

Rustで実装された、UNIXの伝統的なコマンドラインツール `fortune` のクローンです。
`fortune` は、UNIX系のシステムで古くから親しまれている、名言、格言、ジョークなどのテキストをランダムに表示するシンプルなコマンドラインツールです。このプロジェクトは、その機能をRustで再実装し、いくつかの拡張機能を加えたものです。

## 目次 (Table of Contents)

- [機能](#機能)
- [インストール](#インストール)
- [使い方](#使い方)
- [技術スタック](#技術スタック)
- [開発](#開発)
    - [テスト](#テスト)
    - [Lint (Clippy)](#lint-clippy)
    - [カバレッジ計測](#カバレッジ計測)

## 機能

- ファイルまたはディレクトリを指定してfortuneを読み込み
- 正規表現によるテキスト検索 (`-m`, `--pattern`)
- 大文字小文字を区別しない検索 (`-i`, `--insensitive`)
- 再現性のためのシード値指定 (`-s`, `--seed`)
- `.dat` ファイルの自動除外
- サブディレクトリの再帰的探索

## インストール

```bash
cargo install --path .
```

## 使い方

```bash
# 基本的な使用法（ファイルまたはディレクトリを指定）
fortuner ./tests/inputs/jokes

# パターン検索（"frog" を含むfortuneを検索）
fortuner ./tests/inputs/jokes -m "frog"

# 大文字小文字を区別せずに検索
fortuner ./tests/inputs -m "yogi" -i

# シード値を指定して実行（常に同じ結果を得る）
fortuner ./tests/inputs/quotes -s 42
```

## 技術スタック

このプロジェクトでは、以下の主要なRustクレートを使用しています。

- **[clap](https://crates.io/crates/clap):** 強力なコマンドライン引数パーサー。
- **[anyhow](https://crates.io/crates/anyhow):** 柔軟で使いやすいエラーハンドリング。
- **[regex](https://crates.io/crates/regex):** 高速な正規表現マッチング。
- **[walkdir](https://crates.io/crates/walkdir):** ディレクトリの再帰的な探索。
- **[rand](https://crates.io/crates/rand):** ランダムな選択とシード値の制御。

## 開発

### テスト

標準の `cargo test` で単体テストと統合テストを実行できます。

```bash
cargo test
```

### Lint (Clippy)

コードの品質チェックには `clippy` を使用しています。

```bash
cargo clippy
```

出力例:
```text
    Checking fortuner v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.xxs
```

### カバレッジ計測

`cargo-llvm-cov` を使用してコードカバレッジを計測しています。
現在のカバレッジは **Lines: 100.00%**, **Functions: 100.00%** です。

実行コマンド:
```bash
cargo llvm-cov --summary-only
```

出力結果:
```text
Filename                                                                     Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
src/main.rs                                                                      266                 1    99.62%          16                 0   100.00%         154                 0   100.00%
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                                            266                 1    99.62%          16                 0   100.00%         154                 0   100.00%
```