use std::{fs::{read_to_string}};

type Code = Vec<char>;

fn load() -> Vec<Code> {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    return text.split("\n").map(|x|
        x.chars().collect::<Code>()
    ).collect();
}

fn is_open_char(c: &char) -> bool{
    return *&['<','[','{','('].contains(c);
}

fn open_to_close_char(c: &char) -> char {
    return match c {
        '{' => '}',
        '[' => ']',
        '<' => '>',
        '(' => ')',
        _ => panic!("Unknown character")
    }
}

fn check_corrupted(code: &Code) -> (bool,char) {
    let mut stack: Code = Vec::new();
    for c in code {
        if is_open_char(c) {
            stack.push(*c);
        } else {
            if stack.len() == 0 {
                return (true,*c);
            }
            let current_open = stack.pop().unwrap();
            if *c != open_to_close_char(&current_open) {
                return (true,*c)
            }
        }
    }
    return (false,' ');
}

fn complete_code(code: &Code) -> Vec<char> {
    let mut stack: Code = Vec::new();
    for c in code {
        if is_open_char(c) {
            stack.push(*c);
        } else {
            stack.pop().unwrap();
        }
    }
    let mut completion: Vec<char> = Vec::new();
    for c in stack.into_iter().rev() {
        completion.push(open_to_close_char(&c));
    }
    return completion;
}

fn score_char(c: &char) -> u64 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1_197,
        '>' => 25_137,
        _ => panic!("Unknown character")
    } 
}

fn score_completion(comp: &Vec<char>) -> u64 {
    let mut total = 0;
    for c in comp {
        let score = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unknown character")
        };
        total *= 5;
        total += score;
    }
    return total;
}

fn part_a(code: &Vec<Code>) {
    let mut total = 0;
    for cd in code {
        let (is_corrupted,c) = check_corrupted(&cd);
        if is_corrupted {
            total += score_char(&c);
        }
    }
    println!("A: {}", total);
}

fn part_b(code: &Vec<Code>) {
    let mut scores = Vec::new();
    for cd in code {
        let (is_corrupted,_) = check_corrupted(&cd);
        if is_corrupted {
            continue;
        }
        let completion = complete_code(&cd);
        scores.push(score_completion(&completion));
    }
    scores.sort();
    println!("B: {}", scores[scores.len()/2])
}

fn main() {
    let code = load();
    part_a(&code);
    part_b(&code);
}
