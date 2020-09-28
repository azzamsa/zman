mod config;
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
        .arg(
            Arg::new("month")
                .short('m')
                .long("month")
                .about("Show current month progress"),
        )
        .arg(
            Arg::new("week")
                .short('w')
                .long("week")
                .about("Show current week progress"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .about("Display progress in JSON formatted string"),
        )
        .arg(
            Arg::new("width")
                .short('d')
                .long("width")
                .about("Adjust width of the bar (default: 20)")
                .takes_value(true),
        )
        .arg(
            Arg::new("full_bar")
                .short('f')
                .long("full-bar")
                .about("Set full bar string (default: ▓)")
                .takes_value(true),
        )
        .arg(
            Arg::new("rest_bar")
                .short('r')
                .long("rest-bar")
                .about("Set rest bar string (default: ░)")
                .takes_value(true),
        )
        .get_matches();

    let config = config::Config {
        width: match matches.value_of("width") {
            None => 20,
            Some(num) => num.parse::<i32>().unwrap_or(20),
        },
        is_json: {
            if matches.is_present("json") {
                true
            } else {
                false
            }
        },
        full_bar: match matches.value_of("full_bar") {
            None => "▓".to_string(),
            Some(bar) => bar.parse::<String>().unwrap_or("▓".to_string()),
        },
        rest_bar: match matches.value_of("rest_bar") {
            None => "░".to_string(),
            Some(bar) => bar.parse::<String>().unwrap_or("░".to_string()),
        },
    };

    if matches.is_present("year") {
        show_year_progress(config.clone());
    }
    if matches.is_present("month") {
        show_month_progress(config.clone());
    }
    if matches.is_present("week") {
        show_week_progress(config.clone());
    }
}

fn show_year_progress(config: config::Config) {
    let ratio = progress::year_progress_ratio();
    progress::show_progress(ratio, config);
}

fn show_month_progress(config: config::Config) {
    let ratio = progress::month_progress_ratio();
    progress::show_progress(ratio, config);
}

fn show_week_progress(config: config::Config) {
    let ratio = progress::week_progress_ratio();
    progress::show_progress(ratio, config);
}
