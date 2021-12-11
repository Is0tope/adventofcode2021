use std::{fs::{read_to_string}, collections::{HashMap, HashSet}};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy)]
struct Squid {
    energy: u32,
    has_flashed: bool
}

type Grid = HashMap<Point,Squid>;

impl Point {
    fn get_neighbours(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        for y in &[-1,0,1] {
            for x in &[-1,0,1] { 
                if *x == 0 && *y == 0 {
                    continue;
                }
                ret.push(Point { x: self.x + x, y: self.y + y });
            }
        }
        return ret;
    }
}

fn print_grid(grid: &Grid, size: i32) {
    for y in 0..size {
        for x in 0..size {
            let ch: String = match grid.get(&Point {x:x, y:y}) {
                Some(s) => s.energy.to_string(),
                None => " ".to_string()
            };
            print!("{}",ch);
        }
        println!("");
    }
}

fn load() -> Grid {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let mut grid: Grid = Grid::new();
    for (y,line) in text.split("\n").enumerate() {
        for (x,c) in line.chars().enumerate() {
            let p = Point { x: x as i32, y: y as i32};
            grid.insert(p, Squid {
                energy: c.to_digit(10).unwrap(),
                has_flashed: false
            });
        }
    }
    return grid;
}

// Returns (number of flashes, number of squid that flashed)
fn step_grid(grid: &mut Grid) -> (u32, u32) {
    let mut num_flashes: u32 = 0;
    let mut unique_flashed: HashSet<Point> = HashSet::new();
    // Increment all of the squid by 1, and reset their flashed flag
    grid.values_mut().for_each(|mut x| {
        x.energy += 1;
        x.has_flashed = false;
    });
    // While there are squid with flashes, keep iterating
    loop {
        // Find any squids with greater than 9 energy
        let to_flash: Vec<Point> = grid.keys().cloned().filter(|&x| {
            let squid = grid.get(&x).unwrap();
            return squid.energy > 9 && !squid.has_flashed;
        })
        .collect();
        // If there are no more left to flash, then break out
        if to_flash.len() == 0 {
            break;
        }
        // Flash the squids, and get a list of neighbours for any that did
        let flashed_neighbours: Vec<Point> = to_flash.iter().map(|&x| {
            let mut squid = grid.get_mut(&x).unwrap();
            squid.energy = 0;
            squid.has_flashed = true;
            num_flashes += 1;
            unique_flashed.insert(x);
            return x.get_neighbours();
        })
        .flatten()
        .collect();
        // For each of the neighbouts (if they exist, and have not already flashed)
        // increase their energy by 1
        flashed_neighbours.iter().for_each(|x| {
            if let Some(mut squid) = grid.get_mut(&x) {
                if !squid.has_flashed {
                    squid.energy += 1;
                }
            }
        });
    }
    return (num_flashes,unique_flashed.len() as u32);
}

fn part_a(grid: &Grid) {
    let mut grid = grid.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        let (num_flashes, _) = step_grid(&mut grid);
        flashes += num_flashes;
    }
    println!("A: {}", flashes);
}

fn part_b(grid: &Grid) {
    let mut grid = grid.clone();
    let mut step = 1;
    let num_squids = grid.len() as u32;
    loop {
        let (_, num_squids_flashed) = step_grid(&mut grid);
        if num_squids_flashed == num_squids {
            break;
        }
        step += 1;
    }
    println!("B: {}", step)
}

fn main() {
    let grid = load();
    part_a(&grid);
    part_b(&grid);
}
