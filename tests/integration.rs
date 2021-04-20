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

#[test]
fn full_bar() {
    let mut cmd = Command::cargo_bin("zman").unwrap();
    cmd.arg("year").arg("--full-bar").arg("▮");
    // \u{25ae} is ▮
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25ae}"));
}

#[test]
fn rest_bar() {
    let mut cmd = Command::cargo_bin("zman").unwrap();
    cmd.arg("year").arg("--rest-bar").arg("▯");
    // \u{25ae} is ▯
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25af}"));
}
