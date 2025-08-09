//! Command line interface matching the exact behavior of standard uptime
//!
//! This module handles argument parsing to exactly match uptime's behavior

use clap::{Arg, Command};
use runtime::{OutputFormat, RuntimeArgs};

/// Parse command line arguments exactly like standard uptime
///
/// # Returns
/// `RuntimeArgs` struct containing parsed arguments
pub fn parse_args() -> RuntimeArgs {
    let matches = Command::new("runtime")
        .version(env!("CARGO_PKG_VERSION"))
        .about("* Modern colorful uptime utility with interactive dashboard *")
        .long_about("A modern replacement for the classic uptime command with beautiful colors,\nanimations, and multiple output formats.")
        .disable_version_flag(true)  // We handle version ourselves
        .arg(
            Arg::new("container")
                .short('c')
                .long("container")
                .help("Show container uptime indicators")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Show uptime in pretty human-readable format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("raw")
                .short('r')
                .long("raw")
                .help("Show uptime values in raw machine-readable format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("since")
                .short('s')
                .long("since")
                .help("Show system boot timestamp")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("standard")
                .long("standard")
                .help("Show standard uptime format (like original uptime)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Show interactive colorful dashboard (default)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help("Show version information and exit")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Determine output format based on flags (priority order)
    let format = if matches.get_flag("since") {
        OutputFormat::Since
    } else if matches.get_flag("raw") {
        OutputFormat::Raw
    } else if matches.get_flag("pretty") {
        OutputFormat::Pretty
    } else if matches.get_flag("standard") {
        OutputFormat::Standard
    } else if matches.get_flag("interactive") {
        OutputFormat::Interactive
    } else {
        // Default to interactive if no specific format requested
        OutputFormat::Interactive
    };

    RuntimeArgs {
        format,
        show_container: matches.get_flag("container"),
        show_version: matches.get_flag("version"),
    }
}
