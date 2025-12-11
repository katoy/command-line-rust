# false_cmd

Unixの `false` コマンドのRust実装です。

## 概要

`false` コマンドは常に終了コード1（失敗）を返すシンプルなプログラムです。
シェルスクリプトでの条件分岐やテスト用途に使用されます。

## ビルド

```bash
cargo build --release
```

## 実行

```bash
cargo run
echo $?  # 1
```

## テスト

```bash
cargo test
```

## 使用例

```bash
# シェルスクリプトでの使用例
if false_cmd; then
    echo "ここは実行されない"
else
    echo "常にこちらが実行される"
fi
```
