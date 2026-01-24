use std::env;
use std::process;

use cli_text_analyzer::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = cli_text_analyzer::run(config) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
