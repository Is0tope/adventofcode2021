use std::{fs::{read_to_string}, collections::HashMap};

type Template = HashMap<String,char>;
type Mapper = HashMap<String,Vec<String>>;

fn load() -> (Vec<char>, Template) {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let mut iter = text.split("\n");
    let pattern: Vec<char> = iter.next().unwrap().chars().collect();
    let mut template: Template = Template::new();
    iter.next();
    for line in iter {
        let tmp: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
        template.insert(tmp[0].clone(), tmp[1].chars().nth(0).unwrap());
    }
    return (pattern,template);
}

fn get_counts(pattern: &Vec<char>, template: &Template, steps: usize) -> HashMap<String,u128> {
    let mut mapper: Mapper = HashMap::new();
    for k in template.keys() {
        let c = template.get(k).unwrap();
        let mut v = Vec::new();
        let s1 = &[k.chars().nth(0).unwrap(),*c].iter().collect::<String>();
        v.push(s1.to_string());
        let s2 = &[*c,k.chars().nth(1).unwrap()].iter().collect::<String>();
        v.push(s2.to_string());
        mapper.insert(k.to_string(), v);
    }
    
    let mut counts: HashMap<String,u128> = HashMap::new();
    for i in 1..pattern.len() {
        let key: String = Vec::from_iter(pattern[(i-1)..=i].into_iter()).into_iter().collect();
        *counts.entry(key.to_string()).or_insert(0) += 1;
    }
    
    for _ in 0..steps {
        let mut new_counts: HashMap<String,u128> = HashMap::new();
        for k in counts.keys() {
            let new_keys = mapper.get(&*k).unwrap();
            let quantity = counts.get(&*k).unwrap();
            new_keys.into_iter().for_each(|x| *new_counts.entry(x.to_string()).or_insert(0) += quantity);
        }
        counts = new_counts;
    }
    return counts;
}

fn reduce_counts(counts: &HashMap<String,u128>, first_char: char, last_char: char) -> HashMap<char,u128> {
    let mut ret: HashMap<char,u128> = HashMap::new();
    let first_last = vec![first_char,last_char];
    for k in counts.keys() {
        let quantity = counts.get(k).unwrap();
        k.chars().for_each(|x| *ret.entry(x).or_insert(0) += quantity);
    }
    let mut ret_final: HashMap<char,u128> = HashMap::new();
    for k in ret.keys() {
        let quantity = ret.get(k).unwrap();
        let val = match first_last.contains(k) {
            true => 1 + (*quantity - 1) / 2,
            false => *quantity / 2
        };
        ret_final.insert(*k, val);
    }
    return ret_final;
}

fn part_a(pattern: &Vec<char>, template: &Template) {
    let counts = get_counts(pattern, template, 10);
    let counts = reduce_counts(&counts,pattern[0],pattern[pattern.len() - 1]);
    println!("A: {}", counts.values().max().unwrap() - counts.values().min().unwrap());
}

fn part_b(pattern: &Vec<char>, template: &Template) {
    let counts = get_counts(pattern, template, 40);
    let counts = reduce_counts(&counts,pattern[0],pattern[pattern.len() - 1]);
    println!("B: {}", counts.values().max().unwrap() - counts.values().min().unwrap());
}

fn main() {
    let (pattern,template) = load();
    part_a(&pattern,&template);
    part_b(&pattern,&template);
}
