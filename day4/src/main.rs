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

	// Get number of lines (there is definitely a better way to do this)
	let mut line_count = 0;
	if let Ok(lines) = read_lines(f) {
		for _line in lines {
			line_count += 1;
		}
		println!("Lines: {}", line_count);
	}

	// Parse file and validate passports
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
					println!("Result: {:?}", result);	
					println!();

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

//	println!("byr: {}, iyr: {}, eyr: {}", passport.byr, passport.iyr, passport.eyr);
	
	if (passport.byr == 0 || passport.byr < 1920 || passport.byr > 2002) || 
	   (passport.iyr == 0 || passport.iyr < 2010 || passport.iyr > 2020) ||
	   (passport.eyr == 0 || passport.eyr < 2020 || passport.eyr > 2030) {
		println!("failed in part 1");
		r = false;
	}
	if (passport.hgt == String::from("")) ||
	   (passport.hcl == String::from("")) || 
	   (passport.ecl == String::from("")) || 
	   (passport.pid == String::from("")) {
		println!("Failed in part 2");
		r = false;
	}

	if r == true {
		// Check height
		if passport.hgt.contains("cm") {
			let h = passport.hgt.strip_suffix("cm").unwrap().parse::<usize>().unwrap();
			if h < 150 || h > 193 {
				println!("Failed in cm check");
				r = false;
			}
		} else if passport.hgt.contains("in") {
			let h = passport.hgt.strip_suffix("in").unwrap().parse::<usize>().unwrap();
			if h < 59 || h > 76 {
				r = false;
				println!("Failed in in check");
			}
		}
	}	

	if r == true {
		// Check hair color
		println!("hcl: {}", passport.hcl); 

		if passport.hcl.starts_with("#") {
			let t1 = passport.hcl.strip_prefix("#").unwrap().chars().all(|c| char::is_ascii_hexdigit(&c)); 
			println!("stripped: {}", passport.hcl.strip_prefix("#").unwrap());
			let t2 = passport.hcl.chars().count();
			println!("count: {}", t2);
			if t1 == false {
				r = false;
			} else if t2 != 7 {
				r = false;
			}
		} else {
			r = false;
		}
	}
	
	if r == true {
		// Check eye color
		match passport.ecl.as_str() {
			"amb" => r = true,
			"blu" => r = true,
			"brn" => r = true,
			"gry" => r = true,
			"grn" => r = true,
			"hzl" => r = true,
			"oth" => r = true,
			_ => r = false,
		}	
		if r == false {
			println!("failed in ecl check");
		}
	}

	if r == true {
		// Check pid (9-digit number)
		if passport.pid.len() != 9 {
			println!("Failed in pid - too short");
			r = false;	
		}
		if !passport.pid.chars().all(char::is_numeric) {
			println!("Failed in pid, not numeric");
			r = false;	
		}
	}
	r
}
