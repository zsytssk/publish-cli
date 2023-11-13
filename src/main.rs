use std::env;
use whale_cli;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    whale_cli::run(&args[0]).unwrap();
}
