use clap::Parser;

use zman::cli::Opts;
use zman::cli::Time;
use zman::output::Printer;
use zman::progress;

fn run() {
    let opts = Opts::parse();

    let mut printer = Printer::new(opts.width, &opts.full_bar, &opts.rest_bar, opts.json);
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
