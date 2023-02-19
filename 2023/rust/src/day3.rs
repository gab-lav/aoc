use crate::utils;

pub fn day3(test: bool) {
    let input: String;
    if test {
        input = String::from("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
    } else {
        input = utils::read_input(3);
    }

    let mut part1 : u32 = 0;
    let mut part2 : u32 = 0;
    for rucksack in input.split("\n") {
        part1 += get_rucksack_prority(&rucksack) ;
    }
    let mut iter = input.split("\n");
    loop {
        let e1 = iter.next();
        let e2 = iter.next();
        let e3 = iter.next();
        if e1.is_none() || e2.is_none() || e3.is_none() {
            break;
        }
        let elves = (e1.unwrap(), e2.unwrap(), e3.unwrap());
        part2 += get_elves_triple_priority(&elves);
    }
    println!("{part1}");
    println!("{part2}");

}

fn get_rucksack_prority(rucksack: &str) -> u32 {
    let mut prority: u32 = 0; 
    let left = &rucksack[rucksack.len()/2..];
    let right = &rucksack[..rucksack.len()/2];

    for c in left.chars() {
        if right.contains(c) {
            prority = convert_char_to_priority(&c);
            break;
        }
    }
    return prority;
}

fn get_elves_triple_priority(elves: &(&str, &str, &str)) -> u32{
    for c in elves.0.chars() {
        if elves.1.contains(c) && elves.2.contains(c) {
            return convert_char_to_priority(&c);
        }
    }
    return 0;
}

fn convert_char_to_priority(c : &char) -> u32 {
    let c_value = *c as u32;
    if c_value <= 90 {
        return c_value - 38;
    } else {
        return c_value - 96;
    }
}
