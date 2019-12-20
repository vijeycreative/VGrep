use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main()
{
    let args:Vec<String>=env::args().collect();
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    let mut file = File::open(config.filename).expect("File Not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed");
    println!{"{}", contents};
}

struct Config
{
    query: String,
    filename: String
}

fn parse_config(args:&[String]) -> Config
{
    let query = args[1].clone();
    let filename = args[2].clone();
    return Config{query, filename};
}
