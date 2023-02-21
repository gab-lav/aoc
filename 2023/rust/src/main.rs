use std::env;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
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
        let now = Instant::now();
        day1::day1(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    } 
    if day == "2" || day == "all" {
        println!("Day 2");
        let now = Instant::now();
        day2::day2(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "3" || day == "all" {
        println!("Day 3");
        let now = Instant::now();
        day3::day3(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "4" || day == "all" {
        println!("Day 4");
        let now = Instant::now();
        day4::day4(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "5" || day == "all" {
        println!("Day 5");
        let now = Instant::now();
        day5::day5(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "6" || day == "all" {
        println!("Day 6");
        let now = Instant::now();
        day6::day6(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "7" || day == "all" {
        println!("Day 7");
        let now = Instant::now();
        day7::day7(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "8" || day == "all" {
        println!("Day 8");
        let now = Instant::now();
        day8::day8(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "9" || day == "all" {
        println!("Day 9");
        let now = Instant::now();
        day9::day9(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }
    if day == "10" || day == "all" {
        println!("Day 10");
        let now = Instant::now();
        day10::day10(test);
        println!("{}ms\n===============", now.elapsed().as_millis());
    }

}


