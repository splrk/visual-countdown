use std::{env, env::Args};
use std::process;

use timetimer::Config;

fn main() {
    let mut args: Args = env::args();
    args.next();

    let config = Config::new(args).unwrap_or_else(|e| {
        println!("Invalid argurments: {}", e);
        process::exit(1);
    });

    if let Err(e) = timetimer::run(config) {
        println!("Application Aborted: {}", e);
    }
}
