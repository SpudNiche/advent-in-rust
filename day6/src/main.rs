// Author: SpudNiche
// Advent of Code, Day 6

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

fn main() {
    // Read in cmd line args
    let args: Vec<String> = env::args().collect();
    let f = get_filename(&args);

    // Open file
    if let Ok(lines) = read_lines(f) {
        let mut questions = Vec::new();
        let mut sum_ques = 0;
        let mut people_in_group = 0;
        for (_index, line) in lines.enumerate() {
            let line = line.unwrap();
            if line != String::from("") {
                for (_i, c) in line.chars().enumerate() {
                    questions.push(c);
                }
                people_in_group += 1;
            } else {
                questions.sort();
                let mut unique_ques = Vec::from_iter(questions.iter().cloned());
                unique_ques.dedup();
                for c in unique_ques {
                    if people_in_group == questions.iter().filter(|&x| *x == c).count() {
                        sum_ques += 1;
                    }
                }
                questions.clear();
                people_in_group = 0;
            }
            //println!("{}: {}", _index + 1, line);
        }
        // Do it one last time for the last entry
        questions.sort();
        let mut unique_ques = Vec::from_iter(questions.iter().cloned());
        unique_ques.dedup();
        for c in unique_ques {
            if people_in_group == questions.iter().filter(|&x| *x == c).count() {
                sum_ques += 1;
            }
        }
        println!("Total: {}", sum_ques);
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
