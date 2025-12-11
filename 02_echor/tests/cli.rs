use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::process::Command;

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    Command::new(env!("CARGO_BIN_EXE_echor"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// --------------------------------------------------
/// 指定された引数でコマンドを実行し、期待されるファイルの内容と比較する
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::new(env!("CARGO_BIN_EXE_echor"))
        .args(args)
        .output()?;

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout, expected);

    Ok(())
}

// --------------------------------------------------
/// 無効なオプションを渡した場合にエラーになることを確認
#[test]
fn dies_invalid_option() -> Result<()> {
    Command::new(env!("CARGO_BIN_EXE_echor"))
        .args(["-x", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

// --------------------------------------------------
#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

// --------------------------------------------------
#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

// --------------------------------------------------
#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
