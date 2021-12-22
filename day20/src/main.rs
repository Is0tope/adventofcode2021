use std::{fs::{read_to_string}, collections::{HashSet, HashMap}};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn get_grid_3(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        // Order is important here
        for y in -1..=1 {
            for x in -1..=1 { 
                ret.push(Point { x: self.x + x, y: self.y + y });
            }
        }
        return ret;
    }
}

type Grid = HashMap<Point,bool>;
type Algo = HashSet<usize>;

fn load() -> (Grid,Algo) {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let strs: Vec<String> = text.split("\n").map(|x| x.to_string()).collect();
    let algo = strs[0].chars().enumerate()
        .filter(|(_,c)| *c == '#')
        .map(|(i,_)| i);
    let algo = HashSet::from_iter(algo);
    let mut grid = Grid::new();
    for (y, line) in strs[2..].into_iter().enumerate() {
        for (x,c) in line.chars().enumerate() {
            let p = Point { x: x as i32, y: y as i32};
            let b = match c {
                '#' => true,
                '.' => false,
                _ => panic!("unrecognised symbol")
            };
            grid.insert(p,b);
        }
    }
    return (grid,algo);
}

fn enhance_image(grid: &Grid, algo: &Algo, cycles: usize) -> Grid {
    let mut grid = grid.clone();
    let mut new_grid;
    let mut current_unknown = 0;
    for _ in 0..cycles {
        new_grid = Grid::new();
        let mut possible_points: HashSet<Point> = HashSet::new();
        // Get all possible points that could change
        for p in grid.keys() {
            let neighbours = p.get_grid_3();
            for x in neighbours {
                possible_points.insert(x);
            }
        }
        // For each point that could change, update the new grid
        for p in possible_points {
            let lookup_index: String = p.get_grid_3().into_iter().map(|x| {
                match grid.get(&x) {
                    Some(x) => match x {
                        true => '1',
                        false => '0'
                    },
                    None => match current_unknown {
                        0 => '0',
                        511 => '1',
                        _ => panic!("invalid current_unknown")
                    }
                }
            })
            .collect();
            let lookup_index: usize = isize::from_str_radix(&lookup_index, 2).unwrap().try_into().unwrap();
            if algo.contains(&lookup_index) {
                new_grid.insert(p, true);
            } else {
                new_grid.insert(p, false);
            }
        }
        grid = new_grid;
        // Figure out what the current unknown should be
        current_unknown = match algo.contains(&current_unknown) {
            true => 511,
            false => 0
        };

    }
    return grid;
}

// fn print_grid(grid: &Grid) {
//     let min_x = grid.keys().map(|p| p.x).min().unwrap();
//     let max_x = grid.keys().map(|p| p.x).max().unwrap();
//     let min_y = grid.keys().map(|p| p.y).min().unwrap();
//     let max_y = grid.keys().map(|p| p.y).max().unwrap();
//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             let p = Point { x, y };
//             print!("{}",match grid.get(&p) {
//                 Some(x) => match x {
//                     true => '#',
//                     false => '.'
//                 },
//                 None => ' '
//             });
//         }
//         println!("");
//     }
// }

fn part_a(grid: &Grid, algo: &Algo) {
    let grid = enhance_image(&grid, &algo, 2);
    println!("A: {}", grid.values().filter(|&x| *x).count());
}

fn part_b(grid: &Grid, algo: &Algo) {
    let grid = enhance_image(&grid, &algo, 50);
    println!("B: {}", grid.values().filter(|&x| *x).count());}

fn main() {
    let (grid,algo) = load();
    part_a(&grid,&algo);
    part_b(&grid,&algo);
}
