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
					// Completed parsing passport, validate it has everything it needs
					let mut flag: bool = true;

					if  validate_parts_exist(&pp) == false {
						//println!("INVALID: Passport did not contain all parts");
						flag = false;
					}
					if  validate_byr(&pp) == false {
						//println!("INVALID BYR: {}", pp.byr);
						flag = false;
					} else {
						//println!("VALID BYR: {}", pp.byr);
					}
					if  validate_iyr(&pp) == false {
						//println!("INVALID IYR: {}", pp.iyr);
						flag = false;
					} else {
						//println!("VALID IYR: {}", pp.iyr);
					}
					if  validate_eyr(&pp) == false {
						//println!("INVALID EYR: {}", pp.eyr);
						flag = false;
					} else {
						//println!("VALID EYR: {}", pp.eyr);
					}
					if  validate_hgt(&pp) == false {
						//println!("INVALID HGT: {}", pp.hgt);
						flag = false;
					} else {
						//println!("VALID HGT: {}", pp.hgt);
					}
					if  validate_hcl(&pp) == false {
						//println!("INVALID HCL: {}", pp.hcl);
						flag = false;
					} else {
						println!("VALID HCL: {}", pp.hcl);
					}
					if  validate_ecl(&pp) == false {
						//println!("INVALID ECL: {}", pp.ecl);
						flag = false;
					}
					if  validate_pid(&pp) == false {
						//println!("INVALID PID: {}", pp.pid);
						flag = false;
					}

					if flag == true {
						valid_passport_count += 1;	
					}

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

fn validate_parts_exist(passport: &Passport) -> bool {
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

fn validate_byr(passport: &Passport) -> bool {
	let mut r: bool = true;
	if passport.byr < 1920 || passport.byr > 2002 {
		r = false;
	}
	r
}

fn validate_iyr(passport: &Passport) -> bool {
	let mut r: bool = true;
	if passport.iyr < 2010 || passport.iyr > 2020 {
		r = false;
	}
	r
}

fn validate_eyr(passport: &Passport) -> bool {
	let mut r: bool = true;
	if passport.eyr < 2020 || passport.eyr > 2030 {
		r = false;
	}
	r
}
	
fn validate_hgt(passport: &Passport) -> bool {
	let mut r: bool = false;
	if passport.hgt.contains("cm") {
		let h = passport.hgt.strip_suffix("cm").unwrap().parse::<usize>().unwrap();
		if h >= 150 && h <= 193 {
			r = true;
		}
	} 
	if passport.hgt.contains("in") {
		let h = passport.hgt.strip_suffix("in").unwrap().parse::<usize>().unwrap();
		if h >= 59 && h <= 76 {
			r = true;
		}
	}
	r
}

fn validate_hcl(passport: &Passport) -> bool {
	let mut r: bool = true;
	if passport.hcl.chars().count() != 7 {
		r = false;
	} else if passport.hcl.starts_with("#") == false {
		r = false;
	} else if passport.hcl.strip_prefix("#").unwrap().chars().all(|c| char::is_ascii_hexdigit(&c)) == false {
		r = false;
	}	
	r
}
	
fn validate_ecl(passport: &Passport) -> bool {
	let mut r: bool = true;
	let e = passport.ecl.as_str();
	if (e != "amb") &&
	   (e != "blu") &&
	   (e != "brn") &&
	   (e != "gry") &&
	   (e != "grn") &&
	   (e != "hzl") &&
	   (e != "oth") {
		r = false;
	}
	r
}

fn validate_pid(passport: &Passport) -> bool {
	let mut r: bool = true;
	if passport.pid.len() != 9 {
		r = false;
	} else if passport.pid.chars().all(|c| char::is_ascii_digit(&c)) == false {
		r = false;
	}
	r
}
