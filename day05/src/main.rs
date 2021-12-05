use std::{fs::{read_to_string}, collections::HashMap, cmp::{max}};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

fn load() -> Vec<(Point,Point)> {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let instructions = text.split("\n")
        .map(|x| {
            return x.split(" -> ")
                .map(|x| {
                    let coords: Vec<i32> = x.split(",")
                        .map(|x| x.parse::<i32>()
                        .unwrap())
                        .collect();
                    return Point {
                        x: coords[0],
                        y: coords[1]
                    }
                }).collect::<Vec<Point>>()
        })
        .map(|x| (x[0],x[1]))
        .collect::<Vec<(Point,Point)>>();
    return instructions;
}

fn generate_points_between(start: &Point, end: &Point) -> Vec<Point> {
    let x_length = end.x - start.x;
    let y_length = end.y - start.y;
    let vector = Point {
        x: x_length.signum(),
        y: y_length.signum()
    };
    let length = max(x_length.abs(), y_length.abs());
    return (0..=length).into_iter()
        .map(|i| Point {
            x: start.x + i * vector.x,
            y: start.y + i * vector.y
        })
        .collect::<Vec<Point>>();
}

fn count_dangerous(instructions: &Vec<(Point,Point)>, count_diagonal: bool) -> usize {
    let mut vents: HashMap<Point,i32> = HashMap::new();
    for (start,end) in instructions {
        if !count_diagonal && (start.x != end.x) && (start.y != end.y) {
            continue;
        }
        let points = generate_points_between(&start, &end);
        for p in points {
            *vents.entry(p).or_insert(0) += 1;
        }
    }
    let num_dangerous = vents.values()
        .filter(|&&x| x > 1)
        .count();
    return num_dangerous;
}

fn part_a(instructions: &Vec<(Point,Point)>) {
    println!("A: {}", count_dangerous(&instructions,false));
}

fn part_b(instructions: &Vec<(Point,Point)>) {
    println!("B: {}", count_dangerous(&instructions,true))
}

fn main() {
    let instructions = load();
    part_a(&instructions);
    part_b(&instructions);
}
