use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

	// Get cmd line argument
	let args: Vec<String> = env::args().collect();
	let f = &args[1];
	let mut valid_count = 0;

	if let Ok(lines) = read_lines(f) {
		for line in lines {
			if let Ok(cur) = line {
				let groups: Vec<&str> = cur.split(' ').collect();	
				let bounds: Vec<&str> = groups[0].split('-').collect();
				let min = &bounds[0].parse::<usize>().unwrap(); 
				let max = &bounds[1].parse::<usize>().unwrap();
				let character = &groups[1].trim_end_matches(':');
				let pw = &groups[2];
				let num = pw.matches(character).count();

				if num <= *max && num >= *min {
					valid_count += 1;
				}
			}
		}
	}
	
	println!("Valid pws: {}", valid_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
