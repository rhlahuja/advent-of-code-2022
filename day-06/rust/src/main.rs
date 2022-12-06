use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn first_marker_position(characters: &[char], marker_length: usize) -> usize {
    for character_count in 0..characters.len() {
        let set: HashSet<char> = HashSet::from_iter(
            characters[cmp::max(0, character_count as i32 - marker_length as i32) as usize
                ..character_count]
                .iter()
                .cloned(),
        );
        if set.len() >= marker_length {
            return character_count;
        }
    }
    0
}

fn main() {
    let characters = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .chars()
    .collect_vec();

    let part_one_solution = first_marker_position(&characters, 4);
    let part_two_solution = first_marker_position(&characters, 14);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 1702);
    assert_eq!(part_two_solution, 3559);
}
