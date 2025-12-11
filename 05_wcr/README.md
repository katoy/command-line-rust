# Rust版 wc (wcr)

Rust で実装された `wc` コマンド (wcr) です。
コマンドライン引数で指定されたファイル（または標準入力）の行数、単語数、バイト数、文字数をカウントします。

書籍 "Command-Line Rust" (O'Reilly) の第5章の実習コードです。

## 品質ステータス

| チェック項目 | ステータス |
|-------------|-----------|
| clippy | ✅ 警告なし |
| テスト | ✅ 32テスト全パス |
| カバレッジ | ✅ 100% |

## 目次 (Table of Contents)

- [機能](#機能)
- [使用例](#使用例)
- [ファイル構成](#ファイル構成)
- [開発](#開発)
  - [Lint (静的解析)](#lint-静的解析)
  - [テスト](#テスト)
  - [カバレッジ](#カバレッジ)

## 機能

- ファイルまたは標準入力からの読み込み
- 行数 (`-l`, `--lines`), 単語数 (`-w`, `--words`), バイト数 (`-c`, `--bytes`), 文字数 (`-m`, `--chars`) のカウント
- 複数のフラグの同時指定
- 複数ファイルの処理と合計の表示
- UTF-8 文字の正しいカウント

## 使用例

```bash
# 基本的な使用法（行数、単語数、バイト数を表示）
$ cargo run -- tests/inputs/fox.txt
       1       9      48 tests/inputs/fox.txt

# 行数のみ表示
$ cargo run -- -l tests/inputs/fox.txt
       1 tests/inputs/fox.txt

# 文字数を表示（UTF-8対応）
$ cargo run -- -m tests/inputs/utf8.txt
       1 tests/inputs/utf8.txt

# 複数ファイルの処理（合計も表示）
$ cargo run -- tests/inputs/empty.txt tests/inputs/fox.txt
       0       0       0 tests/inputs/empty.txt
       1       9      48 tests/inputs/fox.txt
       1       9      48 total

# 標準入力からの読み込み
$ echo "Hello World" | cargo run
       1       2      12
```

## ファイル構成

```text
.
├── Cargo.lock          # 依存関係のロックファイル
├── Cargo.toml          # 依存関係とプロジェクト設定
├── README.md           # このファイル
├── mk-outs.sh          # テスト用の期待値を生成するスクリプト
├── src
│   └── main.rs         # メインのソースコード（Args, FileInfo, count, open, format_field）
└── tests
    ├── cli.rs          # 統合テスト（29テスト）
    ├── expected        # テストの期待値ファイル群
    │   ├── *.out       # 各入力ファイルに対する期待出力
    │   └── all.*.out   # 複数ファイル処理時の期待出力
    └── inputs          # テストの入力ファイル群
        ├── empty.txt   # 空ファイル
        ├── fox.txt     # 英語テキスト（1行）
        ├── atlamal.txt # 複数行テキスト
        └── utf8.txt    # UTF-8マルチバイト文字テスト用
```

## 開発

### Lint (静的解析)

コードの品質チェックには `cargo clippy` を使用します。警告をエラーとして扱う場合:

```bash
cargo clippy --all-targets -- -D warnings
```

### テスト

ユニットテスト（3テスト）と統合テスト（29テスト）を実行します。

```bash
cargo test
```

#### テスト内容

- **ユニットテスト** (`src/main.rs`):
  - `test_count`: `count` 関数のテスト
  - `test_format_field`: `format_field` 関数のテスト
  - `test_count_error`: I/Oエラー時の動作テスト

- **統合テスト** (`tests/cli.rs`):
  - 各オプション (`-l`, `-w`, `-c`, `-m`) の単独・組み合わせテスト
  - 複数ファイル処理と合計表示のテスト
  - 標準入力からの読み込みテスト
  - 存在しないファイルのエラー処理テスト
  - UTF-8文字のカウントテスト

### カバレッジ

`cargo-llvm-cov` を使用して、コードカバレッジを計測します。
現在 **100%** を達成しています。

1. **ツールのインストール** (未インストールの場合):

   ```bash
   cargo install cargo-llvm-cov
   ```

2. **カバレッジの計測**:

   ```bash
   # サマリーのみ表示
   cargo llvm-cov --summary-only

   # 詳細なテキスト出力
   cargo llvm-cov --text
   ```

3. **HTMLレポートの生成**:

   ```bash
   cargo llvm-cov --html --open
   ```
