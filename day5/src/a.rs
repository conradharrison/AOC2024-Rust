use std::env;
use std::fs::read_to_string;
use std::fmt;
use std::collections::HashMap;

#[allow(dead_code)]
fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

#[allow(dead_code)]
fn read_chars(file_path: &str) -> String {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
}

#[derive(Clone)]
struct State<'a> {
    visited: bool,
    matches: HashMap<&'a str, bool>,
}
impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}.{}{}{}{}{}{}{}{}", self.visited as u8, 
                                self.matches["e"] as u8,
                                self.matches["w"] as u8,
                                self.matches["n"] as u8,
                                self.matches["s"] as u8,
                                self.matches["ne"] as u8,
                                self.matches["sw"] as u8,
                                self.matches["nw"] as u8,
                                self.matches["se"] as u8,
                                )
    }
}

fn print_state(state: &mut Vec<Vec<State>>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            let c = &(state[y])[x];
            print!("{c} ");
        }
        println!("")
    }
}

fn init_state(state: &mut Vec<Vec<State>>, height: usize, width: usize) {
    for _h in 0..height {
        let mut row = vec![];
        for _w in 0..width{
            let h = HashMap::from([("e", false), ("e", false), ("w", false), ("n", false), ("s", false), ("ne", false), ("sw", false), ("nw", false), ("se", false)]);
            let s = State{visited:false, matches:h};
            row.push(s);
        }
        state.push(row)
    }
}

fn mark_words(lines: &Vec<String>, word: &str, cx: usize, cy: usize, state: &mut Vec<Vec<State>>, dir: &str) -> u32 {
    let wl = word.len();
    let h = lines.len();
    let w = lines[0].len();

    let within_bound = || {
        match dir {
            "w" => cx >= wl-1,
            "e" => cx <= w-wl,
            "n" => cy >= wl-1,
            "s" => cy <= h-wl,
            "sw" => cx >= wl-1 && cy <= h-wl,
            "ne" => cx <= w-wl && cy >= wl-1,
            "nw" => cx >= wl-1 && cy >= wl-1,
            "se" => cx <= w-wl && cy <= h-wl,
            &_ => todo!()
        }
    };

    let x_index = | c, i | {
        match dir {
            "w" => c-i,
            "e" => c+i,
            "n" => c,
            "s" => c,
            "sw" => c-i,
            "ne" => c+i,
            "nw" => c-i,
            "se" => c+i,
            &_ => todo!()
        }
    };

    let y_index = | c, i | {
        match dir {
            "w" => c,
            "e" => c,
            "n" => c-i,
            "s" => c+i,
            "sw" => c+i,
            "ne" => c-i,
            "nw" => c-i,
            "se" => c+i,
            &_ => todo!()
        }
    };

    if within_bound() { 
        let mut hmatch = true;
        for (i, c1) in word.chars().enumerate() {
            let c2 = (lines[y_index(cy,i)]).chars().nth(x_index(cx,i)).unwrap();
            //println!("comparing {c1} with {c2}");
            hmatch = hmatch && (c1 == c2);
        }
        if hmatch {
            //println!("found");
            for i in 0..wl {
                *state[y_index(cy,i)][x_index(cx,i)].matches.get_mut(dir).unwrap() = true;
            }
        }
        hmatch as u32
    } else {
        0
    }

}

fn find_word(lines: &Vec<String>, word: &str) -> u32 {

    let wl = word.len();
    let h = lines.len();
    let w = lines[0].len();

    //println!("h={h}, w={w}, word={word}");

    let mut state: Vec<Vec<State>> = vec![];

    init_state(&mut state, h, w);
    print_state(&mut state, h, w);

    let mut score = 0;

    for y in 0..h {
        for x in 0..w {
            //let c = (lines[y]).chars().nth(x).unwrap();
            //print!("{c}");
            score += mark_words(lines, word, x, y, &mut state, "e");
            score += mark_words(lines, word, x, y, &mut state, "w");
            score += mark_words(lines, word, x, y, &mut state, "n");
            score += mark_words(lines, word, x, y, &mut state, "s");
            score += mark_words(lines, word, x, y, &mut state, "ne");
            score += mark_words(lines, word, x, y, &mut state, "sw");
            score += mark_words(lines, word, x, y, &mut state, "nw");
            score += mark_words(lines, word, x, y, &mut state, "se");
        }
        //println!("")
    }
    
    print_state(&mut state, h, w);

    score
}

enum OrderList {
    Empty,
    Elem(u32, Box<OrderList>),
}
fn parse_rules(lines: &mut Vec<String>) {
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
    //
    let rulelines: Vec<String>;
    let pagelines: Vec<String>;
    let mut rules_done: bool = false;
    for l in lines {
        if rules_done == false {
            if l == "" {
                rules_done = true;
            } else {
                rulelines.insert(l)
            }
        } else {
            pagelines.insert(l)
        }
    }
    
    parse_rules(&rulelines);

}
