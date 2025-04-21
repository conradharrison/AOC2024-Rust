use std::env;
use std::fs::read_to_string;

#[allow(dead_code)]
fn read_lines(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
        .lines()            // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()          // gather them together into a vector
}

fn read_chars(file_path: &str) -> String {
    read_to_string(file_path) 
        .unwrap_or_else(|error| panic!("File read of {file_path} failed. {error}"))
}

enum Operation {
    //MUL{a:u32, b:u32},
    MUL(u32, u32),
}
impl Operation {
    fn exec(&self) -> u32{
        match *self {
            Self::MUL(a,b) => {
                a*b
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialEq,Debug)]
enum State {
    OUTSIDE,
    OP_A,
    OP_B,
}

fn parse(s: &str) -> u32 {
    let length = s.len();
    let mut i = 0;
    let mut st = State::OUTSIDE;
    let mut opa = String::from("");
    let mut opb = String::from("");
    let mut sum = 0;
    let mut do_it = true;

    while i < length {
        println!("Considering index {i} with state {st:?} {do_it}");
        match st {
            State::OUTSIDE => {
                // Check for don't
                if do_it {
                    if i <= (length - 7) {
                        if &s[i..(i+7)] == "don't()" {
                            let t = &s[i..(i+7)];
                            println!("slice0 = {t}");
                            do_it = false;
                            i = i + 7;
                        } else {
                            if i <= (length - 4) {
                                let t = &s[i..(i+4)];
                                println!("slice = {t}");
                                if &s[i..(i+4)] == "mul(" {
                                    st = State::OP_A;
                                    i = i + 4;
                                } else {
                                    i = i + 1;
                                }
                            } else {
                                i = i + 1;
                            }
                        }
                    } else {
                        if i <= (length - 4) {
                            let t = &s[i..(i+4)];
                            println!("slice2 = {t}");
                            if &s[i..(i+4)] == "mul(" {
                                st = State::OP_A;
                                i = i + 4;
                            } else {
                                i = i + 1;
                            }
                        } else {
                            i = i + 1;
                        }
                    }
                } else {
                    if i <= (length - 4) {
                        if &s[i..(i+4)] == "do()" {
                            do_it = true;
                            i = i + 4;
                        } else {
                            i = i + 1;
                        }
                    } else {
                        i = i + 1;
                    }
                }
            }
            State::OP_A => {
                if (&s[i..(i+1)]).chars().nth(0).unwrap().is_digit(10) {
                    opa.push(s.chars().nth(i).unwrap());
                } else if (&s[i..(i+1)]).chars().nth(0).unwrap() == ',' {
                    if opa.len() == 0 {
                        st = State::OUTSIDE;
                    } else {
                        st = State::OP_B;
                    }
                } else {
                    opa.clear();
                    opb.clear();
                    st = State::OUTSIDE;
                }
                i = i + 1;
            }
            State::OP_B => {
                if (&s[i..(i+1)]).chars().nth(0).unwrap().is_digit(10) {
                    opb.push(s.chars().nth(i).unwrap());
                } else if (&s[i..(i+1)]).chars().nth(0).unwrap() == ')' {
                    if opb.len() != 0 {
                        println!("Found an Op: {opa:?}, {opb:?}");
                        sum = sum + Operation::MUL(opa.parse::<u32>().unwrap(), opb.parse::<u32>().unwrap()).exec();
                    }
                    opa.clear();
                    opb.clear();
                    st = State::OUTSIDE;
                } else {
                    opa.clear();
                    opb.clear();
                    st = State::OUTSIDE;
                }
                i = i + 1;
            }
        }
    }

    sum
}

fn main() {

    // Read in data file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must provide data file path (only)");
    }
    let file_path = &args[1];

    let chars = read_chars(file_path);
    
    // Debug
    //println!("Read the following from {file_path}:\n{chars:?}");

    // Solution
    let r = parse(&chars);
    println!("{r}");

}
