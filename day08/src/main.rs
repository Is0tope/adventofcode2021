use std::{fs::{read_to_string}, collections::{HashSet, HashMap}};

type Digit = HashSet<char>;
type DigitList = Vec<Digit>;

fn load() -> Vec<(DigitList,DigitList)> {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let ret = text.split("\n").map(|x| {
        let line = x.split(" | ").collect::<Vec<&str>>();
        let input: DigitList  = line[0].split(" ").map(|x| HashSet::from_iter(x.chars())).collect();
        let output: DigitList = line[1].split(" ").map(|x| HashSet::from_iter(x.chars())).collect();
        return (input,output);
    }).collect::<Vec<(DigitList,DigitList)>>();
    return ret;
}

fn get_numbers() -> HashMap<usize,Digit>{
    let mut map: HashMap<usize,Digit> = HashMap::new();
    map.insert(0, HashSet::from_iter("abcefg".chars()));  // 6
    map.insert(1, HashSet::from_iter("cf".chars()));      // 2
    map.insert(2, HashSet::from_iter("acdeg".chars()));   // 5
    map.insert(3, HashSet::from_iter("acdfg".chars()));   // 5
    map.insert(4, HashSet::from_iter("bcdf".chars()));    // 4
    map.insert(5, HashSet::from_iter("abdfg".chars()));   // 5
    map.insert(6, HashSet::from_iter("abdefg".chars()));  // 6
    map.insert(7, HashSet::from_iter("acf".chars()));     // 3
    map.insert(8, HashSet::from_iter("abcdefg".chars())); // 7
    map.insert(9, HashSet::from_iter("abcdfg".chars()));  // 6
    return map;
}

fn get_number_counts(digits: &DigitList) -> HashMap<char,usize>{
    let mut counts: HashMap<char,usize> = HashMap::new();
    for m in digits {
        for c in m {
            *counts.entry(*c).or_insert(0) += 1;
        }
    }
    return counts;
}

fn find_key_for_value<K,V: std::cmp::PartialEq>(map: &HashMap<K,V>, value: V) -> &K {
    for (k,v) in map {
        if *v == value {
            return k;
        }
    }
    panic!("Could not find value in map")
}

fn find_keys_for_value(map: &HashMap<char,usize>, value: usize) -> HashSet<char> {
    let mut ret: HashSet<char> = HashSet::new();
    for (k,v) in map {
        if *v == value {
            ret.insert(*k);
        }
    }
    return ret;
}

fn remap(mapping: &HashMap<char,char>, digit: &Digit) -> Digit {
    return HashSet::from_iter(digit.into_iter().map(|y| *mapping.get(y).unwrap()))
}

fn part_a(instructions: &Vec<(DigitList,DigitList)>) {
    let unique_counts: &[usize] = &[2 ,4, 3, 7];
    let count_1438: usize = instructions.into_iter().map(|ins| {
        let (_,output) = ins;
        return output.into_iter().filter(|&x| unique_counts.contains(&x.len())).count();
    }).sum();
    println!("A: {}", count_1438);
}

fn part_b(instructions: &Vec<(DigitList,DigitList)>) {
    let mut total = 0;
    let numbers = get_numbers();

    for (input, output) in instructions {
        let mut num_mapping: HashMap<usize,Digit> = HashMap::new();
        let mut clear_to_cipher: HashMap<char,char> = HashMap::new();

        // Step 1: Figure out 1, 4, 7 and 8
        num_mapping.insert(1, input.into_iter().filter(|x| x.len() == 2).nth(0).unwrap().clone());
        num_mapping.insert(4, input.into_iter().filter(|x| x.len() == 4).nth(0).unwrap().clone());
        num_mapping.insert(7, input.into_iter().filter(|x| x.len() == 3).nth(0).unwrap().clone());
        num_mapping.insert(8, input.into_iter().filter(|x| x.len() == 7).nth(0).unwrap().clone());

        // Step 2: Figure out the 'a' mapping by subtracting 1 from 7
        let a = num_mapping.get(&7).unwrap().difference(num_mapping.get(&1).unwrap()).into_iter().nth(0).unwrap();
        clear_to_cipher.insert('a', *a);

        // Step 3: Count number of characters in total, and map distinct ones to e, b and f
        let counts = get_number_counts(&input);
        clear_to_cipher.insert('e', *find_key_for_value(&counts, 4));
        clear_to_cipher.insert('b', *find_key_for_value(&counts, 6));
        clear_to_cipher.insert('f', *find_key_for_value(&counts, 9));
        
        // Step 4: Find c by finding the two counts with 8 and subtracting a which also has 8
        let mut c_set = find_keys_for_value(&counts, 8);
        c_set.remove(&*a);
        let c = c_set.into_iter().nth(0).unwrap();
        clear_to_cipher.insert('c',c);

        // Step 5: Find d by subtracting b,c and f from 4
        let d_exclusion_set = HashSet::from_iter("bcf".chars().map(|x| *clear_to_cipher.get(&x).unwrap()));
        let d = num_mapping.get(&4).unwrap().difference(&d_exclusion_set).into_iter().nth(0).unwrap();
        clear_to_cipher.insert('d', *d);

        // Step 6: Find g by subtracting all of the characters from 8
        let g_exclusion_set = HashSet::from_iter("abcdef".chars().map(|x| *clear_to_cipher.get(&x).unwrap()));
        let g = num_mapping.get(&8).unwrap().difference(&g_exclusion_set).into_iter().nth(0).unwrap();
        clear_to_cipher.insert('g', *g);

        // Step 7: Create the cipher to clear map by reversing the clear to cipher map
        let cipher_to_clear: HashMap<char,char> = HashMap::from_iter(clear_to_cipher.iter().map(|(&k,&v)| (v,k)));
        
        // Step 8: Convert the output numbers to their correct mappings, and reference them against the real numbers
        let output_number: usize = output.into_iter()
            .map(|x| remap(&cipher_to_clear, x))
            .map(|x| *find_key_for_value(&numbers, x))
            .reduce(|a,b|a * 10 + b)
            .unwrap();
        
        total += output_number;
    }
    println!("B: {}", total);
}

fn main() {
    get_numbers();
    let instructions = load();
    part_a(&instructions);
    part_b(&instructions);
}
