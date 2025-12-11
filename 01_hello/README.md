# micro:bit v2 で Rust - LED 点滅プログラム

Rust を使用して micro:bit v2 で LED を点滅させる「Hello, World!」プログラムです。

## 目次

- [必要なもの](#必要なもの)
- [環境構築](#環境構築)
  - [1. Rust のインストール](#1-rust-のインストール)
  - [2. ターゲットの追加](#2-ターゲットの追加)
  - [3. probe-rs のインストール](#3-probe-rs-のインストール)
- [プロジェクトの作成](#プロジェクトの作成)
  - [1. 新規プロジェクト作成](#1-新規プロジェクト作成)
  - [2. Cargo.toml の設定](#2-cargotoml-の設定)
  - [3. .cargo/config.toml の作成](#3-cargoconfgtoml-の作成)
  - [4. memory.x の作成](#4-memoryx-の作成)
  - [5. build.rs の作成](#5-buildrs-の作成)
  - [6. src/main.rs の作成](#6-srcmainrs-の作成)
- [ビルドと実行](#ビルドと実行)
- [トラブルシューティング](#トラブルシューティング)
- [ファイル構成](#ファイル構成)
- [参考資料](#参考資料)

---

## 必要なもの

- **ハードウェア**
  - micro:bit v2（nRF52833搭載）
  - USB ケーブル（micro USB）

- **ソフトウェア**
  - Rust（nightly 推奨）
  - probe-rs（フラッシュ書き込みツール）

---

## 環境構築

### 1. Rust のインストール

```bash
# rustup がインストールされていない場合
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# nightly ツールチェーンをインストール
rustup install nightly
rustup default nightly
```

### 2. ターゲットの追加

micro:bit v2 は ARM Cortex-M4F プロセッサを使用しています。

```bash
rustup target add thumbv7em-none-eabihf
```

### 3. probe-rs のインストール

micro:bit へのプログラム書き込みに使用します。

```bash
cargo install probe-rs-tools --locked
```

> **注意**: インストール後、新しいターミナルを開くか `source ~/.zshrc` を実行してパスを反映させてください。

---

## プロジェクトの作成

### 1. 新規プロジェクト作成

```bash
cargo new hello --bin
cd hello
```

### 2. Cargo.toml の設定

`Cargo.toml` を以下の内容に置き換えます：

```toml
[package]
name = "hello"
version = "0.1.1"
edition = "2021"

[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
panic-halt = "0.2"
microbit-v2 = "0.15"
embedded-hal = "1.0"

[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
debug = false
```

**依存クレートの説明:**

| クレート | 説明 |
|---------|------|
| `cortex-m` | ARM Cortex-M プロセッサのサポート |
| `cortex-m-rt` | ランタイム（エントリポイント、割り込みハンドラ） |
| `panic-halt` | パニック時に無限ループで停止 |
| `microbit-v2` | micro:bit v2 のボードサポート |
| `embedded-hal` | 組み込み HAL（抽象化レイヤー） |

### 3. .cargo/config.toml の作成

`.cargo/config.toml` を作成します：

```bash
mkdir -p .cargo
```

`.cargo/config.toml`:

```toml
# micro:bit v2 ターゲット設定
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
# probe-rs を使用してフラッシュとデバッグ
runner = "probe-rs run --chip nRF52833_xxAA"

rustflags = [
    "-C", "link-arg=-Tlink.x",
]
```

### 4. memory.x の作成

プロジェクトルートに `memory.x` を作成します：

```ld
/* micro:bit v2 (nRF52833) メモリレイアウト */
MEMORY
{
  /* フラッシュメモリ: 512KB */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  /* RAM: 128KB */
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```

### 5. build.rs の作成

プロジェクトルートに `build.rs` を作成します：

```rust
//! ビルドスクリプト - memory.x をリンカに認識させる

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
```

### 6. src/main.rs の作成

`src/main.rs` を以下の内容に置き換えます：

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_halt as _;

/// micro:bit v2 で LED を点滅させる "Hello, World!" プログラム
/// 
/// micro:bit v2 の 5x5 LED マトリクスは行と列で制御される:
/// - ROW を HIGH にして電流のソースにする
/// - COL を LOW にして電流のシンクにする
/// - ROW=HIGH かつ COL=LOW のとき LED が点灯
#[entry]
fn main() -> ! {
    // ボードの初期化
    let board = Board::take().unwrap();
    
    // タイマーの初期化
    let mut timer = Timer::new(board.TIMER0);
    
    // LED マトリクスの行1と列1を取得
    let mut row1 = board
        .display_pins
        .row1
        .into_push_pull_output(microbit::hal::gpio::Level::High);
    let mut col1 = board
        .display_pins
        .col1
        .into_push_pull_output(microbit::hal::gpio::Level::Low);
    
    // LED を点滅（無限ループ）
    loop {
        // LED ON: ROW=HIGH, COL=LOW
        row1.set_high().unwrap();
        col1.set_low().unwrap();
        timer.delay_ms(500u32);
        
        // LED OFF: COL=HIGH にして電流を止める
        col1.set_high().unwrap();
        timer.delay_ms(500u32);
    }
}
```

**コードの説明:**

| 属性/マクロ | 説明 |
|------------|------|
| `#![no_std]` | 標準ライブラリを使用しない（OS なし環境） |
| `#![no_main]` | 通常の main 関数を使用しない |
| `#[entry]` | 組み込み用エントリポイント |
| `-> !` | この関数は戻らない（無限ループ） |

---

## ビルドと実行

### 1. ビルド

```bash
cargo build --release
```

### 2. micro:bit への転送

micro:bit を USB で接続して実行：

```bash
cargo run --release
```

または：

```bash
probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/release/hello
```

成功すると、以下のような出力が表示されます：

```
Erasing ✔ 100% [####################]   4.00 KiB @  20.67 KiB/s
Programming ✔ 100% [####################]   4.00 KiB @  15.10 KiB/s
Finished in 0.56s
```

**左上の LED が 0.5 秒間隔で点滅します！** 🔴⚫🔴⚫

---

## トラブルシューティング

### probe-rs: command not found

パスが通っていません。以下を試してください：

```bash
# 新しいターミナルを開くか
source ~/.zshrc

# または絶対パスで実行
~/.cargo/bin/probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/release/hello
```

### No probe detected

micro:bit が接続されていないか、認識されていません：

1. USB ケーブルを確認（データ転送対応のケーブルを使用）
2. micro:bit を抜き差し
3. 別の USB ポートを試す

### Target device did not respond

micro:bit が応答しない状態です：

1. micro:bit 背面のリセットボタンを押す
2. USB ケーブルを抜き差し
3. 再度転送を実行

### No loadable segments were found in the ELF file

`memory.x` と `build.rs` が正しく設定されていません：

1. プロジェクトルートに `memory.x` があるか確認
2. `build.rs` があるか確認
3. `cargo clean && cargo build --release` を実行

---

## ファイル構成

```
hello/
├── .cargo/
│   └── config.toml    # ターゲットとランナー設定
├── src/
│   └── main.rs        # メインプログラム（LED点滅）
├── Cargo.toml         # 依存関係と最適化設定
├── build.rs           # ビルドスクリプト
├── memory.x           # メモリレイアウト定義
└── README.md          # このファイル
```

---

## 参考資料

- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [microbit-v2 crate](https://docs.rs/microbit-v2/)
- [probe-rs](https://probe.rs/)
- [nRF52833 Datasheet](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf)

---

## ライセンス

MIT License
