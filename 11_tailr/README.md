# tailr

`tail`コマンドのRust実装。

## 目次

- [使い方](#使い方)
- [開発](#開発)
  - [ビルド](#ビルド)
  - [テスト](#テスト)
  - [Lint (Clippy)](#lint-clippy)
  - [カバレッジ](#カバレッジ)

## 使い方

```bash
cargo run -- [OPTIONS] <FILES>...
```

詳細については、以下を実行してください:

```bash
cargo run -- --help
```

## 開発

### ビルド

プロジェクトをビルドするには:

```bash
cargo build
```

### テスト

すべてのテストを実行するには:

```bash
cargo test
```

### Lint (Clippy)

Lintエラーと警告をチェックするためにClippyを実行します:

```bash
cargo clippy
```

### カバレッジ

コードカバレッジを測定するには、`cargo-llvm-cov`を使用します。

まず、`cargo-llvm-cov`をまだインストールしていない場合はインストールします:

```bash
cargo install cargo-llvm-cov
```

次に、カバレッジコマンドを実行します:

```bash
cargo llvm-cov --summary-only
```

詳細なHTMLレポートについては:

```bash
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```