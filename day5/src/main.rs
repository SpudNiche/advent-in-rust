// Author: SpudNiche
// Advent of Code, Day 5

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read in cmd line args
    let args: Vec<String> = env::args().collect();
    let f = get_filename(&args);

    // Open file
    if let Ok(lines) = read_lines(f) {
        // Parse input line by line 
        // Thanks: https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
        for (index, line) in lines.enumerate() {
            let line = line.unwrap();
            println!("{}: {}", index + 1, line);
        }
    }
}

fn get_filename(args: &[String]) -> &str {
	if args.len() != 2 {
		panic!("Invalid arguments");
	}
	let filename = &args[1];
	filename
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}
