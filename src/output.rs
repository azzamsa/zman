use std::io::{self, Write};

use colored::Colorize;

pub struct Printer {
    ratio: f64,
    width: i32,
    full_bar: String,
    rest_bar: String,
    json_format: bool,
}

impl Printer {
    pub fn new(width: i32, full_bar: &str, rest_bar: &str, json_format: bool) -> Self {
        Self {
            ratio: 0.0,
            width,
            full_bar: full_bar.to_string(),
            rest_bar: rest_bar.to_string(),
            json_format,
        }
    }
    pub fn ratio(mut self, ratio: f64) -> Self {
        self.ratio = ratio;
        self
    }
    /// Show progress-bar
    pub fn print(&self) {
        let ratio_int = (self.ratio * 100.0) as i32;
        let progress_int = (self.ratio * f64::from(self.width)).round() as i32;
        let rest_int = self.width - progress_int;

        let mut progress_fmt = format!(
            "{}{} {}%",
            self.full_bar.repeat(progress_int as usize),
            self.rest_bar.repeat(rest_int as usize),
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
        if state == "Critical" && !self.json_format {
            progress_fmt = format!("{}", progress_fmt.red());
        }
        // JSON
        if self.json_format {
            progress_fmt = format!(r#"{{"state":"{}", "text": "{}"}}"#, state, progress_fmt);
        }
        writeln!(io::stdout(), "{}", progress_fmt).ok();
    }
}
