// Author: SpudNiche
// Advent of Code, Day 9

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	// Read in cmd line args
    let args: Vec<String> = env::args().collect();
    let f = get_filename(&args);

	let mut stream: Vec<usize> = Vec::new();
	if let Ok(lines) = read_lines(f) {
		for line in lines {
			let num = line.unwrap().parse::<usize>().unwrap();
			stream.push(num);
		}
	}	

	let window_size: usize = 25;
	for window in stream.windows(window_size + 1) {
		//println!("{:?}", window);
		let mut pairs: Vec<usize> = Vec::new();

		// For the current window, we want all possible pair sums
		// val(1) + val(2), val(1) + val(3)...val(24) + val(25)
		for (i, num) in window.iter().enumerate() {
			let a = num;				
			let mut index = i + 1;
			while index < window_size {
				let b = window.iter().nth(index).unwrap();	
				pairs.push(a + b);
				//println!("{} + {} = {}", a, b, a + b);
				index += 1;
			}
		}
		
		// Check if the remaining number in the window (the "last" one)
		// is the sum of any pairs in the previous segment of the window
		let last = window.last().unwrap();
		if pairs.contains(last) {
			continue;
		} else {
			println!("The culprit: {}", last);
			break;
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
