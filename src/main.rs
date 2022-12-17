use clap::Parser;

use zman::cli::Opts;
use zman::cli::Time;
use zman::output::Printer;
use zman::progress;

fn run() {
    let opts = Opts::parse();

    let width = opts.width;
    let json_format = opts.json;
    // safe to use unwrap() here. there must be default value
    let full_bar = opts.full_bar;
    let rest_bar = opts.rest_bar;

    let mut printer = Printer::new(width, &full_bar, &rest_bar, json_format);

    match opts.time {
        Time::Year => {
            let ratio = progress::year();
            printer = printer.ratio(ratio).ratio_char("y");
            printer.print();
        }
        Time::Month => {
            let ratio = progress::month();
            printer = printer.ratio(ratio).ratio_char("m");
            printer.print();
        }
        Time::Week => {
            let ratio = progress::week();
            printer = printer.ratio(ratio).ratio_char("w");
            printer.print();
        }
    }
}

fn main() {
    run();
}
