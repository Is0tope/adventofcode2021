use std::{fs::{read_to_string}, collections::{HashMap, BinaryHeap}, cmp::{Ordering, max}};

type Grid = HashMap<Point,u32>;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
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

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct State {
    pos: Point,
    cost: u32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.cost.cmp(&self.cost);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn load() -> Grid {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let mut grid: Grid = Grid::new();
    text.split("\n").enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x,c)| {
            let p = Point { x: x as i32, y: y as i32};
            grid.insert(p, c.to_digit(10).unwrap());
        });
    });
    return grid;
}

fn find_shortest_path(grid: &Grid, start: &Point, end: &Point) -> u32 {
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let mut distance: HashMap<Point,u32> = HashMap::new();
    let initial_state = State {
        pos: start.clone(),
        cost: 0
    };
    frontier.push(initial_state);
    distance.insert(*start, 0);

    while let Some(state) = frontier.pop() {
        // If reached target break out
        if state.pos == *end {
            return state.cost;
        }
        // If this cost is worse than what we have already, skip
        if state.cost > *distance.entry(state.pos).or_insert(u32::MAX) {
            continue;
        }
        // For each of the points neighbours, add it to the frontier if viable
        for neighbour in state.pos.get_neighbours() {
            if let Some(risk) = grid.get(&neighbour) {
                let new_risk = state.cost + risk;
                if new_risk < *distance.entry(neighbour).or_insert(u32::MAX) {
                    frontier.push(State { pos: neighbour, cost: new_risk });
                    distance.insert(neighbour, new_risk);
                }
            }
        }
    }
    return 0;
}


fn wrap_risk(initial: u32, delta: u32) -> u32 {
    let tmp = initial + delta;
    let num_wrapped = tmp / 10;
    return (tmp + num_wrapped) % 10;
}

fn part_a(grid: &Grid) {
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: grid.keys().map(|p| p.x).max().unwrap(),
        y: grid.keys().map(|p| p.y).max().unwrap()
    };
    let shortest_path = find_shortest_path(&grid, &start, &end);
    println!("A: {}", shortest_path);
}

fn part_b(grid: &Grid) {
    // Expand the grid
    let size = 1 + grid.keys().map(|p| p.x).max().unwrap();
    let size = max(size,1 + grid.keys().map(|p| p.y).max().unwrap());
    let mut large_grid: Grid = HashMap::new();
    for y in 0..5 {
        for x in 0..5 {
            for p in grid.keys() {
                let new_p = Point { x: x * size + p.x, y: y * size + p.y};
                let risk = *grid.get(p).unwrap();
                let risk = wrap_risk(risk, x as u32 + y as u32);
                large_grid.insert(new_p, risk);
            }
        }
    }
    // Calculate the shortest path
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: large_grid.keys().map(|p| p.x).max().unwrap(),
        y: large_grid.keys().map(|p| p.y).max().unwrap()
    };
    let shortest_path = find_shortest_path(&large_grid, &start, &end);
    println!("B: {}", shortest_path);
}

fn main() {
    let grid = load();
    part_a(&grid);
    part_b(&grid);
}
