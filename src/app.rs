use clap::{crate_version, App, AppSettings, Arg};

/// Build a clap app
pub fn build() -> App<'static> {
    let app = App::new("Zman [A CLI time progress bar]")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .arg(
            Arg::new("time")
                .possible_values(&["year", "month", "week"])
                .hide_possible_values(true)
                .default_value("year")
                .takes_value(true)
                .about("A time to show"),
        )
        .arg(
            Arg::new("json")
                .short('J')
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
                .default_value("\u{2593}")
                .about("Set full bar string")
                .takes_value(true),
        )
        .arg(
            Arg::new("rest_bar")
                .short('r')
                .long("rest-bar")
                .default_value("\u{2591}")
                .about("Set rest bar string")
                .takes_value(true),
        );
    app
}
