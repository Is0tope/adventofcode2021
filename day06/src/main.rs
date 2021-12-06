use std::{fs::{read_to_string}, collections::HashMap};

const REGULAR_LIFE: i32 = 6;
const NEW_LIFE_START: i32 = 8;

fn load() -> Vec<i32> {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    return text.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
}

fn count_lanternfish(numbers: &Vec<i32>, generations: i32) -> u128 {
    let mut fish: HashMap<i32,u128> = HashMap::new();
    numbers.iter().for_each(|&x| *fish.entry(x).or_insert(0) += 1);
    for _ in 0..generations {
        let mut updated: HashMap<i32,u128> = HashMap::new();
        for &life in fish.keys() {
            let count = *fish.get(&life).unwrap();
            let mut new_life = life - 1;
            if life == 0 {
                new_life = REGULAR_LIFE;
            }
            *updated.entry(new_life).or_insert(0) += count;
        }
        if fish.contains_key(&0) {
            let new_fish = fish.get(&0).unwrap();
            *updated.entry(NEW_LIFE_START).or_insert(0) += new_fish;
        }
        fish = updated;
    }
    return fish.into_values().sum::<u128>();
}

fn part_a(numbers: &Vec<i32>) {
    let num_fish = count_lanternfish(&numbers,80);
    println!("A: {}", num_fish);
}

fn part_b(numbers: &Vec<i32>) {
    let num_fish = count_lanternfish(&numbers,256);
    println!("B: {}", num_fish)
}

fn main() {
    let numbers = load();
    part_a(&numbers);
    part_b(&numbers);
}
