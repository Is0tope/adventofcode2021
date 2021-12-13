use std::{fs::{read_to_string}, collections::HashSet, cmp::max};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Left
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Fold {
    dir: Direction,
    coord: i32
}

type Grid = HashSet<Point>;

impl Point {
    fn fold(self: Point, fold: &Fold) -> Point{
        let c = fold.coord;
        return match fold.dir {
            Direction::Left => Point { x: c - (c - self.x).abs(), y: self.y },
            Direction::Up => Point { x: self.x, y: c - (c - self.y).abs() }
        };
    }
}

fn load() ->  (Grid, Vec<Fold>){
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let mut grid = Grid::new();
    let mut iter = text.split("\n");
    loop {
        let line =  iter.next().unwrap();
        if line.is_empty() {
            break
        }
        let tmp: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        grid.insert(Point {
            x: tmp[0],
            y: tmp[1]
        });
    }
    let mut folds = Vec::new();
    for line in iter {
        let tmp: Vec<&str> = line.split("=").collect();
        let dir = match tmp[0].split(" ").nth(2).unwrap() {
            "x" => Direction::Left,
            "y" => Direction::Up,
            _ => panic!("unknown fold")
        };
        let coord: i32 = tmp[1].parse().unwrap();
        folds.push(Fold {
            dir,
            coord
        })
    }
    return (grid,folds);
}

fn print_grid(grid: &Grid, size: i32) {
    for y in 0..size {
        for x in 0..size {
            let ch: String = match grid.get(&Point {x:x, y:y}) {
                Some(_) => "#".to_string(),
                None => " ".to_string()
            };
            print!("{}",ch);
        }
        println!("");
    }
}

fn apply_folds_to_grid(grid: &Grid, folds: &Vec<Fold>) -> Grid {
    let mut grid = grid.clone();
    for f in folds {
        let mut grid_swap = Grid::new();
        for p in &grid {
            grid_swap.insert(p.fold(f));
        }
        grid = grid_swap;
    }
    return grid;
}

fn part_a(grid: &Grid, folds: &Vec<Fold>) {
    let first_fold: Vec<Fold> = folds.into_iter().take(1).map(|x|*x).collect();
    let grid = apply_folds_to_grid(&grid, &first_fold);
    println!("A: {}", grid.len());
}

fn part_b(grid: &Grid, folds: &Vec<Fold>) {
    let grid = apply_folds_to_grid(&grid, &folds);
    let max_size = 1 + *&grid.iter()
        .map(|p| max(p.x,p.y))
        .max()
        .unwrap();
    println!("B:");
    print_grid(&grid, max_size);
}

fn main() {
    let (grid,folds) = load();
    part_a(&grid, &folds);
    part_b(&grid, &folds);
}
