use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Alternative to error handling of config above.
    if let Err(e) = minigrep::run(config){
        eprint!("Application error: {e}");
        process::exit(1)
    };
}
