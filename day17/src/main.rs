use std::{fs::{read_to_string}};
use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Zone {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Zone {
    fn contains(self: &Zone, p: &Vector2) -> bool {
        return  (p.x >= self.x_min) &&
                (p.x <= self.x_max) &&
                (p.y >= self.y_min) &&
                (p.y <= self.y_max);
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    fn add(self: &mut Vector2, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn load() -> Zone {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let re = Regex::new(r"x=([+-]?\d+)\.\.([+-]?\d+), y=([+-]?\d+)\.\.([+-]?\d+)").unwrap();
    let caps = re.captures(&text).unwrap();
    return Zone { 
        x_min: caps[1].parse().unwrap(),
        x_max: caps[2].parse().unwrap(),
        y_min: caps[3].parse().unwrap(), 
        y_max: caps[4].parse().unwrap()
    }
}

fn find_max_y_velocity(zone: &Zone) -> i32 {
    // The projectile always must return back to the x=0 line with the same starting velocity,
    // but in the opposite direction. Therefore the next step will have v+1, so v must be abs(y_min -1)
    return zone.y_min.abs() - 1;
}

// This assumes the target is below us, otherwise would need more complicated exit strategy
fn will_hit(zone: &Zone, v: Vector2) -> bool {
    let mut v = v.clone();
    let mut p = Vector2 { x: 0, y: 0};
    loop {
        if zone.contains(&p) {
            return true;
        }
        if p.x > zone.x_max || p.y < zone.y_min {
            return false;
        }
        // Update position
        p.add(&v);
        // Drag
        if v.x > 0 {
            v.x -= 1;
        }
        // Gravity
        v.y -= 1;   
    }
}

fn part_a(zone: &Zone) {
    let v_y = find_max_y_velocity(&zone);
    // Max height is just triangular number summation
    let max_height = (v_y * (v_y + 1)) / 2;
    println!("A: {}", max_height);
}

fn part_b(zone: &Zone) {
    let min_xv = 0;
    // Max x velocity is maximum step to far side of zone
    let max_xv = zone.x_max;
    // Max y velocity is same as part A
    let max_yv = find_max_y_velocity(&zone);
    // Min y velocity is biggest single jump that can be made down
    let min_yv = zone.y_min;
    // Try all the combos and see if they hit
    let mut num_trajectories = 0;
    for x in min_xv..=max_xv {
        for y in min_yv..=max_yv {
            let v = Vector2 { x, y };
            if will_hit(&zone, v) {
                num_trajectories += 1;
            }
        }
    }
    println!("B: {}", num_trajectories);
}

fn main() {
    let zone = load();
    part_a(&zone);
    part_b(&zone);
}
