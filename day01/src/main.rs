use std::fs::File;
use std::io::{BufRead, BufReader};

fn load() -> Vec<i32> {
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    return reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn part_a(numbers: &Vec<i32>) {
    let counter = numbers.iter()
        .skip(1)
        .enumerate()
        .filter(|&(i,x)| {
            return *x > numbers[i];
        })
        .count();
    println!("A: {}", counter)
}

fn part_b(numbers: &Vec<i32>) {
    let window_sums = numbers.windows(3)
        .map(|x| x.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    let counter = window_sums.iter()
        .skip(1)
        .enumerate()
        .filter(|&(i,x)| {
            return *x > window_sums[i];
        })
        .count();
    println!("B: {}", counter)
}

fn main() {
    let numbers = load();
    part_a(&numbers);
    part_b(&numbers);
}
