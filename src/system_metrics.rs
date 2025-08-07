//! System metrics collection and formatting utilities
//!
//! Provides functionality to gather system metrics (uptime, load averages, users)
//! and format them for display.

use sysinfo::{System, SystemExt, LoadAvg};
use crate::cli::OutputFormat;

/// System metrics collector and formatter
pub struct SystemMetrics {
    /// System uptime in seconds
    pub uptime: u64,
    /// Load averages over 1, 5, and 15 minutes
    pub load_avg: LoadAvg,
    /// Number of logged-in users
    pub user_count: usize,
}

impl SystemMetrics {
    /// Creates a new SystemMetrics instance with refreshed data
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            uptime: system.uptime(),
            load_avg: system.load_average(),
            user_count: system.users().len(),
        }
    }
    
    /// Refreshes all metric values with current system data
    pub fn refresh(&mut self) {
        let mut system = System::new_all();
        system.refresh_all();
        self.uptime = system.uptime();
        self.load_avg = system.load_average();
        self.user_count = system.users().len();
    }
    
    /// Gets system boot time as UNIX timestamp
    /// 
    /// # Returns
    /// `Result<u64, sysinfo::SystemError>` - Boot time or error
    pub fn boot_time() -> u64 {
        System::new_all().boot_time()
    }
}

/// Formats uptime duration according to requested format
pub fn format_uptime(uptime: u64, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Raw => format!("{}", uptime),
        OutputFormat::Pretty => {
            let hours = uptime / 3600;
            let minutes = (uptime % 3600) / 60;
            format!("{} hour{} {} minute{}", 
                hours, plural(hours), 
                minutes, plural(minutes))
        },
        OutputFormat::Standard => {
            let days = uptime / 86400;
            let hours = (uptime % 86400) / 3600;
            format!("{} days {} hours", days, hours)
        }
    }
}

/// Helper function for pluralization
fn plural(count: u64) -> &'static str {
    if count != 1 { "s" } else { "" }
}

/// Formats load averages into a tuple of floating point values
pub fn format_load_avg(load_avg: LoadAvg) -> (f64, f64, f64) {
    (load_avg.one, load_avg.five, load_avg.fifteen)
}
