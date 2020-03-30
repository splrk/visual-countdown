use std::env;
use std::string::String;
use std::process;

use timetimer::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(args).unwrap_or_else(|e| {
        println!("Invalid argurments: {}", e);
        process::exit(1);
    });

    if let Err(e) = timetimer::run(config) {
        println!("Application Aborted: {}", e);
    }
}
