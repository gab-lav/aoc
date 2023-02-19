use crate::utils;

pub fn day4(test: bool) {
    let input: String;
    if test {
        input = String::from("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
    } else {
        input = utils::read_input(4);
    }

    let mut part1 : u32 = 0;
    let mut part2 : u32 = 0;
    for pair in input.split("\n") {
        if is_pair_contained(pair) {
            part1 += 1;
        }
        if is_pair_overlapping(pair) {
            part2 += 1;
        }
    }
    println!("{part1}");
    println!("{part2}");
} 

fn get_tuple_from_pair_str(pair: &str) -> ((u32, u32), (u32, u32)) {
    let mut iter = pair.split(",");
    let mut iter_e1 = iter.next().unwrap().split("-").map(|x| x.parse::<u32>().unwrap());
    let mut iter_e2 = iter.next().unwrap().split("-").map(|x| x.parse::<u32>().unwrap());
    let e1 = (iter_e1.next().unwrap(), iter_e1.next().unwrap());
    let e2 = (iter_e2.next().unwrap(), iter_e2.next().unwrap());
    return (e1, e2)
}

fn is_pair_contained(pair: &str) -> bool{
    let tup = get_tuple_from_pair_str(pair);
    return (tup.0.0 <= tup.1.0 && tup.0.1 >= tup.1.1) || (tup.1.0 <= tup.0.0 && tup.1.1 >= tup.0.1)
}

fn is_pair_overlapping(pair: &str) -> bool {
    let tup = get_tuple_from_pair_str(pair);
    return (tup.0.0 <= tup.1.0 && tup.0.1 >= tup.1.0) ||
            (tup.0.0 <= tup.1.1 && tup.0.1 >= tup.1.1) || is_pair_contained(pair); 
}
