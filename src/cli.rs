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
        .about("Show how long the system has been running")
        .disable_version_flag(true)  // We handle version ourselves
        .arg(
            Arg::new("container")
                .short('c')
                .long("container")
                .help("show container uptime")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("show uptime in pretty format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("raw")
                .short('r')
                .long("raw")
                .help("show uptime values in raw format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("since")
                .short('s')
                .long("since")
                .help("system up since")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help("output version information and exit")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Determine output format based on flags (priority order matches uptime)
    let format = if matches.get_flag("since") {
        OutputFormat::Since
    } else if matches.get_flag("pretty") {
        OutputFormat::Pretty
    } else if matches.get_flag("raw") {
        OutputFormat::Raw
    } else {
        OutputFormat::Standard
    };

    RuntimeArgs {
        format,
        show_container: matches.get_flag("container"),
        show_version: matches.get_flag("version"),
    }
}
