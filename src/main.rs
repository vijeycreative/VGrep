use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main()
{
    let args:Vec<String>=env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In File {}", filename);

    let mut file = File::open(filename).expect("File Not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed");
    println!{"{}", contents};
}
