use crate::utils;

pub fn day6(test: bool) {
    let input: String;
    if test {
        input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    } else {
        input = utils::read_input(6);
    }
    let part1 = detect_start_of_packet(&input, 4);
    let part2 = detect_start_of_packet(&input, 14);
    println!("{part1}");
    println!("{part2}");
}

fn detect_start_of_packet(input: &String, num: usize) -> i32 {
    let mut index : i32 = -1;
    for n in num..&input.len() + 1 {
        let test = &input[n-num..n];
        index = n as i32;
        for c in test.chars() {
            let count = test.matches(c).count();
            if count != 1 {
                index = -1;
                break;
            }
        }
        if index != -1 {
            break;
        }
    }
    return index;
}
