use runtime::Runtime;
use runtime::RuntimeArgs;

// use run::system_metrics::SystemMetrics;
mod cli;
// mod system_metrics;

fn main() {
    let args: RuntimeArgs = cli::parse_args();
    let runtime = Runtime::new(args);
    println!("{runtime}");
}
