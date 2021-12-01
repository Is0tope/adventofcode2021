use std::fs::File;
use std::io::{BufRead, BufReader};

fn load() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        v.push(line.parse::<i32>().unwrap())
    }
    return v;
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
