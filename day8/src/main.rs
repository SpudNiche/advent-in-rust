// Author: SpudNiche
// Advent of Code, Day 8

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	// Read in cmd line args
    let args: Vec<String> = env::args().collect();
    let f = get_filename(&args);

	// Open file, load program into a vector
	let mut prog: Vec<String> = Vec::new();
	if let Ok(lines) = read_lines(f) {
		for line in lines {
			prog.push(line.unwrap());
		}
	}	

	// Parse vector
	let mut acc: i32 = 0;
	let mut index: i32 = 0;
	let mut visited: Vec<i32> = Vec::new();
	while index < prog.len() as i32 {
		let line: Vec<&str> = prog.iter().nth(index as usize).unwrap().split(' ').collect();
		let op = line[0]; 
		let num = line[1].parse::<i32>().unwrap();

		if visited.contains(&index) {
			println!("Accumulated: {}", acc);
			break;
		} else {
			println!("Pushing index {}", index);
			visited.push(index);
		}

		if op == "acc" {
			acc += num;
			index += 1;
		} else if op == "jmp" {
			index += num;
		} else {
			index += 1;
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

