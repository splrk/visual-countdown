use std::env::Args;
use std::string::String;
use std::fs;
use std::error::Error;
use std::thread;
use std::time::{SystemTime, Duration};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = config.filename {
        println!("Reading from {}", filename);
    }

    let total_time = Duration::new(config.seconds, 0);
    println!("{:?}", total_time);
    

    let start = SystemTime::now();
    for i in 0..config.seconds {
        let seconds_left = config.seconds - i;
        println!("{}", String::from("#").repeat(seconds_left as usize));

        let wait_time = Duration::new(i + 1, 0);
        let elapsed = start.elapsed()?;
        if elapsed < wait_time {
            thread::sleep(wait_time - start.elapsed()?);
        }
    }

    Ok(())
}

pub struct Config {
    seconds: u64,
    filename: Option<String>,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, Box<dyn Error>> {
        let mut time_str = String::new();

        loop {
            let arg = args.next();
            if arg.is_none() {
                break;
            }

            let arg = arg.unwrap();
            if arg == "-f" {
                return read_file_flag(&mut args);
            } else {
                time_str = arg;
            }
        }

        let seconds = parse_int(&time_str)?;

        Ok(Config { filename: None, seconds })
    }
}

fn read_file_flag(args: &mut Args) -> Result<Config, Box<dyn Error>> {
    let path = match args.next() {
        Some(p) => p,
        None => String::new(),
    };

    let filename = Some(path.clone());
    
    let time_str = fs::read_to_string(path)?;
    let seconds = parse_int(time_str.trim())?;

    Ok(Config { seconds, filename })
}

fn parse_int(input_str: &str) -> Result<u64, Box<dyn Error>> {
    Ok(input_str.parse()?)
}