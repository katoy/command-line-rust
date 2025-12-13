#![allow(deprecated)]
use assert_cmd::Command;

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    // falseコマンドは非ゼロの終了コードを返すべきです。
    // 現在の実装は std::process::abort() なので、失敗することが期待されます。
    cmd.assert().failure();
}
