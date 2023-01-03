use std::process::Command;

use anyhow::Result;
use assert_cmd::{crate_name, prelude::*};
use predicates::prelude::*;

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
    let today = time::OffsetDateTime::now_local()?.date();

    cmd.arg("year").arg("--full-bar").arg("▮");
    // \u{25ae} is ▮
    if today.month() != time::Month::January {
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("\u{25ae}"));
    }

    Ok(())
}

#[test]
fn rest_bar() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    let today = time::OffsetDateTime::now_local()?.date();

    cmd.arg("year").arg("--rest-bar").arg("▯");
    // \u{25ae} is ▯
    if today.month() != time::Month::December {
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("\u{25af}"));
    }
    cmd.assert().success();

    Ok(())
}
