# Biggie

指定された行数のランダムなテキストを含むファイルを生成するプログラムです。
他のプログラムをテストするために、非常に大きなファイルを作成することを目的としています。

## 目次

- [使い方](#使い方)
- [オプション](#オプション)
- [インストール](#インストール)
- [著者](#著者)

## 使い方

`cargo run` で実行できます。`-h` または `--help` で使用法を表示できます。

```bash
$ cargo run -- --help
```

基本的な実行例:

```bash
# デフォルト設定で実行 (100,000行、out.txtに出力)
cargo run

# 行数を指定して実行
cargo run -- --lines 500

# 出力ファイル名を指定して実行
cargo run -- --outfile mydata.txt

# 両方を指定
cargo run -- -l 50 -o test.txt
```

## オプション

| ショート | ロング | 値 | デフォルト | 説明 |
| :--- | :--- | :--- | :--- | :--- |
| `-o` | `--outfile` | FILE | `out.txt` | 出力ファイル名 |
| `-l` | `--lines` | LINES | `100000` | 生成する行数 (1以上) |

## インストール

ソースコードからビルドする場合:

```bash
git clone https://github.com/study-rust/command-line-rust.git
cd command-line-rust/util/biggie
cargo build --release
```

## 著者

Ken Youens-Clark <kyclark@gmail.com>
