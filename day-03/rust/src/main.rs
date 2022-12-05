use std::collections::HashMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn common_character(strings: &[&str]) -> char {
    let common_character = &mut strings[0].to_string();
    for string in strings {
        common_character.retain(|e| string.contains(e));
    }
    common_character.chars().next().unwrap()
}

fn sum_priorities(strings: &Vec<Vec<&str>>) -> i32 {
    let mut priority_map: HashMap<char, i32> = HashMap::new();
    for (i, character) in ('a'..='z').chain('A'..='Z').into_iter().enumerate() {
        priority_map.insert(character, i as i32 + 1);
    }

    strings
        .iter()
        .map(|strings| {
            *priority_map
                .get(&common_character(strings.as_slice()))
                .unwrap()
        })
        .collect_vec()
        .iter()
        .sum()
}

fn part_one(rucksacks: &Vec<&str>) -> i32 {
    sum_priorities(
        &rucksacks
            .iter()
            .map(|rucksack| {
                let (compartment_one, compartment_two) = rucksack.split_at(rucksack.len() / 2);
                vec![compartment_one, compartment_two]
            })
            .collect_vec(),
    )
}

fn part_two(rucksacks: &Vec<&str>) -> i32 {
    sum_priorities(
        &rucksacks
            .into_iter()
            .copied()
            .chunks(3)
            .into_iter()
            .map(|rucksacks| rucksacks.collect_vec())
            .collect_vec(),
    )
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let rucksacks = input.lines().collect();

    let part_one_solution = part_one(&rucksacks);
    let part_two_solution = part_two(&rucksacks);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 8018);
    assert_eq!(part_two_solution, 2518);
}
