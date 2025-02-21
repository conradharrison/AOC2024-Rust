use std::env;
use regex::Regex;
use std::fs::read_to_string;
use std::collections::HashMap;

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
    let mut left_hash = HashMap::new();
    let mut right_hash = HashMap::new();
    let mut sum: u32 = 0;
    for p in lines {
        let Ok(re) = Regex::new(r"(\d+) *(\d+)") else { panic!("No way!!!") };
        let Some(captures) = re.captures(p.as_str()) else { panic!("Capture failed for {:?}", p) };
        let l = String::from(captures.get(1).unwrap().as_str()); // convert t
        let r = String::from(captures.get(2).unwrap().as_str());
        //println!("{}: l={}, r={}", line!(), l, r);

        // first time seeing l in left hash? Add to left hash
        if !left_hash.contains_key(&l) {
            left_hash.insert(l.clone(), (1, 0));
        } else {
            let Some((x, _)) = left_hash.get_mut(&l) else { panic!("No way!!!") };
            *x = *x + 1;
        }

        // If r is still not in left hash, insert (or increment) in right hash
        if !left_hash.contains_key(&r) { // save it away...
            if right_hash.contains_key(&r) { // increment
                let Some(x) = right_hash.get_mut(&r) else { panic!("No way!!!") };
                *x = *x + 1;
            } else { // or insert
                right_hash.insert(r.clone(), 1);
            }
        } else { // ...or, directly increment left hash
            let Some((_, y)) = left_hash.get_mut(&r) else { panic!("No way!!!") };
            *y = *y + 1;
            right_hash.remove(&r);
        }

        if right_hash.contains_key(&l) {
            let Some(xr) = right_hash.get_mut(&l) else { panic!("No way!!!") };
            let Some((_, yl)) = left_hash.get_mut(&l) else { panic!("No way!!!") };
            // xl will always be 0 here; non-zero xl is handled in 57
            *yl = *yl + *xr;
            right_hash.remove(&l);
        }

        //println!("{} {left_hash:?}", line!());
        //println!("{} {right_hash:?}", line!());
    }

    for (k,(vx, vy)) in left_hash.into_iter() {
        sum += k.parse::<u32>().unwrap() * vx * vy;
    }

    println!("{sum}");
}
