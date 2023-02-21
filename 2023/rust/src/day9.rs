use crate::utils;

pub fn day9(test: bool) {
    let input: String;
    if test {
        input = String::from("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
    } else {
        input = utils::read_input(9);
    }

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut visited2: Vec<(i32, i32)> = Vec::new();
    let mut pos = ((0,0), (0,0));

    let mut rope2: [(i32, i32); 10] = [(0,0);10];
    for l in input.split("\n") {

        let mut iter = l.split(" ");
        let dir = iter.next().unwrap();
        let cnt = iter.next().unwrap().parse::<i32>().unwrap();

        for c in 0..cnt {
            let new_head = update_head(dir, &pos.0);
            let new_tail = update_tail(&new_head, &pos.1);
            pos = (new_head, new_tail);
            let mut already = false;
            for v in &visited {
                if pos.1.0 == v.0 && pos.1.1 == v.1 {
                    already = true;
                    break;
                }
            }
            if !already {
                visited.push(pos.1.clone());
            }

            rope2[0] = update_head(dir, &rope2[0]);
            for i in 1..rope2.len() {
                rope2[i] = update_tail(&rope2[i-1], &rope2[i]);
            }
            already = false;
            for v in &visited2 {
                if rope2.last().unwrap().0 == v.0 && rope2.last().unwrap().1 == v.1 {
                    already = true;
                    break;
                }
            }
            if !already {
                visited2.push(rope2.last().unwrap().clone());
            }
        }
    }    
    let part1 = visited.len() as i32;
    let part2 = visited2.len() as i32;
    println!("{part1}");
    println!("{part2}");
}


fn update_head(dir: &str, head_pos: &(i32, i32)) -> (i32, i32) {
    let mut new_head = (0,0);

    if dir == "U" {
        new_head = (head_pos.0, head_pos.1 + 1); 
    } else if dir == "D" {
        new_head = (head_pos.0, head_pos.1 - 1); 
    } else if dir == "L" {
        new_head = (head_pos.0 - 1, head_pos.1); 
    } else if dir == "R" {
        new_head = (head_pos.0 + 1, head_pos.1); 
    }
    return new_head
}

fn update_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let dist = get_dist(head, tail);
    if dist <= 1 {
        return tail.clone();
    } else if dist == 2 {
        if head.0 == tail.0 {
            return (tail.0, tail.1 + (head.1 - tail.1).signum())
        }
        if head.1 == tail.1 {
            return (tail.0 + (head.0 - tail.0).signum(), tail.1)
        }
        return tail.clone()
    }
    return (tail.0 + (head.0 - tail.0).signum(), tail.1 + (head.1 - tail.1).signum())
}

fn get_dist(head: &(i32, i32), tail: &(i32, i32)) -> i32 {
    return (tail.0 - head.0).abs() + (tail.1 - head.1).abs();
}

