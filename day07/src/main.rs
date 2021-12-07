use std::{fs::{read_to_string}, cmp::{min}, i64::MAX};

fn load() -> Vec<i64> {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    return text.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
}

fn get_fuel_used_a(crabs: &Vec<i64>, location: usize) -> i64{
    return crabs.into_iter().map(|&x| (x - location as i64).abs()).sum()
}

fn triangle_sum(target: i64) -> i64 {
    return (target * (target + 1)) / 2;
}

fn get_fuel_used_b(crabs: &Vec<i64>, location: usize) -> i64 {
    return crabs.into_iter().map(|&x| triangle_sum((x - location as i64).abs())).sum()
}

fn part_a(crabs: &Vec<i64>) {
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();
    let mut min_fuel: i64 = MAX;
    for i in min_crab..max_crab {
        min_fuel = min(min_fuel,get_fuel_used_a(&crabs, i.try_into().unwrap()));
    }
    println!("A: {}", min_fuel);
}

fn part_b(crabs: &Vec<i64>) {
    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();
    let mut min_fuel: i64 = MAX;
    for i in min_crab..max_crab {
        min_fuel = min(min_fuel,get_fuel_used_b(&crabs, i.try_into().unwrap()));
    }
    println!("B: {}", min_fuel);
}

fn main() {
    let crabs = load();
    part_a(&crabs);
    part_b(&crabs);
}
