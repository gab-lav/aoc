use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
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
        println!("Day 1");
        day1::day1(test);
    } 
    if day == "2" || day == "all" {
        println!("Day 2");
        day2::day2(test);
    }
    if day == "3" || day == "all" {
        println!("Day 3");
        day3::day3(test);
    }
    if day == "4" || day == "all" {
        println!("Day 4");
        day4::day4(test);
    }
    if day == "5" || day == "all" {
        println!("Day 5");
        day5::day5(test);
    }
    if day == "6" || day == "all" {
        println!("Day 6");
        day6::day6(test);
    }
    if day == "7" || day == "all" {
        println!("Day 7");
        day7::day7(test);
    }
    if day == "8" || day == "all" {
        println!("Day 8");
        day8::day8(test);
    }

}


