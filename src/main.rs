use qst::config;
use std::{env, process};

fn main() {
    let args = env::args();
    let config = config::Config::build_from_cmdline(args).unwrap_or_else(|msg| {
        eprintln!("Error parsing config: {msg}");
        process::exit(1);
    });
    match qst::start_server(config) {
        Ok(_) => println!("Bye Bye!"),
        Err(msg) => {
            eprintln!("Server failed with message: {msg}");
            process::exit(1);
        },
    }
}
