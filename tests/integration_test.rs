use assert_cmd::prelude::*;
use std::error::Error;
use std::process::Command;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn run_with_default() -> TestResult {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicates::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_nonexisting_file() -> TestResult {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}
