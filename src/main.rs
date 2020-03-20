use std::{env, env::Args};
use std::string::String;
use std::fs;

fn main() {
    let mut args: Args = env::args();
    args.skip();

    let config = Config::new(args);

    if config.filename.is_some() {
        println!("{}", config.filename.as_ref().unwrap());
    }
    println!("{:?}", config.seconds);
}

struct Config {
    seconds: u32,
    filename: Option<String>,
}

impl Config {
    fn new(mut args: Args) -> Config {
        let mut time_str = String::new();
        let mut filename: Option<String> = None;

        loop {
            let arg = args.next();
            if arg.is_none() {
                break;
            }

            let arg = arg.unwrap();
            if arg == "-f" {
                let path = args.next().expect("Must provide a filename").clone();
                filename = Some(path.clone());
                
                time_str = fs::read_to_string(path).expect("Unable to read file");
                time_str = String::from(time_str.trim());
            } else {
                time_str = arg;
            }
        }

        let seconds: u32 = time_str.parse().expect("Can only take an integer");

        Config { filename, seconds }
    }
}