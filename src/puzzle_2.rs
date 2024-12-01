use std::{collections::HashMap, fs::read_to_string};

pub fn solve_puzzle_2() -> u64 {
    let mut location_id_list_1: Vec<u64> = vec![];
    let mut location_id_list_2_counts: HashMap<u64, u64> = HashMap::new();

    let input_str =
        read_to_string("src/input/puzzle_1.txt").expect("Failed to read the input file.");

    for line in input_str.split_terminator("\n") {
        let id_pair = line
            .split_ascii_whitespace()
            .map(|id| id.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        location_id_list_1.push(id_pair[0]);
        if !location_id_list_2_counts.contains_key(&id_pair[1]) {
            location_id_list_2_counts.insert(id_pair[1], 1);
        } else {
            location_id_list_2_counts
                .entry(id_pair[1])
                .and_modify(|counter| *counter += 1);
        }
    }

    location_id_list_1
        .into_iter()
        .filter(|id| location_id_list_2_counts.contains_key(&id))
        .map(|id| id * location_id_list_2_counts.get(&id).unwrap())
        .sum()
}
