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
        let mut max_seat: i16 = 0;
        for (index, line) in lines.enumerate() {
            let line = line.unwrap();
            let s = get_seat(&line);
            if s > max_seat {
                max_seat = s;
            }
            println!("{}: {}", index + 1, s);
        }
        println!("Max seat number: {}", max_seat);
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

fn get_seat(input: &str) -> i16 {
    let mut seat: i16 = -1;
    let mut flag = true;

    // Validate the input
    if input.chars().count() != 10 {
        println!("INVALID: Incorrect # of characters"); 
        flag = false;
    } else {
        for (i, c) in input.chars().enumerate() {
            if i < 7 {
                if c != 'F' && c != 'B' {
                    println!("INVALID: F/B section");
                    flag = false;
                }
            } else {
                if c != 'L' && c != 'R' {
                    println!("INVALID: L/R section");
                    flag = false;
                }
            }
        }
    }

    // Determine the row using the first 7 chars of the input
    if flag == true {
        let temp = input.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
        seat = i16::from_str_radix(&temp, 2).unwrap();
    }
    seat
}
