use clap::Parser;
use std::{process, sync::Arc};

use anyhow::Result;
use jiff::Zoned;

use zman::{
    cli::{Opts, Period},
    output::Printer,
    progress,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let opts = Arc::new(Opts::parse());

    let today = Zoned::now().date();
    let mut printer = Printer::new(Arc::clone(&opts));
    match opts.period {
        Period::Year => {
            let ratio = progress::year(today)?;
            printer = printer.ratio(ratio).ratio_char("y");
            printer.print();
        }
        Period::Month => {
            let ratio = progress::month(today)?;
            printer = printer.ratio(ratio).ratio_char("m");
            printer.print();
        }
        Period::Week => {
            let ratio = progress::week(today)?;
            printer = printer.ratio(ratio).ratio_char("w");
            printer.print();
        }
    }

    Ok(())
}
