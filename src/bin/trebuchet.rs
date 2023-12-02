/*
 * https://adventofcode.com/2023/day/1
 * Prompts for Copilot in comments.
 */

/* read a file and return a String.  Use filepath as parameter */  
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(filepath: &str) -> String {
    let f = File::open(filepath).expect("file not found");
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("error reading file");
    contents
}

/*  write a function that splits a string by newline and returns a vector of the lines */
pub fn split_lines(s: &str) -> Vec<&str> {
    s.split("\n").collect()
}

/*
 * find first digit and last digit in a string, then return first digit times ten plus last digit
 */
pub fn find_calibration_value(s: &str) -> u32 {
    let mut last = 0;
    let mut first = 0;
    let mut first_digit = true;
    for c in s.chars() {
        if c.is_digit(10) {
            if first_digit {
                first = c.to_digit(10).unwrap();
                first_digit = false;
            }
            last = c.to_digit(10).unwrap();
        }
    }
    first * 10 + last
}


fn main() {
    /* take first argument of program and store it as filename */
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    /* pass filename into splitlines, save as lines */
    let file = read_file(filename);
    let lines = split_lines(&file);

    /* iterate over lines, sum first and last digit, add to total */
    let mut total = 0;
    for line in lines {
        total += find_calibration_value(line);
    }
    
    println!("Total: {}", total);
}
