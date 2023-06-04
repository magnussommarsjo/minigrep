use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Alternative to error handling of config above.
    if let Err(e) = minigrep::run(config){
        eprint!("Application error: {e}");
        process::exit(1)
    };
}
