use std::{env, env::Args};
use std::string::String;
use std::fs;

fn main() {
    let mut args: Args = env::args();
    args.next();

    let seconds = parse_args(args);
    
    println!("{:?}", seconds);
}

fn parse_args(mut args: Args) -> u32 {
    let mut time_str = String::new();
    loop {
        let arg = args.next();
        if arg.is_none() {
            break;
        }

        let arg = arg.unwrap();
        if arg == "-f" {
            let filename = args.next().expect("Must provide a filename");
            time_str = fs::read_to_string(filename).expect("Unable to read file");
            time_str = String::from(time_str.trim());
        } else {
            time_str = arg;
        }
    }

    time_str.parse().expect("Can only take an integer")
}