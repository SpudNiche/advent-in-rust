use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

	// Get cmd line argument
        let args: Vec<String> = env::args().collect();
        let f = get_filename(&args);

	// Variables we need
	let tree = '#';
	let mut cur_index = 0;
	let mut tree_count = 0;

	// Open file and start parsing
	if let Ok(lines) = read_lines(f) {
		for line in lines {
			if let Ok(cur_line) = line {
				// Check for trees!
				let t = &cur_line.chars().nth(cur_index).unwrap(); 
				if t.eq(&tree) {
					tree_count += 1;
				}

				// Increment our counter!
				if cur_index + 3 < cur_line.chars().count() {
					cur_index += 3;
				} else {
					cur_index = (cur_index + 3) % cur_line.chars().count();
				}
			}
		}
	}

	// The result?
	println!("Trees encountered: {}", tree_count);
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
