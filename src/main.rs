use std::env;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("the string to look for : {}", query);
    println!("the filename to look for: {}", filename);

    let mut f = File::open(filename).expect("File not found sorry");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text: \n {}", contents);
}
