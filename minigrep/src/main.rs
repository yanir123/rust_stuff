use minigrep::Config;
use std::{env::args, process::exit};

fn main() {
    let argv: Vec<String> = args().collect();

    let conf = Config::build(&argv).unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {e}");
        exit(1);
    }
}
