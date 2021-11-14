use clap::{crate_version, App, AppSettings, Arg};

/// Build a clap app
pub fn build() -> App<'static, 'static> {
    let app = App::new("Zman [A CLI time progress bar]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::with_name("time")
                .possible_values(&["year", "month", "week"])
                .hide_possible_values(true)
                .default_value("year")
                .takes_value(true)
                .help("A time to show"),
        )
        .arg(
            Arg::with_name("json")
                .short("J")
                .long("json")
                .help("Display progress in JSON formatted string"),
        )
        .arg(
            Arg::with_name("width")
                .short("d")
                .long("width")
                .help("Adjust width of the bar (default: 20)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("full_bar")
                .short("f")
                .long("full-bar")
                .default_value("\u{2593}")
                .help("Set full bar icon")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rest_bar")
                .short("r")
                .long("rest-bar")
                .default_value("\u{2591}")
                .help("Set rest bar icon")
                .takes_value(true),
        );
    app
}
