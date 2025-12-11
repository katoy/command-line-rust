# true_cmd

Unixの `true` コマンドのRust実装です。

## 概要

`true` コマンドは常に終了コード0（成功）を返すシンプルなプログラムです。
シェルスクリプトでの条件分岐やテスト用途に使用されます。

## ビルド

```bash
cargo build --release
```

## 実行

```bash
cargo run
echo $?  # 0
```

## テスト

```bash
cargo test
```

## 使用例

```bash
# シェルスクリプトでの使用例
if true_cmd; then
    echo "常にこちらが実行される"
fi
```
