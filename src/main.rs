use runtime::Runtime;

mod cli;

fn main() {
    let args = cli::parse_args();

    // Handle version flag
    if args.show_version {
        println!("runtime from uptime-rs {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Create runtime and collect metrics
    let mut runtime = Runtime::new(args);
    runtime.refresh();

    // Print the result
    println!("{}", runtime);
}
