use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    hello_rust::course::day12_cli_config::run_with_args(args).unwrap_or_else(|error| {
        eprintln!("{error}");
        eprintln!("示例：cargo run --bin day12_cli_config -- --service checkout-api --dry-run");
        std::process::exit(1);
    });
}
