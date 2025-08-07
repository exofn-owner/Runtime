use sysinfo::{System, SystemExt};
use ansi_term::Colour::{Green, Yellow, Blue, Purple, Red};
use chrono::prelude::*;
use clap::{Arg, Command};
mod cli;
mod system_metrics;


fn main() {
    let args = cli::parse_args();
    let mut system = system_metrics::SystemMetrics::new();
    let (load1, load5, load15) = system_metrics::format_load_avg(system.load_avg);

    let current_time = Local::now();
    let formatted_time = current_time.format("⏰ %H:%M:%S").to_string();
    
    let formatted_uptime = system_metrics::format_uptime(system.uptime, &args.format);
    
    let up_since = if args.show_since {
        let boot_time = system_metrics::SystemMetrics::boot_time();
        let boot_datetime = DateTime::from_timestamp(boot_time as i64, 0)
            .unwrap_or_default()
            .with_timezone(&Local);
        format!(" since {}", boot_datetime.format("%Y-%m-%d %H:%M:%S"))
    } else {
        String::new()
    };
    
    let container_suffix = if args.show_container {
        " (container)"
    } else {
        ""
    };
    
    let user_label = if system.user_count == 1 { "user" } else { "users" };
    
    println!(
        "{} {} up {}{}{}, {} load average: {:.2}, {:.2}, {:.2}",
        Green.paint(formatted_time),
        Yellow.paint("💻"),
        Blue.paint(formatted_uptime),
        container_suffix,
        up_since,
        Purple.paint(format!("{} {}", system.user_count, user_label)),
        load1,
        load5,
        load15
    );
}
