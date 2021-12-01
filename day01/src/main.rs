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
    let mut counter = 0;
    for index in 1..numbers.len() {
        let prev = numbers[index - 1];
        let num = numbers[index];
        if num > prev {
            counter += 1;
        }
    }
    println!("A: {}", counter)
}

fn part_b(numbers: &Vec<i32>) {
    let mut counter = 0;
    let mut prev = window_sum(&numbers, 3, 0);
    for index in 1..numbers.len()-2 {
        let current = window_sum(numbers, 3, index);
        if current > prev {
            counter += 1;
        }
        prev = current;
    }
    println!("B: {}", counter)
}

fn window_sum(numbers: &Vec<i32>, size: usize, index: usize) -> i32 {
    let mut total = 0;
    for i in index..index+size {
        total += numbers[i];
    }
    return total;
}

fn main() {
    let numbers = load();
    part_a(&numbers);
    part_b(&numbers);
}
