use crate::utils;

pub fn day8(test: bool) {
    let input: String;
    if test {
        input = String::from("30373
25512
65332
33549
35390");
    } else {
        input = utils::read_input(8);
    }

    let grid: Vec<&str> = input.split("\n").collect();
    let mut part1 = 0;
    part1 += grid.len() * 2;
    part1 += (grid[0].len() - 2) * 2;
    let mut part2 = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if is_tree_visible(i, j, &grid) {
                part1 += 1;
            }
            let score = visibility_score(i, j, &grid);
            if score > part2 {
                part2 = score;
            }
        }
    }
    println!("{part1}");
    println!("{part2}");
}

fn is_tree_visible(i: usize, j: usize, grid: &Vec<&str> ) -> bool{
    let mut tree = grid[i].chars().nth(j).unwrap().to_digit(10).unwrap();
    let mut vis_up = true;
    let mut vis_down = true;
    let mut vis_left = true;
    let mut vis_right = true;
    for n in (0..i).rev() {
        let new_tree = grid[n].chars().nth(j).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            vis_up = false;
            break;
        }
    }
    for n in (i+1..grid.len()){
        let new_tree = grid[n].chars().nth(j).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            vis_down = false;
            break;
        }
    } 
    for n in (0..j).rev() {
        let new_tree = grid[i].chars().nth(n).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            vis_left = false;
            break;
        }
    }
    for n in (j+1..grid[0].len()) {
        let new_tree = grid[i].chars().nth(n).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            vis_right = false;
            break;
        }
    }

    return vis_up || vis_left || vis_down || vis_right;
}

fn visibility_score(i: usize, j: usize, grid: &Vec<&str> ) -> i32{
    let mut tree = grid[i].chars().nth(j).unwrap().to_digit(10).unwrap();
    let mut score_up = 0;
    let mut score_down = 0;
    let mut score_left = 0;
    let mut score_right = 0;

    for n in (0..i).rev() {
        score_up += 1;
        let new_tree = grid[n].chars().nth(j).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            break;
        }
    }
    for n in (i+1..grid.len()){
        score_down += 1;
        let new_tree = grid[n].chars().nth(j).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            break;
        }
    } 
    for n in (0..j).rev() {
        score_left += 1;
        let new_tree = grid[i].chars().nth(n).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            break;
        }
    }
    for n in (j+1..grid[0].len()) {
        score_right += 1;
        let new_tree = grid[i].chars().nth(n).unwrap().to_digit(10).unwrap();
        if new_tree >= tree {
            break;
        }
    }
    
    return score_up * score_left * score_down * score_right;
}
