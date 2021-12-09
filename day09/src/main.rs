use std::{fs::{read_to_string}, collections::{HashMap, HashSet}};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

type Grid = HashMap<Point,u32>;

fn load() -> Grid {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let mut grid: Grid = Grid::new();
    for (y,line) in text.split("\n").enumerate() {
        for (x,c) in line.chars().enumerate() {
            let p = Point { x: x as i32, y: y as i32};
            grid.insert(p, c.to_digit(10).unwrap());
        }
    }
    return grid;
}

impl Point {
    fn get_neighbours(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        ret.push(Point { x: self.x + 1, y: self.y    });
        ret.push(Point { x: self.x - 1, y: self.y    });
        ret.push(Point { x: self.x,     y: self.y + 1});
        ret.push(Point { x: self.x,     y: self.y - 1});
        return ret;
    }
}

fn get_sinks(grid: &Grid) -> Vec<Point> {
    let mut points = Vec::new();
    for p in grid.keys() {
        let value = *grid.get(p).unwrap();
        let neighbours = p.get_neighbours();
        let is_lower = neighbours.into_iter().all(|x| {
            return match grid.get(&x) {
                None => true,
                Some(v) => *v > value
            }
        });
        if is_lower {
            points.push(*p);
        }
    }
    return points;
}

fn find_basin(grid: &Grid, sink: Point) -> HashSet<Point> {
    let basin_height = 9;
    let mut seen: HashSet<Point> = HashSet::new();
    let mut queue = vec![sink];
    while queue.len() > 0{
        let point = queue.pop().unwrap();
        seen.insert(point);
        let neighbours = point.get_neighbours();
        for n in neighbours {
            if seen.contains(&n) {
                continue;
            }
            let accept = match grid.get(&n) {
                Some(v) => *v < basin_height,
                None => false
            };
            if accept {
                queue.push(n);
            }
        }
    }
    return seen;
}

fn part_a(grid: &Grid) {
    let risk_score: u32 = get_sinks(&grid).into_iter()
        .map(|x| *grid.get(&x).unwrap() + 1)
        .sum();
    println!("A: {}", risk_score);
}

fn part_b(grid: &Grid) {
    let sinks = get_sinks(&grid);
    let mut basin_sizes: Vec<usize> = Vec::new();
    for s in sinks {
        let basin = find_basin(&grid, s);
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort();
    let total: usize = basin_sizes.into_iter()
        .rev()
        .take(3)
        .reduce(|x,y| x * y)
        .unwrap();
    println!("B: {}", total)
}

fn main() {
    let grid = load();
    part_a(&grid);
    part_b(&grid);
}
