use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_default() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay").expect("binary exists").assert().success().stdout(predicate::str::contains("Meow!"));

    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay").expect("binary exists").args(&["-f", "no/such/file.txt"]).assert().failure();

    Ok(())
}