
use assert_cmd::cargo::cargo_bin_cmd;
use pretty_assertions::assert_eq;
use std::fs;


const EXPECTED_FILE: &str = "./tests/expected/ascii.txt";

// --------------------------------------------------
#[test]
fn runs() {
    let expected = fs::read_to_string(EXPECTED_FILE).unwrap();
    let output = cargo_bin_cmd!("ascii").output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
}
