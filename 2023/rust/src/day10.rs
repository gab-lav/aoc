use crate::utils;

pub fn day10(test: bool) {
    let input: String;
    if test {
        input = get_test_string();
    } else {
        input = utils::read_input(10);
    }

    let mut cycle = 0;
    let mut x = 0;
    let mut interesting_signals: Vec<i32> = Vec::new();

    for n in (20..260).step_by(40) {
        interesting_signals.push(n);
    }

    let mut part1 = 0;
    let add_active = false;

    for l in input.split("\n") {
        let check = false;
        if !add_active {
            if &l[..4] == "noop" {
            } else {
                let op = l.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                x += op;
                add_active = true;
            }
        } else {
            add_active = false;
        }

        cycle += 1;
        if interesting_signals.contains(&cycle) {
            println!("cycle={} x={}", cycle, x);
            part1 += cycle * x;
        }
    }
}




fn get_test_string() -> String {
return String::from("noop
addx 3
addx -5");
}
