use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32
}

fn load() -> Vec<Instruction> {
    let mut v: Vec<Instruction> = Vec::new();
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = line.split(" ").collect::<Vec<&str>>();
        let direction = match tokens[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Incorrect type"),
        };
        let distance: i32 = tokens[1].parse().unwrap();
        v.push(Instruction {
            direction,
            distance
        })
    }
    return v;
}

fn part_a(instructions: &Vec<Instruction>) {
    let mut horizontal = 0;
    let mut vertical = 0;
    for ins in instructions {
        match ins.direction {
            Direction::Forward => horizontal += ins.distance,
            Direction::Up => vertical -= ins.distance,
            Direction::Down => vertical += ins.distance,
        }
    }
    println!("A: {}", vertical * horizontal);
}

fn part_b(instructions: &Vec<Instruction>) {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for ins in instructions {
        match ins.direction {
            Direction::Forward => {
                horizontal += ins.distance;
                vertical += aim * ins.distance;
            },
            Direction::Up => aim -= ins.distance,
            Direction::Down => aim += ins.distance,
        }
    }
    println!("B: {}", vertical * horizontal);}

fn main() {
    let instructions = load();
    part_a(&instructions);
    part_b(&instructions);
}
