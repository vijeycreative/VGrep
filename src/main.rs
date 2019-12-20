use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

fn main()
{
    let args:Vec<String>=env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem in Parsing {}: ", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    run(config).unwrap_or_else(|err|{
        println!("Application Error {}: ", err);
        process::exit(1);
    });
}

fn run(config:Config) -> Result<(), Box< dyn Error>>
{
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed");
    println!{"{}", contents};
    return Ok(());
}

struct Config
{
    query: String,
    filename: String
}

impl Config{
    fn new(args:&[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3{
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config{query, filename});
    }
}