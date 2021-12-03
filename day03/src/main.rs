use std::fs::File;
use std::io::{BufRead, BufReader};

fn load() -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = Vec::new();
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let bits: Vec<char> = line.chars().collect();
        v.push(bits)
    }
    return v;
}

fn part_a(lines: &Vec<Vec<char>>) {
    let mut ones: Vec<u32> = Vec::new();
    let mut zeros: Vec<u32> = Vec::new();
    // Length is all the same for each line
    let bit_length = lines[0].len();
    for i in 0..bit_length {
        ones.push(0);
        zeros.push(0);
        for j in 0..lines.len() {
            match lines[j][i] {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => panic!("Invalid bit")
            }
        }
    }
    let gamma: String = zeros.iter()
        .zip(ones.iter())
        .map(|(z,o)| {
            return match *o > *z {
                true => '1',
                false => '0'
            }
        })
        .collect();
    let epsilon: String = gamma.chars()
        .map(|x| {
            return match x {
                '0' => '1',
                '1' => '0',
                _ => panic!("Invalid char")
            }
        })
        .collect();
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("A: {}", gamma * epsilon);
}

enum RatingType {
    CO2,
    Oxygen
}

fn filter_by_bit_criteria(numbers: &Vec<Vec<char>>, indices: Vec<usize>, bit_index: usize, rating: RatingType) -> Vec<usize>{
    let mut zeros = 0;
    let mut ones = 0;
    indices.iter()
        .map(|&x| numbers[x][bit_index])
        .for_each(|x| {
            match x {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => panic!("Invalid bit")
            }
        });
    let to_match = match rating {
        RatingType::Oxygen => if ones >= zeros { '1' } else {'0' },
        RatingType::CO2 => if zeros <= ones { '0' } else {'1' }
    };
    return indices.into_iter().filter(|&x| numbers[x][bit_index] == to_match).collect();
}

fn part_b(lines: &Vec<Vec<char>>) {
    let bit_length = lines[0].len();
    let mut oxygen_indices: Vec<usize> = (0..lines.len()).collect();
    for i in 0..bit_length {
        oxygen_indices = filter_by_bit_criteria(&lines,oxygen_indices,i,RatingType::Oxygen);
        if oxygen_indices.len() == 1 {
            break;
        }
    }
    let oxygen_rating = lines[oxygen_indices[0]].iter().collect::<String>();
    let oxygen_rating = isize::from_str_radix(&oxygen_rating, 2).unwrap();

    let mut co2_indices: Vec<usize> = (0..lines.len()).collect();
    for i in 0..bit_length {
        co2_indices = filter_by_bit_criteria(&lines,co2_indices,i,RatingType::CO2);
        if co2_indices.len() == 1 {
            break;
        }
    }
    let co2_rating = lines[co2_indices[0]].iter().collect::<String>();
    let co2_rating = isize::from_str_radix(&co2_rating, 2).unwrap();

    println!("B: {}", oxygen_rating * co2_rating);
}

fn main() {
    let lines = load();
    part_a(&lines);
    part_b(&lines);
}
