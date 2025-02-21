use std::env;
use regex::Regex;
use std::fs::read_to_string;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn main() {

    // Read in data file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must provide data file path (only)");
    }
    let file_path = &args[1];

    let lines = read_lines(file_path);

    println!("Read the following from {file_path}:\n{lines:?}");

    // Solution
    let mut l = Vec::<u32>::new();
    let mut r = Vec::<u32>::new();
    let mut sum: u32 = 0;
    for p in lines {
        let re = Regex::new(r"(\d+) *(\d+)").unwrap();
        let captures = re.captures(p.as_str()).unwrap_or_else(|| panic!("Capture failed for {:?}", l));
        l.push(captures[1].parse().unwrap());
        r.push(captures[2].parse().unwrap());
    }

    l.sort();
    r.sort();
    for i in 0..(l.len()) {
        sum += ((l[i] as i32) - (r[i] as i32)).abs() as u32;
    }

    println!("{sum}");

}
