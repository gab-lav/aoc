use crate::utils;

pub fn day10(test: bool) {
    let input: String;
    if test {
        input = get_test_string();
    } else {
        input = utils::read_input(10);
    }

    let mut cycle: i32 = 1;
    let mut x: i32 = 1;
    let mut interesting_signals: Vec<i32> = Vec::new();
    let mut crt: Vec<Vec<char>> = Vec::new();

    for _n in 0..6 {
        crt.push(Vec::new());
    } 

    for n in (20..260).step_by(40) {
        interesting_signals.push(n);
    }

    let mut part1 = 0;

    for l in input.split("\n") {
        let mut add_active = false;
        { 
            let row = ((cycle - 1) / 40) as usize;
            let col = (cycle - 1) % 40;
            if col == x-1 || col == x || col == x+1 {
                crt[row].push('#');
            } else {
                crt[row].push('.');
            }
        }

        cycle += 1;

        if interesting_signals.contains(&cycle) {
            part1 += cycle * x;
        }
       
        if &l[..4] == "noop" {
        } else {
            let op = l.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
             { 
                let row = ((cycle - 1) / 40) as usize;
                let col = (cycle - 1) % 40;
                if col == x-1 || col == x || col == x+1 {
                    crt[row].push('#');
                } else {
                    crt[row].push('.');
                }
            }
            x += op;
            add_active = true;
        }

        if add_active {
           
            cycle += 1;
            if interesting_signals.contains(&cycle) {
                part1 += cycle * x;
            }
        }
    }
    println!("{}", part1);

    for r in crt {
        for c in r {
            print!("{c}");
        }
        print!("\n");
    }
}




fn get_test_string() -> String {
return String::from("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
}
