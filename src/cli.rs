use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "zman",
    version,
    about = "Zman [A CLI time progress bar]",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/zman/issues"
)]
pub struct Opts {
    /// A period to display
    #[arg(value_enum)]
    pub period: Period,

    /// Display progress in JSON formatted string
    #[arg(short = 'J', long)]
    pub json: bool,

    /// Adjust width of the bar
    #[arg(short, long, default_value_t = 20)]
    pub width: i32,

    /// Set full bar icon
    #[arg(short, long, default_value = "▓")]
    pub full_bar: String,

    /// Set rest bar icon
    #[arg(short, long, default_value = "░")]
    pub rest_bar: String,
}

#[derive(Clone, ValueEnum)]
pub enum Period {
    /// Year
    Year,
    /// Month
    Month,
    /// Week
    Week,
}
