use crate::utils;

pub fn day1(test: bool) {
    let input: String;
    if test {
        input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
    } else {
        input = utils::read_input(1);
    }

    let mut elves = Vec::new();
    for e in input.split("\n\n") {
        let mut calories = 0;
        let snacks = e.split("\n");
        for s in snacks {
            calories += s.parse::<i32>().unwrap();
        }
        elves.push(calories);
    }

    let mut max: i32;
    {
        let part1 = elves.iter().max().unwrap();
        max = part1.clone();
        println!("{part1}");
    }

    let mut i = 0;
    let mut part2 = max.clone();
    while i < 2 {
        let index = elves.iter().position(|&r| r == max).unwrap();
        elves.remove(index);
        max = *(elves.iter().max().unwrap());
        part2 += max ;
        i += 1;
    }

    println!("{part2}");
}

