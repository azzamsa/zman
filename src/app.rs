use clap::{crate_version, App, AppSettings, Arg};

/// Build a clap app
pub fn build() -> App<'static> {
    let app = App::new("Zman [A CLI time progress bar]")
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
                .about("Set full bar string (default: ▓)")
                .takes_value(true),
        )
        .arg(
            Arg::new("rest_bar")
                .short('r')
                .long("rest-bar")
                .about("Set rest bar string (default: ░)")
                .takes_value(true),
        );
    app
}
