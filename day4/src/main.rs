// Author: SpudNiche
// Advent of Code, Day 4

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Passport {
        byr: usize,
        iyr: usize,
        eyr: usize,
        hgt: String,
        hcl: String,
	ecl: String,
	pid: String,
	cid: usize
}

fn main() {

	// Get cmd line argument
        let args: Vec<String> = env::args().collect();
        let f = get_filename(&args);

	// Open file and start parsing
	let mut line_num: usize = 0;
	let mut valid_passport_count = 0;
	let mut pp = Passport { 
		byr: 0,
		iyr: 0,
		eyr: 0,
		hgt: String::from(""),
		hcl: String::from(""),
		ecl: String::from(""),
		pid: String::from(""),
		cid: 0,
	};

	let mut line_count = 0;
	if let Ok(lines) = read_lines(f) {
		for line in lines {
			line_count += 1;
		}
		println!("Lines: {}", line_count);
	}

	if let Ok(lines) = read_lines(f) {
		for line in lines {
			if let Ok(cur_line) = line {
				if cur_line.len() > 0 {
					//println!("{}", cur_line);
					let items: Vec<&str> = cur_line.split(' ').collect();
					for i in items.iter() {
						let pair: Vec<&str> = i.split(':').collect();
						match pair[0] {
							"byr" => pp.byr = pair[1].parse::<usize>().unwrap(),
							"iyr" => pp.iyr = pair[1].parse::<usize>().unwrap(),
							"eyr" => pp.eyr = pair[1].parse::<usize>().unwrap(),
							"hgt" => pp.hgt = String::from(pair[1]),
							"hcl" => pp.hcl = String::from(pair[1]),
							"ecl" => pp.ecl = String::from(pair[1]),
							"pid" => pp.pid = String::from(pair[1]),
							"cid" => pp.cid = pair[1].parse::<usize>().unwrap(),
							_ => println!("Something else"),
						}
					}
				}
				
				if cur_line.len() <= 0 || line_num == line_count - 1 {
					// Completed parsing passport, validate
					let result = validate_passport(&pp);
					if result == true {
						valid_passport_count += 1;	
					}
					//println!("Result: {:?}", result);	
					//println!();

					// Reset the passport struct
					pp.byr = 0;
					pp.iyr = 0;
					pp.eyr = 0;
					pp.hgt = String::from("");
					pp.hcl = String::from("");
					pp.ecl = String::from("");
					pp.pid = String::from("");
					pp.cid = 0;
				}
			}
			line_num += 1;
		}
	}
	println!("Valid Passports: {}", valid_passport_count);
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

fn validate_passport(passport: &Passport) -> bool {
	let mut r: bool = true;
	if (passport.byr == 0) || 
	   (passport.iyr == 0) ||
	   (passport.eyr == 0) ||
	   (passport.hgt == String::from("")) || 
	   (passport.hcl == String::from("")) || 
	   (passport.ecl == String::from("")) || 
	   (passport.pid == String::from("")) {
		r = false;
	}
	r
}
