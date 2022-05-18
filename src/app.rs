use clap::{crate_version, Arg, Command};

/// Build a clap app
pub fn build() -> Command<'static> {
    let app = Command::new("Zman [A CLI time progress bar]")
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("time")
                .possible_values(&["year", "month", "week"])
                .hide_possible_values(true)
                .default_value("year")
                .takes_value(true)
                .help("A time to show"),
        )
        .arg(
            Arg::new("json")
                .short('J')
                .long("json")
                .help("Display progress in JSON formatted string"),
        )
        .arg(
            Arg::new("width")
                .short('d')
                .long("width")
                .help("Adjust width of the bar (default: 20)")
                .takes_value(true),
        )
        .arg(
            Arg::new("full_bar")
                .short('f')
                .long("full-bar")
                .default_value("\u{2593}")
                .help("Set full bar icon")
                .takes_value(true),
        )
        .arg(
            Arg::new("rest_bar")
                .short('r')
                .long("rest-bar")
                .default_value("\u{2591}")
                .help("Set rest bar icon")
                .takes_value(true),
        );
    app
}
