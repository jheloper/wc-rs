use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let file_name = &args[1];
    println!("file name: {}", &file_name);

    let contents = fs::read_to_string(&file_name).expect("Something went wrong reading the file");
    println!("contents:\n{}", &contents);
}
