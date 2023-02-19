use crate::utils;

pub fn day5(test: bool) {
    let input: String;
    if test {
        input = String::from("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");
    } else {
        input = utils::read_input(5);
    }

    let mut position :Vec<Vec<char>>;
    let mut input_iter = input.split("\n");
    {
        let mut starting_pos_vec: Vec<&str> = Vec::new();
        loop {
            let line = input_iter.next().unwrap();
            if line.len() == 0 {
                break;
            } else {
                starting_pos_vec.push(line);
            }
        }
        position = construct_starting_position(starting_pos_vec);
    }

    let mut position2 = position.clone();

    for line in input_iter {
        let operation = decode_operation(line);

        let mut temp: Vec<char> = Vec::new();
        for _n in 0..operation.0 {
            let item = position[operation.1 as usize].pop().unwrap();
            position[operation.2 as usize].push(item);

            let item2 = position2[operation.1 as usize].pop().unwrap();
            temp.push(item2.clone());
        }
        temp.reverse();
        position2[operation.2 as usize].append(&mut temp);
    }

    for s in position {
        let top = s.last().unwrap();
        print!("{top}");
    }
    print!("\n");

    for s in position2 {
        let top = s.last().unwrap();
        print!("{top}");
    }
    print!("\n");

} 

fn print_position(pos : &Vec<Vec<char>>) {
    for v in pos {
        for b in v {
            print!("{b} ");
        }
    println!("");
    }
    println!("==============================");
}

fn decode_operation(line: &str) -> (u32, u32, u32) {
    
    let mut iter = line.split(" ");
    iter.next();
    let num = iter.next().unwrap().parse::<u32>().unwrap();
    iter.next();
    let source = iter.next().unwrap().parse::<u32>().unwrap() - 1;
    iter.next();
    let dest = iter.next().unwrap().parse::<u32>().unwrap() - 1;

    return (num, source, dest);
}

fn construct_starting_position(lines: Vec<&str>) -> Vec<Vec<char>>{
    let num_stacks = (lines[0].len() + 1)/4;
    let mut pos: Vec<Vec<char>> = Vec::new();
    for _n in 0..num_stacks {
        let new_vec: Vec<char> = Vec::new();
        pos.push(new_vec);
    }
    let mut iter = lines.iter().rev();
    iter.next();
    for l in iter {
        let mut i = 0;
        for c in l.chars().skip(1).step_by(4) {
            if c != ' ' {
                pos[i].push(c);
            }
            i += 1;
        }
    }
    return pos;
}

