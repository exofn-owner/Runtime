//! Command line interface configuration and parsing
//!
//! This module handles argument parsing using clap and returns structured output.

use clap::{Arg, Command};

use runtime::OutputFormat;
use runtime::RuntimeArgs;

/// Runtime display format options
/// 
/// Parses command line arguments using clap
///
/// # Returns
/// `RuntimeArgs` struct containing parsed arguments
///
/// # Example
/// ```no_run
/// use runtime::cli::parse_args;
///
/// let args = parse_args();
/// ```
pub fn parse_args() -> RuntimeArgs {
    let matches = Command::new("Runtime")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Displays system runtime metrics with customizable formatting")
        // .arg_required_else_help(true)
        .arg(
            Arg::new("output-format")
                .short('f')
                .long("format")
                .value_parser(["pretty", "raw", "standard"])
                .default_value("pretty")
                .help("Output format style"),
        )
        // Remove old format flags
        .arg(
            Arg::new("container")
                .short('c')
                .long("container")
                .help("Show container uptime")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("since")
                .short('s')
                .long("since")
                .help("Show system up since timestamp")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let format = match matches
        .get_one::<String>("output-format")
        .expect("default is set")
        .as_str()
    {
        "pretty" => OutputFormat::Pretty,
        "raw" => OutputFormat::Raw,
        "standard" => OutputFormat::Standard,
        _ => unreachable!("Invalid format variant"),
    };

    RuntimeArgs {
        format,
        show_container: matches.get_flag("container"),
        show_since: matches.get_flag("since"),
    }
}
