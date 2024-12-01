use std::fs::read_to_string;

pub fn solve_puzzle_1() -> u64 {
    let mut location_id_list_1: Vec<u64> = vec![];
    let mut location_id_list_2: Vec<u64> = vec![];

    let input_str =
        read_to_string("src/input/puzzle_1.txt").expect("Failed to read the input file.");

    for line in input_str.split_terminator("\n") {
        let id_pair = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        location_id_list_1.push(id_pair[0]);
        location_id_list_2.push(id_pair[1]);
    }

    location_id_list_1.sort();
    location_id_list_2.sort();

    location_id_list_1
        .into_iter()
        .zip(location_id_list_2)
        .map(|(id1, id2)| u64::abs_diff(id1, id2))
        .sum()
}
