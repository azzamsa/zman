mod progress;

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
}


fn show_year_progress() {
    progress::show_progress(20)
}
