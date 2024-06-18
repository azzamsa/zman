use std::{
    io::{self, Write},
    sync::Arc,
};

use owo_colors::OwoColorize;

use crate::cli::Opts;

pub struct Printer {
    opts: Arc<Opts>,
    ratio: f64,
    /// only used in status bar
    ratio_char: String,
}

impl Printer {
    pub fn new(opts: Arc<Opts>) -> Self {
        Self {
            opts,
            ratio: 0.0,
            ratio_char: "y".to_string(),
        }
    }
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = ratio;
        self
    }
    pub fn ratio_char(mut self, ratio_char: &str) -> Self {
        self.ratio_char = ratio_char.to_string();
        self
    }
    /// Show progress-bar
    pub fn print(&self) {
        let ratio_int = (self.ratio * 100.0) as i32;
        let progress_int = (self.ratio * f64::from(self.opts.width)).round() as i32;
        let rest_int = self.opts.width - progress_int;

        let mut progress_fmt = format!(
            "{}{} {}%",
            self.opts.full_bar.repeat(progress_int as usize),
            self.opts.rest_bar.repeat(rest_int as usize),
            ratio_int
        );
        let state = {
            if ratio_int > 90 {
                "Critical"
            } else {
                "Info"
            }
        };
        // color
        if state == "Critical" && !self.opts.json {
            progress_fmt = format!("{}", progress_fmt.red());
        }
        // JSON
        if self.opts.json {
            progress_fmt = format!(
                r#"{{"icon": "{}", "state": "{}", "text": "{}: {}"}}"#,
                "zman", state, self.ratio_char, progress_fmt
            );
        }
        writeln!(io::stdout(), "{}", progress_fmt).ok();
    }
}
