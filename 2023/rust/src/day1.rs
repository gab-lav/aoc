use crate::utils;

pub fn day1(test: bool) {
    let input: String;
    if test {
        input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
    } else {
        input = utils::read_input(1);
    }

    let elves = input.split("\n\n");
    
    let mut max = 0;
    for e in elves {
        let mut calories = 0;
        let snacks = e.split("\n");
        for s in snacks {
            calories += s.parse::<i32>().unwrap();
        }
        if calories > max {
            max = calories;
        }
    }
    println!("{max}");
}
