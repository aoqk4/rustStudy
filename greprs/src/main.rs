use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1];
    let _filename = &args[2];

    println!("Searching for {}", _query);
    println!("In file {}", _filename);
}
