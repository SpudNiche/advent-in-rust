use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

	// Get cmd line argument
        let args: Vec<String> = env::args().collect();
        let f = get_filename(&args);

	// Array of slopes: (right, down)
	let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

	// The result?
	let mut total = 1;
	for slope in slopes.iter() {
		let result = count_trees(slope, f);
		total *= result;
		println!("For slope ({}, {}), trees encountered: {}", slope.0, slope.1, result);	
	}
	println!("All together now: {}", total);
}

fn get_filename(args: &[String]) -> &str {
	if args.len() < 2 {
		panic!("Missing file input argument");
	}
	let filename = &args[1];
	filename
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

fn count_trees(slope: &(usize, usize), file: &str) -> usize {

	let tree = '#';
	let mut cur_index = 0;
	let mut tree_count = 0;
	let mut line_num = 0;

	// Open file and start parsing
	if let Ok(lines) = read_lines(file) {
		for line in lines {
			if line_num % slope.1 == 0 { 
				if let Ok(cur_line) = line {
					// Check for trees!
					let t = &cur_line.chars().nth(cur_index).unwrap(); 
					if t.eq(&tree) {
						tree_count += 1;
					}

					// Increment our counter!
					if cur_index + slope.0 < cur_line.chars().count() {
						cur_index += slope.0;
					} else {
						cur_index = (cur_index + slope.0) % cur_line.chars().count();
					}
				}
			}
			line_num += 1;
		}
	}
	tree_count
}
