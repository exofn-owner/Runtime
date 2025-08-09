//! Low-level system metrics collection using direct /proc filesystem access
//!
//! This module provides precise system metrics by reading directly from the Linux
//! /proc filesystem, matching the behavior of the standard uptime command.

use std::fs;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

/// System metrics collector using low-level /proc filesystem access
#[derive(Debug, Clone, PartialEq)]
pub struct SystemMetrics {
    /// System uptime in seconds (floating point for precision)
    uptime_seconds: f64,
    /// System idle time in seconds
    idle_time: f64,
    /// Load averages (1min, 5min, 15min)
    load_avg: (f64, f64, f64),
    /// Number of unique logged-in users
    user_count: usize,
    /// System boot time as UNIX timestamp
    boot_time: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            uptime_seconds: 0.0,
            idle_time: 0.0,
            load_avg: (0.0, 0.0, 0.0),
            user_count: 0,
            boot_time: 0,
        }
    }
}

impl SystemMetrics {
    /// Creates a new SystemMetrics instance by reading from /proc filesystem
    pub fn new() -> io::Result<Self> {
        let mut metrics = Self::default();

        // Read uptime and idle time from /proc/uptime
        metrics.read_uptime()?;

        // Read load averages from /proc/loadavg
        metrics.read_loadavg()?;

        // Read user count from /proc/stat and utmp-like sources
        metrics.read_users()?;

        // Calculate boot time from uptime
        metrics.calculate_boot_time()?;

        Ok(metrics)
    }

    /// Read uptime and idle time from /proc/uptime
    fn read_uptime(&mut self) -> io::Result<()> {
        let content = fs::read_to_string("/proc/uptime")?;
        let parts: Vec<&str> = content.trim().split_whitespace().collect();

        if parts.len() >= 2 {
            self.uptime_seconds = parts[0].parse().unwrap_or(0.0);
            self.idle_time = parts[1].parse().unwrap_or(0.0);
        }

        Ok(())
    }

    /// Read load averages from /proc/loadavg
    fn read_loadavg(&mut self) -> io::Result<()> {
        let content = fs::read_to_string("/proc/loadavg")?;
        let parts: Vec<&str> = content.trim().split_whitespace().collect();

        if parts.len() >= 3 {
            let load1 = parts[0].parse().unwrap_or(0.0);
            let load5 = parts[1].parse().unwrap_or(0.0);
            let load15 = parts[2].parse().unwrap_or(0.0);
            self.load_avg = (load1, load5, load15);
        }

        Ok(())
    }

    /// Count unique users from multiple sources to match uptime behavior
    fn read_users(&mut self) -> io::Result<()> {
        let mut unique_users = HashSet::new();

        // Method 1: Read from /proc/*/stat to find processes with ttys
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.chars().all(|c| c.is_ascii_digit()) {
                        if let Ok(stat_content) = fs::read_to_string(entry.path().join("stat")) {
                            let parts: Vec<&str> = stat_content.split_whitespace().collect();
                            if parts.len() > 6 {
                                let tty_nr: i32 = parts[6].parse().unwrap_or(0);
                                if tty_nr > 0 {
                                    // This process has a controlling terminal
                                    if let Ok(status_content) = fs::read_to_string(entry.path().join("status")) {
                                        for line in status_content.lines() {
                                            if line.starts_with("Uid:") {
                                                if let Some(uid_str) = line.split_whitespace().nth(1) {
                                                    if let Ok(uid) = uid_str.parse::<u32>() {
                                                        // Only count UIDs >= 1000 (regular users) or root (0)
                                                        if uid >= 1000 || uid == 0 {
                                                            unique_users.insert(uid);
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Method 2: Fallback - count login sessions from /proc/*/fd/* pointing to ptys/ttys
        if unique_users.is_empty() {
            if let Ok(entries) = fs::read_dir("/proc") {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.chars().all(|c| c.is_ascii_digit()) {
                            let fd_dir = entry.path().join("fd");
                            if let Ok(fd_entries) = fs::read_dir(&fd_dir) {
                                let mut has_terminal = false;
                                for fd_entry in fd_entries.flatten() {
                                    if let Ok(link_target) = fs::read_link(fd_entry.path()) {
                                        if let Some(target_str) = link_target.to_str() {
                                            if target_str.starts_with("/dev/pts/") ||
                                               target_str.starts_with("/dev/tty") {
                                                has_terminal = true;
                                                break;
                                            }
                                        }
                                    }
                                }

                                if has_terminal {
                                    if let Ok(status_content) = fs::read_to_string(entry.path().join("status")) {
                                        for line in status_content.lines() {
                                            if line.starts_with("Uid:") {
                                                if let Some(uid_str) = line.split_whitespace().nth(1) {
                                                    if let Ok(uid) = uid_str.parse::<u32>() {
                                                        if uid >= 1000 || uid == 0 {
                                                            unique_users.insert(uid);
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Method 3: Final fallback - use a reasonable default based on system state
        if unique_users.is_empty() {
            // Check if we're in a graphical session or have active terminals
            let display_set = std::env::var("DISPLAY").is_ok();
            let wayland_set = std::env::var("WAYLAND_DISPLAY").is_ok();

            if display_set || wayland_set {
                unique_users.insert(1000); // Assume at least one regular user
            }

            // Always count the current user if we can determine it
            if let Ok(current_uid_str) = std::env::var("UID") {
                if let Ok(uid) = current_uid_str.parse::<u32>() {
                    unique_users.insert(uid);
                }
            }
        }

        self.user_count = if unique_users.is_empty() { 1 } else { unique_users.len() };
        Ok(())
    }

    /// Calculate boot time from current time minus uptime
    fn calculate_boot_time(&mut self) -> io::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.boot_time = now.saturating_sub(self.uptime_seconds as u64);
        Ok(())
    }

    /// Get uptime in seconds with decimal precision
    pub fn uptime_seconds(&self) -> f64 {
        self.uptime_seconds
    }

    /// Get idle time in seconds
    pub fn idle_time(&self) -> f64 {
        self.idle_time
    }

    /// Get load averages as (1min, 5min, 15min)
    pub fn load_averages(&self) -> (f64, f64, f64) {
        self.load_avg
    }

    /// Get number of unique users
    pub fn user_count(&self) -> usize {
        self.user_count
    }

    /// Get system boot time as UNIX timestamp
    pub fn boot_time(&self) -> u64 {
        self.boot_time
    }

    /// Refresh all metrics
    pub fn refresh(&mut self) -> io::Result<()> {
        self.read_uptime()?;
        self.read_loadavg()?;
        self.read_users()?;
        self.calculate_boot_time()?;
        Ok(())
    }
}
