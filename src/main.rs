use std::env;

use zman::app;
use zman::output::Printer;
use zman::progress;

fn run() {
    let matches = app::build().get_matches_from(env::args_os());

    let width = match matches.value_of("width") {
        None => 20,
        Some(num) => num.parse::<i32>().unwrap_or(20),
    };
    let json_format = matches.is_present("json");
    let full_bar = match matches.value_of("full_bar") {
        Some(bar) => bar,
        None => "▓",
    };
    let rest_bar = match matches.value_of("rest_bar") {
        Some(bar) => bar,
        None => "░",
    };

    let mut printer = Printer::new(width, full_bar, rest_bar, json_format);

    if matches.is_present("year") {
        let ratio = progress::year_progress_ratio();
        printer = printer.ratio(ratio);
        printer.print();
    }
    if matches.is_present("month") {
        let ratio = progress::month_progress_ratio();
        printer = printer.ratio(ratio);
        printer.print();
    }
    if matches.is_present("week") {
        let ratio = progress::week_progress_ratio();
        printer = printer.ratio(ratio);
        printer.print();
    }
}

fn main() {
    run()
}
