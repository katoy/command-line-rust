# lsr - Rust版 ls コマンド

Rust による `ls` コマンドの実装です。ファイルやディレクトリの一覧を表示し、ロングフォーマットや隠しファイルの表示などの機能をサポートしています。

## 目次

- [機能](#機能)
- [ビルドと実行](#ビルドと実行)
- [テスト](#テスト)
- [コード品質 (Clippy)](#コード品質-clippy)
- [カバレッジ計測](#カバレッジ計測)

## 機能

- ファイルとディレクトリの一覧表示
- `-l`, `--long`: 詳細情報の表示（パーミッション、リンク数、所有者、グループ、サイズ、更新日時）
- `-a`, `--all`: 隠しファイルを含むすべてのファイルを表示

## ビルドと実行

### ビルド

```bash
cargo build --release
```

### 実行

```bash
cargo run -- [OPTIONS] [PATH]...
```

例:

```bash
# カレントディレクトリの内容を表示
cargo run

# 特定のディレクトリを詳細表示
cargo run -- -l src

# 隠しファイルを含めてすべて表示
cargo run -- -a
```

## テスト

ユニットテストと統合テストを実行します。

```bash
cargo test
```

## コード品質 (Clippy)

Rust のリンターである Clippy を使用して、コードの品質をチェックします。

```bash
cargo clippy
```

## カバレッジ計測

`cargo-llvm-cov` を使用して、コードカバレッジを計測します。
事前に `cargo-llvm-cov` のインストールが必要です。

### インストール

```bash
cargo install cargo-llvm-cov
```

### 計測実行

サマリーのみを表示する場合:

```bash
cargo llvm-cov --summary-only
```

詳細なテキストレポートを表示する場合:

```bash
cargo llvm-cov --text
```

HTMLレポートを生成する場合:

```bash
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は [LICENSE](LICENSE) を参照してください。
