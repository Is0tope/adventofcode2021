use std::{fs::{read_to_string}, collections::HashMap};

type CaveMap = HashMap<String,Vec<String>>;
type TravelState = Vec<String>;

fn load() -> CaveMap {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let mut caves = CaveMap::new(); 
    text.split("\n").for_each(|line| {
        let mut tmp = line.split("-");
        let start = tmp.next().unwrap().to_string();
        let end = tmp.next().unwrap().to_string();
        caves.entry(start.clone())
            .or_insert(Vec::new())
            .push(end.clone());
        caves.entry(end)
            .or_insert(Vec::new())
            .push(start);
    });
    return caves;
}

fn is_small_cave(cave: String) -> bool {
    return cave.chars().all(|c| c.is_lowercase());
}

fn is_terminal_state(state: &TravelState) -> bool{
    return state.contains(&"end".to_string());
}

fn has_multiple_small_visit(state: &TravelState, max_visits: usize) -> bool {
    // If rust allowed me to use group_by, this would not be necessary
    let mut map: HashMap<String,usize> = HashMap::new();
    for s in state {
        if is_small_cave(s.to_string()) {
            *map.entry(s.to_string()).or_insert(0) += 1;
        }
    }
    return map.values().any(|&x| x >= max_visits);
}

fn get_travel_options(state: &TravelState, caves: &CaveMap, part_b: bool) -> Vec<TravelState> {
    let last_visited = state.last().unwrap();
    let options = caves.get(last_visited).unwrap();
    let mut ret: Vec<TravelState> = Vec::new();
    for o in options {
        // Ignore the start cave
        if o == "start" {
            continue;
        }
        // If this is a small cave, and we have been there before, then ignore
        let mut accept = true;
        if is_small_cave(o.to_string()) {
            if !part_b {
                accept = !state.contains(o);
            } else {
                if has_multiple_small_visit(&state, 2) {
                    accept = !state.contains(o);
                }
            }
        }
        // TODO: In theory if two or more large caves are next to each other it allows an
        //       infinite loop. Current input does not have this issue however, so ignore.
        if accept {
            let mut new_state = state.clone();
            new_state.push(o.to_string());
            ret.push(new_state);
        }
    }
    return ret;
}

fn count_number_paths(caves: &CaveMap, part_b: bool) -> usize {
    let initial_state: TravelState = vec!["start".to_string()];
    let mut terminal_states = Vec::new();
    let mut queue: Vec<TravelState> = Vec::new();
    queue.push(initial_state);
    while queue.len() > 0 {
        let state = queue.pop().unwrap();
        if is_terminal_state(&state) {
            terminal_states.push(state);
            continue;
        }
        let mut next_states = get_travel_options(&state, &caves, part_b);
        queue.append(&mut next_states);
    }
    return terminal_states.len();
}

fn part_a(caves: &CaveMap) {
    let num_paths = count_number_paths(&caves, false);
    println!("A: {}", num_paths);
}

fn part_b(caves: &CaveMap) {
    let num_paths = count_number_paths(&caves, true);
    println!("B: {}", num_paths);
}

fn main() {
    let caves = load();
    part_a(&caves);
    part_b(&caves);
}
