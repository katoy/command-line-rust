use std::process::Command;

#[test]
fn runs() {
    let bin_path = assert_cmd::cargo::cargo_bin!("false_cmd");
    let mut cmd = Command::new(bin_path);
    let output = cmd.output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn returns_exit_code_one() {
    let bin_path = assert_cmd::cargo::cargo_bin!("false_cmd");
    let mut cmd = Command::new(bin_path);
    let output = cmd.output().unwrap();
    assert_eq!(output.status.code(), Some(1));
}
