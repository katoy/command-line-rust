# ascii

ASCII コード表（33〜127）を表示する CLI ツールです。

## 目次

- [概要](#概要)
- [使用方法](#使用方法)
- [開発](#開発)
    - [Lint](#lint)
    - [テスト & カバレッジ](#テスト--カバレッジ)

## 概要

このツールは、ASCII コード（33〜127）とその対応する文字を、見やすい表形式で標準出力に表示します。

## 使用方法

Cargo を使用して実行します。

```bash
cargo run
```

出力例:

```text
 33: !   52: 4   71: G   90: Z  109: m
 34: "   53: 5   72: H   91: [  110: n
 ...
```

## 開発

### Lint

コードの静的解析には `clippy` を使用します。

```bash
cargo clippy
```

### テスト & カバレッジ

まず、`cargo-llvm-cov` をインストールします。

```bash
cargo install cargo-llvm-cov
```

テストを実行し、カバレッジを計測するには `cargo-llvm-cov` を使用します。

```bash
# テストの実行
cargo test

# カバレッジの計測（サマリーのみ表示）
cargo llvm-cov --summary-only
```
