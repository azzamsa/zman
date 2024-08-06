use std::process::Command;

use anyhow::Result;
use assert_cmd::{crate_name, prelude::*};
use predicates::prelude::*; // Import chrono's prelude to use DateTime<Utc> and Local.

use zman::progress::today;

#[test]
fn help() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;

    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Zman [A CLI time progress bar]"));
    Ok(())
}

#[test]
fn year() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;

    cmd.arg("year");
    cmd.assert().success().stdout(predicate::str::contains("%"));
    Ok(())
}

#[test]
fn year_json() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;

    cmd.arg("year").arg("-J");
    cmd.assert().success().stdout(predicate::str::contains("}"));
    Ok(())
}

#[test]
fn full_bar() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;

    cmd.arg("year").arg("--full-bar").arg("▮");

    // `1` is January
    if today().month() != 1 {
        cmd.assert()
            .success()
            // \u{25ae} is ▮
            .stdout(predicate::str::contains("\u{25ae}"));
    }

    Ok(())
}

#[test]
fn rest_bar() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;

    cmd.arg("year").arg("--rest-bar").arg("▯");

    // `12` is December
    if today().month() != 12 {
        cmd.assert()
            .success()
            // \u{25ae} is ▯
            .stdout(predicate::str::contains("\u{25af}"));
    }
    cmd.assert().success();

    Ok(())
}
