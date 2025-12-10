use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn runs() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("hello"));
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("true"));
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("false"));
    cmd.assert().failure();
}
