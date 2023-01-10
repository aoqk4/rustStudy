use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1];
    let _filename = &args[2];

    println!("Searching for {}", _query);
    println!("In file {}", _filename);

    let mut f = File::open(_filename).expect("file not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Somthing went wrong reading the file");

    println!("With text:\n{}", contents);
}
