mod progress;
mod util;

use clap::{crate_version, App, AppSettings, Arg};

fn main() {
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
        .arg(Arg::new("month").short('m').long("month"))
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

    let is_json: bool = {
        if matches.is_present("json") {
            true
        } else {
            false
        }
    };

    if matches.is_present("year") {
        show_year_progress(is_json);
    }
    if matches.is_present("month") {
        show_month_progress(is_json);
    }
    if matches.is_present("week") {
        show_week_progress(is_json);
    }
}

fn show_year_progress(is_json: bool) {
    let ratio = progress::year_progress_ratio();
    progress::show_progress(ratio, 20, is_json);
}

fn show_month_progress(is_json: bool) {
    let ratio = progress::month_progress_ratio();
    progress::show_progress(ratio, 20, is_json);
}

fn show_week_progress(is_json: bool) {
    let ratio = progress::week_progress_ratio();
    progress::show_progress(ratio, 20, is_json);
}
