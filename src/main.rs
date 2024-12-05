use std::env;
use std::fs;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            println!("{}, {}", i, c);
        }
    }
}

