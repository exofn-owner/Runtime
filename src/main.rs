use std::fmt::Display;

use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};
use chrono::prelude::*;

use crate::system_metrics::SystemMetrics;
mod cli;
mod system_metrics;

fn main() {
    let args: RuntimeArgs = cli::parse_args();
    let runtime = Runtime::new(args);
    println!("{runtime}");
}

#[derive(Debug, Clone)]
struct Runtime {
    args: RuntimeArgs,
    system: SystemMetrics,
}

impl Runtime {
    fn new(args: RuntimeArgs) -> Runtime {
        Self {
            args,
            system: SystemMetrics::default(),
        }
    }

    fn refresh(&mut self) {
        self.system.refresh();
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (load1, load5, load15) = self.system.format_load_avg();

        let current_time = Local::now();
        let formatted_time = current_time.format("⏰ %H:%M:%S").to_string();

        let formatted_uptime = self.system.format_uptime(&self.args.format);

        let up_since = if self.args.show_since {
            let boot_time = SystemMetrics::boot_time();
            let boot_datetime = DateTime::from_timestamp(boot_time as i64, 0)
                .unwrap_or_default()
                .with_timezone(&Local);
            format!(" since {}", boot_datetime.format("%Y-%m-%d %H:%M:%S"))
        } else {
            String::new()
        };

        let container_suffix = if self.args.show_container {
            " (container)"
        } else {
            ""
        };

        let user_label = if self.system.user_count == 1 {
            "user"
        } else {
            "users"
        };

        writeln!(
            f,
            "{} {} up {}{}{}, {} load average: {:.2}, {:.2}, {:.2}",
            Green.paint(formatted_time),
            Yellow.paint("💻"),
            Blue.paint(formatted_uptime),
            container_suffix,
            up_since,
            Purple.paint(format!("{} {}", self.system.user_count, user_label)),
            load1,
            load5,
            load15
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    /// Human-readable formatted output
    Pretty,
    /// Raw numerical values
    Raw,
    /// Default system format
    Standard,
}

/// Command line arguments structure
#[derive(Debug, Clone)]
pub struct RuntimeArgs {
    pub format: OutputFormat,
    pub show_container: bool,
    pub show_since: bool,
}
