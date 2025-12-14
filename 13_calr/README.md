# calr: Rust製カレンダーコマンドラインツール

`calr`は、Unixの`cal`コマンドにインスパイアされた、Rustで書かれたシンプルなカレンダー表示コマンドラインツールです。

## 目次
- [概要](#概要)
- [機能](#機能)
- [インストール](#インストール)
- [使用方法](#使用方法)
- [開発](#開発)
  - [コードの整形と検査 (clippy)](#コードの整形と検査-clippy)
  - [テスト](#テスト)
  - [カバレッジ計測](#カバレッジ計測)

## 概要
`calr`は、指定された月や年のカレンダーをターミナルに表示します。現在の月や年を表示するだけでなく、特定の月や年のカレンダーも表示できます。

## 機能
- 現在の月のカレンダー表示
- 特定の月（数値または略称）のカレンダー表示
- 特定の年のカレンダー表示
- 全年のカレンダー表示
- 今日の日付のハイライト表示

## インストール
Cargoがインストールされている環境であれば、以下のコマンドでインストールできます。

```bash
cargo install --path .
```

## 使用方法
### 基本的な使用方法
現在の月のカレンダーを表示します。

```bash
calr
```

### 特定の月のカレンダーを表示
月の数値（1-12）または月名の略称（例: `jan`, `feb`）を指定します。

```bash
calr -m 3
calr -m mar
```

### 特定の年のカレンダーを表示
年の数値を指定します。

```bash
calr 2024
```

### 特定の月の特定年のカレンダーを表示
月と年の両方を指定します。

```bash
calr -m 7 2024
```

### 全年のカレンダーを表示
`-y`または`--year`オプションを使用します。

```bash
calr -y
```

## 開発

### コードの整形と検査 (clippy)
コードの品質と慣用的なRustの書き方をチェックします。

```bash
cargo clippy
```

**現在の `cargo clippy` の結果:**
```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
```

### テスト
プロジェクトのテストを実行します。

```bash
cargo test
```

**現在の `cargo test` の結果:**
```text
   Compiling anstyle v1.0.13
   Compiling memchr v2.7.6
   Compiling utf8parse v0.2.2
   Compiling regex-syntax v0.8.8
   Compiling is_terminal_polyfill v1.70.2
   Compiling num-traits v0.2.19
   Compiling colorchoice v1.0.4
   Compiling libc v0.2.178
   Compiling anstyle-parse v0.2.7
   Compiling anstyle-query v1.1.5
   Compiling core-foundation-sys v0.8.7
   Compiling anstream v0.6.21
   Compiling clap_lex v0.7.6
   Compiling strsim v0.11.1
   Compiling aho-corasick v1.1.4
   Compiling predicates-core v1.0.9
   Compiling float-cmp v0.10.0
   Compiling clap_builder v4.5.53
   Compiling iana-time-zone v0.1.64
   Compiling either v1.15.0
   Compiling difflib v0.4.0
   Compiling termtree v0.5.1
   Compiling normalize-line-endings v0.3.0
   Compiling assert_cmd v2.1.1
   Compiling predicates-tree v1.0.12
   Compiling itertools v0.14.0
   Compiling chrono v0.4.42
   Compiling anyhow v1.0.100
   Compiling regex-automata v0.4.13
   Compiling wait-timeout v0.2.1
   Compiling ansi_term v0.12.1
   Compiling diff v0.1.13
   Compiling yansi v1.0.1
   Compiling clap v4.5.53
   Compiling pretty_assertions v1.4.1
   Compiling calr v0.1.0 (/Users/katoy/github/study-rust/command-line-rust/13_calr)
   Compiling regex v1.12.2
   Compiling bstr v1.12.1
   Compiling predicates v3.1.3
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.81s
     Running unittests src/main.rs (target/debug/deps/calr-19353cb1e282fffb)

running 3 tests
test tests::test_last_day_in_month ... ok
test tests::test_parse_month ... ok
test tests::test_format_month ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/debug/deps/cli-9354cfce12b2761b)

running 16 tests
test default_one_month ... ok
test dies_invalid_year ... ok
test dies_year_0 ... ok
test dies_y_and_year ... ok
test dies_month_0 ... ok
test dies_y_and_month ... ok
test dies_invalid_month ... ok
test dies_month_13 ... ok
test dies_year_10000 ... ok
test test_2020 ... ok
test test_4_2020 ... ok
test test_2_2020_leap_year ... ok
test year ... ok
test test_april_2020 ... ok
test partial_month ... ok
test month_num ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.62s
```

### カバレッジ計測
コードのカバレッジを計測します。

```bash
cargo llvm-cov --summary-only
```

**現在の `cargo llvm-cov --summary-only` の結果:**
```text
info: cargo-llvm-cov currently setting cfg(coverage) and cfg(coverage_nightly); you can opt-out it by passing --no-cfg-coverage and --no-cfg-coverage-nightly
   Compiling calr v0.1.0 (/Users/katoy/github/study-rust/command-line-rust/13_calr)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running unittests src/main.rs (target/llvm-cov-target/debug/deps/calr-47b41a36d6cc1475)

running 3 tests
test tests::test_last_day_in_month ... ok
test tests::test_parse_month ... ok
test tests::test_format_month ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/llvm-cov-target/debug/deps/cli-857f0e886e76cf60)

running 16 tests
test dies_month_0 ... ok
test dies_invalid_year ... ok
test dies_year_0 ... ok
test default_one_month ... ok
test dies_invalid_month ... ok
test dies_y_and_year ... ok
test dies_y_and_month ... ok
test dies_month_13 ... ok
test dies_year_10000 ... ok
test test_april_2020 ... ok
test test_2_2020_leap_year ... ok
test test_2020 ... ok
test test_4_2020 ... ok
test year ... ok
test partial_month ... ok
test month_num ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s

Filename                                                                 Regions    Missed Regions     Cover   Functions  Missed 
Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
---------------------------------------------------------------------------------------------------------------------------------
---------------------------------------------------------------------------------------------------
/Users/katoy/github/study-rust/command-line-rust/13_calr/src/main.rs         309                 0   100.00%          13         
        0   100.00%         178                 0   100.00%           0                 0         -
---------------------------------------------------------------------------------------------------------------------------------
---------------------------------------------------------------------------------------------------
TOTAL                                                                        309                 0   100.00%          13         
        0   100.00%         178                 0   100.00%           0                 0         -
```
