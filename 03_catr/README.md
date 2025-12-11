# catr - Rust implementation of `cat`

「Command Line Rust」の第3章で作成する `cat` コマンドの Rust 実装です。
標準の `cat` コマンドの主要な機能をサポートしています。

## 目次

- [catr - Rust implementation of `cat`](#catr---rust-implementation-of-cat)
  - [目次](#目次)
  - [概要](#概要)
  - [機能](#機能)
  - [ファイル構成](#ファイル構成)
  - [インストールと実行](#インストールと実行)
  - [オプション](#オプション)
  - [標準 cat コマンドとの違い](#標準-cat-コマンドとの違い)
  - [開発](#開発)
    - [Lint](#lint)
    - [Format](#format)
    - [Test](#test)
    - [Coverage](#coverage)
  - [ライセンス](#ライセンス)

## 概要

`catr` は、ファイルの内容を標準出力に連結して表示するコマンドラインツールです。
標準入力からの読み込みや、行番号の表示などのオプションをサポートしています。

## 機能

- ファイルの内容を標準出力に表示
- 標準入力からの読み込み (`-`)
- 全行に行番号を付加 (`-n`, `--number`)
- 非空行のみに行番号を付加 (`-b`, `--number-nonblank`)
- 複数ファイルの連結表示

## ファイル構成

```text
03_catr/
├── Cargo.toml          # プロジェクト設定（依存関係、メタデータ）
├── src/
│   └── main.rs         # メインロジック（引数解析、ファイル読み込み、出力処理）
├── tests/
│   ├── cli.rs          # 統合テスト（22テストケース）
│   ├── inputs/         # テスト用入力ファイル
│   │   ├── empty.txt   # 空ファイル
│   │   ├── fox.txt     # 1行ファイル
│   │   ├── spiders.txt # 複数行ファイル（空行含む）
│   │   └── the-bustle.txt  # 詩のテキスト
│   └── expected/       # 期待される出力ファイル（18ファイル）
└── README.md           # このファイル
```

## インストールと実行

Rust がインストールされている環境で、以下のコマンドで実行できます。

```bash
# ビルド
cargo build --release

# ヘルプの表示
cargo run -- --help

# バージョンの表示
cargo run -- --version

# ファイルの表示
cargo run -- tests/inputs/fox.txt

# 行番号を表示 (-n)
cargo run -- -n tests/inputs/fox.txt

# 非空行のみ行番号を表示 (-b)
cargo run -- -b tests/inputs/the-bustle.txt

# 標準入力からの読み込み
echo "hello" | cargo run -- -

# 複数ファイルの連結
cargo run -- tests/inputs/fox.txt tests/inputs/spiders.txt
```

## オプション

| オプション | 短縮形 | 説明 |
|------------|--------|------|
| `--help` | `-h` | ヘルプを表示 |
| `--version` | `-V` | バージョンを表示 |
| `--number` | `-n` | 全ての行に行番号を付加 |
| `--number-nonblank` | `-b` | 非空行のみに行番号を付加 |

> **注意**: `-n` と `-b` は同時に指定できません（排他的オプション）。

## 標準 cat コマンドとの違い

本実装 (`catr`) と標準的な `cat` コマンド (GNU cat や BSD cat) には、以下の仕様上の差異があります。

| 項目 | catr | 標準 cat |
|------|------|----------|
| 行番号 | ファイルごとにリセット | 通し番号 |

この挙動は、本プロジェクトの仕様（およびテストケース）に基づくものです。

## 開発

### Lint

コードの静的解析には `clippy` を使用します。

```bash
cargo clippy -- -D warnings
```

### Format

コードのフォーマットチェックと整形には `rustfmt` を使用します。

```bash
# チェックのみ
cargo fmt -- --check

# 整形を適用
cargo fmt
```

### Test

テストの実行は標準の `cargo test` を使用します。

```bash
cargo test
```

**テストカバレッジ:**

- 22 テストケース
- ヘルプ表示 (`-h`, `--help`)
- バージョン表示 (`-V`, `--version`)
- 排他オプションエラー (`-n -b` 同時指定)
- 存在しないファイルのエラー処理
- 空ファイル、1行、複数行ファイル
- 標準入力からの読み込み
- 複数ファイルの連結

### Coverage

カバレッジ計測を行う場合は、`cargo-llvm-cov` を使用します。

```bash
# インストール（初回のみ）
cargo install cargo-llvm-cov

# カバレッジ計測
cargo llvm-cov

# HTML レポート生成
cargo llvm-cov --html
```

## ライセンス

MIT License
