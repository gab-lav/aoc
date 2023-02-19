use crate::utils;

pub fn day2(test: bool) {
    let input: String;
    if test {
        input = String::from("A Y\nB X\nC Z");
    } else {
        input = utils::read_input(2);
    }
    
    let mut score1: i32 = 0;
    let mut score2: i32 = 0;
    for round in input.split("\n") {
        let split: Vec<&str> = round.split(" ").collect();
        score1 += get_round_score(split[0], split[1]);
        score2 += get_round_score(split[0], &get_converted_me_round2(split[0], split[1]));
    }
    println!("{score1}");
    println!("{score2}");
}

fn get_converted_me_round2<'a>(them : &'a str, rule : &'a str) -> String {
    let them_char = them.chars().nth(0).unwrap() as u32;
    let mut retval = String::from(char::from_u32(them_char + 23).unwrap());
    if them == "A" 
    {
        if rule == "X" {
            retval = "Z".to_string();
        } else if rule == "Z" {
            retval = "Y".to_string();
        }
    } else if them == "B" {
        if rule == "X" {
            retval = "X".to_string();
        }else if rule == "Z" {
            retval = "Z".to_string();
        }
    } else if them == "C" {
        if rule == "X" {
            retval = "Y".to_string();
        } else if rule == "Z" {
            retval = "X".to_string();
        }
    }
    return retval;
}

fn get_round_score(them : &str, me: &str) -> i32{
    let mut score: i32 = 0;
    if me == "X" {
        score = 1;
    } else if me == "Y" {
        score = 2;
    } else if me == "Z" {
        score = 3;
    }

    if them == "A" {
        if me == "X" {
            score += 3;
        } else if me == "Y" {
            score += 6;
        } else if me == "Z" {
            score += 0;
        }
    } else if them == "B" {
        if me == "X" {
            score += 0;
        } else if me == "Y" {
            score += 3;
        } else if me == "Z" {
            score += 6;
        }
    } else if them == "C" {
        if me == "X" {
            score += 6;
        } else if me == "Y" {
            score += 0;
        } else if me == "Z" {
            score += 3;
        }
    }
    return score;
}
