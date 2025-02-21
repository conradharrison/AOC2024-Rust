use std::env;
use std::fs::read_to_string;

fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn _is_safe(words: &Vec<String>) -> bool {
    let mut dir: i32 = 0;
    let mut words_iter = words.iter();
    let Some(mut x) = words_iter.next() else { panic!("No way!!!") };
    while let Some(n) = words_iter.next() {
        let d: i32 = n.parse::<i32>().unwrap() - x.parse::<i32>().unwrap();
        if d*dir < 0 || d.abs() > 3 || d == 0 {
            return false;
        }
        dir = d;
        x = n;
    }
    return true;
}

fn is_safe(line: &String) -> bool {
    let words: Vec<String> = line.split_whitespace().map(String::from).collect();
    //println!("{:?}", words);
    for i in 0..words.len() {
        let mut new_words = words.clone();
        new_words.remove(i);
        //println!("    {:?}", new_words);
        if !_is_safe(&new_words) {
            //println!("    not safe");
        } else {
            //println!("    safe");
            return true;
        }
    }
    return false;
}

fn main() {

    // Read in data file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must provide data file path (only)");
    }
    let file_path = &args[1];

    let lines = read_lines(file_path);
    
    // Debug
    //println!("Read the following from {file_path}:\n{lines:?}");


    // Solution
    let mut safe_lines: u32 = 0;
    for l in lines {
        safe_lines += is_safe(&l) as u32;
    }

    println!("{safe_lines}");

}
