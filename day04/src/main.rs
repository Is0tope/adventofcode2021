use std::{fs::{read_to_string}};

type Grid = Vec<Vec<u32>>;

fn load() -> (Vec<u32>, Vec<Grid>) {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let lines: Vec<&str> = text.split("\n").collect();
    // Get the choices
    let choices: Vec<u32> = lines.get(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    // Skip a line and get the grids
    let grid_text = lines
        .as_slice()[2..]
        .split(|x| x.len() == 0);
    let mut grids: Vec<Grid> = Vec::new();
    for g in grid_text {
        let g: Grid = g.iter()
            .map(|&l| l.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>())
            .collect();
        grids.push(g);
    }
    return (choices,grids);
}

fn evaluate_grid(grid: &Grid, choices: &Vec<u32>) -> bool{
    let height = grid.len();
    let width = grid[0].len();
    // We assume only one line can win
    for h in 0..height {
        if grid[h].iter().all(|x| choices.contains(x)) {
            return true;
        }
    }
    for w in 0..width {
        if grid.iter().map(|x| x[w]).all(|x| choices.contains(&x)) {
            return true;
        }
    }
    return false;
}

fn calculate_score(grid: &Grid, choices: &Vec<u32>) -> u32 {
    let all_numbers = grid.into_iter().flatten();
    return all_numbers.filter(|x| !choices.contains(x)).sum();
}

fn part_a(grids: &Vec<Grid>, choices: &Vec<u32>) {
    let mut called: Vec<u32> = Vec::new();
    for &num in choices {
        called.push(num);
        for grid in grids {
            if evaluate_grid(&grid, &called) {
                let score = calculate_score(&grid, &called);
                return println!("A: {}", num * &score);
            }
        }
    } 
}

fn part_b(grids: &Vec<Grid>, choices: &Vec<u32>) {
    let mut called: Vec<u32> = Vec::new();
    let mut grids: Vec<Grid> = grids.clone();
    for &num in choices {
        called.push(num);
        let mut winners = Vec::new();
        for (index,grid) in grids.iter().enumerate() {
            if evaluate_grid(&grid, &called) {
                // If only one grid left, it must be the winner
                if grids.len() == 1 {
                    let score = calculate_score(&grid, &called);
                    return println!("B: {}", num * &score);
                }
                winners.push(index);
            }
        }
        // Do it in reverse order to keep index removal stable
        winners.into_iter().rev().for_each(|i| {
            grids.remove(i);
            return
        });
    }
}

fn main() {
    let (choices, grids) = load();
    part_a(&grids,&choices);
    part_b(&grids,&choices);
}
