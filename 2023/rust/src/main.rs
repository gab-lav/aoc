use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
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
    if day == "1" || day == "all" {
       day1::day1(test);
    } 
    if day == "2" || day == "all" {
       day2::day2(test);
    }
    if day == "3" || day == "all" {
       day3::day3(test);
    }
}


