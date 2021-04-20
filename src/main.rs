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
    // safe to use unwrap() here. there must be default value
    let full_bar = matches.value_of("full_bar").unwrap();
    let rest_bar = matches.value_of("rest_bar").unwrap();

    let mut printer = Printer::new(width, full_bar, rest_bar, json_format);

    match matches.value_of("time") {
        Some("year") => {
            let ratio = progress::year();
            printer = printer.ratio(ratio).ratio_char("y");
            printer.print();
        }
        Some("month") => {
            let ratio = progress::month();
            printer = printer.ratio(ratio).ratio_char("m");
            printer.print();
        }
        Some("week") => {
            let ratio = progress::week();
            printer = printer.ratio(ratio).ratio_char("w");
            printer.print();
        }
        Some(&_) | None => (),
    }
}

fn main() {
    run()
}
