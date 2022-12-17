use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<()> {
    let mut cmd = Command::cargo_bin("zman")?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Zman [A CLI time progress bar]"));
    Ok(())
}

#[test]
fn year() -> Result<()> {
    let mut cmd = Command::cargo_bin("zman")?;
    cmd.arg("year");
    cmd.assert().success().stdout(predicate::str::contains("%"));
    Ok(())
}

#[test]
fn year_json() -> Result<()> {
    let mut cmd = Command::cargo_bin("zman")?;
    cmd.arg("year").arg("-J");
    cmd.assert().success().stdout(predicate::str::contains("}"));
    Ok(())
}

#[test]
fn full_bar() -> Result<()> {
    let mut cmd = Command::cargo_bin("zman")?;
    cmd.arg("year").arg("--full-bar").arg("▮");
    // \u{25ae} is ▮
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25ae}"));
    Ok(())
}

#[test]
fn rest_bar() -> Result<()> {
    let mut cmd = Command::cargo_bin("zman")?;
    cmd.arg("year").arg("--rest-bar").arg("▯");
    // \u{25ae} is ▯
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25af}"));
    Ok(())
}
