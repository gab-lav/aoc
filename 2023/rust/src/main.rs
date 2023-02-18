use std::env;

pub mod day1;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).unwrap();
    let test: bool;
    if args.get(2).is_some() && args.get(2).unwrap() == "test" {
        test = true;
    } else {
        test = false;
    }
    if day == "1" {
       day1::day1(test);
    }
}


