use std::fmt::Display;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod system_metrics;
use system_metrics::SystemMetrics;

#[cfg(test)]
mod tests;

/// Runtime structure that holds system metrics and formatting options
#[derive(Debug, Clone)]
pub struct Runtime {
    args: RuntimeArgs,
    system: SystemMetrics,
}

impl Runtime {
    /// Creates a new Runtime instance
    pub fn new(args: RuntimeArgs) -> Runtime {
        Self {
            args,
            system: SystemMetrics::new().unwrap_or_default(),
        }
    }

    /// Refreshes system metrics
    pub fn refresh(&mut self) {
        if let Ok(metrics) = SystemMetrics::new() {
            self.system = metrics;
        }
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.args.format {
            OutputFormat::Raw => {
                // Format: boot_time uptime_seconds idle_time load1 load5 load15
                let boot_time = self.system.boot_time();
                let uptime_secs = self.system.uptime_seconds();
                let idle_time = self.system.idle_time();
                let (load1, load5, load15) = self.system.load_averages();

                write!(
                    f,
                    "{} {:.6} {} {:.2} {:.2} {:.2}",
                    boot_time, uptime_secs, idle_time, load1, load5, load15
                )
            }
            OutputFormat::Pretty => {
                // Format: "up X hours, Y minutes"
                let uptime_secs = self.system.uptime_seconds();
                let hours = uptime_secs / 3600.0;
                let minutes = (uptime_secs % 3600.0) / 60.0;

                if hours >= 1.0 {
                    let h = hours as u64;
                    let m = minutes as u64;

                    if m > 0 {
                        write!(
                            f,
                            "up {} hour{}, {} minute{}",
                            h,
                            if h != 1 { "s" } else { "" },
                            m,
                            if m != 1 { "s" } else { "" }
                        )
                    } else {
                        write!(f, "up {} hour{}", h, if h != 1 { "s" } else { "" })
                    }
                } else {
                    let m = minutes as u64;
                    if m > 0 {
                        write!(f, "up {} minute{}", m, if m != 1 { "s" } else { "" })
                    } else {
                        write!(f, "up less than a minute")
                    }
                }
            }
            OutputFormat::Since => {
                // Format: "YYYY-MM-DD HH:MM:SS"
                let boot_time = self.system.boot_time();
                let datetime = chrono::DateTime::from_timestamp(boot_time as i64, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local);
                write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S"))
            }
            OutputFormat::Standard => {
                // Standard uptime format with time, uptime, users, load averages
                let now = chrono::Local::now();
                let time_str = now.format("%H:%M:%S");

                let uptime_secs = self.system.uptime_seconds();
                let days = uptime_secs as u64 / 86400;
                let hours = (uptime_secs as u64 % 86400) / 3600;
                let minutes = (uptime_secs as u64 % 3600) / 60;

                let uptime_str = if days > 0 {
                    if hours > 0 {
                        format!("{}:{:02}", days, hours)
                    } else {
                        format!("{} day{}", days, if days != 1 { "s" } else { "" })
                    }
                } else if hours > 0 {
                    format!("{}:{:02}", hours, minutes)
                } else {
                    format!("{} min", minutes)
                };

                let user_count = self.system.user_count();
                let user_str = if user_count == 1 { "user" } else { "users" };

                let (load1, load5, load15) = self.system.load_averages();

                let container_suffix = if self.args.show_container {
                    " (container)"
                } else {
                    ""
                };

                write!(
                    f,
                    " {} up {}{}, {} {}, load average: {:.2}, {:.2}, {:.2}",
                    time_str,
                    uptime_str,
                    container_suffix,
                    user_count,
                    user_str,
                    load1,
                    load5,
                    load15
                )
            }
        }
    }
}

/// Output format options
#[derive(Debug, PartialEq, Clone)]
pub enum OutputFormat {
    /// Standard uptime format
    Standard,
    /// Human-readable format
    Pretty,
    /// Raw numerical values
    Raw,
    /// Show since timestamp
    Since,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Standard
    }
}

/// Command line arguments structure
#[derive(Debug, Clone)]
pub struct RuntimeArgs {
    pub format: OutputFormat,
    pub show_container: bool,
    pub show_version: bool,
}

impl Default for RuntimeArgs {
    fn default() -> Self {
        Self {
            format: OutputFormat::Standard,
            show_container: false,
            show_version: false,
        }
    }
}
