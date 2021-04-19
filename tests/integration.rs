use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("zman").unwrap();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Zman [A CLI time progress bar]"));
}

#[test]
fn year() {
    let mut cmd = Command::cargo_bin("zman").unwrap();
    cmd.arg("year");
    cmd.assert().success().stdout(predicate::str::contains("%"));
}

#[test]
fn year_json() {
    let mut cmd = Command::cargo_bin("zman").unwrap();
    cmd.arg("year").arg("-J");
    cmd.assert().success().stdout(predicate::str::contains("}"));
}
