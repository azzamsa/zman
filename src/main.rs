mod progress;
mod util;

use clap::{crate_version, App, AppSettings, Arg};

fn main() {
    let mut _is_json: bool = false;
    let matches = App::new("Jaro [A CLI time progress bar]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .about("Show current year progress"),
        )
        .arg(
            Arg::new("month")
                .short('m')
                .long("month")
                .about("Show current month progress"),
        )
        .arg(
            Arg::new("week")
                .short('w')
                .long("wek")
                .about("Show current week progress"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .about("Display progress in JSON formatted string"),
        )
        .get_matches();

    if matches.is_present("json") {
        _is_json = true;
    } else {
        _is_json = false;
    }

    if matches.is_present("year") {
        show_year_progress();
    }
    if matches.is_present("month") {
        show_month_progress();
    }
}

fn show_year_progress() {
    let ratio = progress::year_progress_ratio();
    progress::show_progress(ratio, 20);
}

fn show_month_progress() {
    let ratio = progress::month_progress_ratio();
    progress::show_progress(ratio, 20);
}
