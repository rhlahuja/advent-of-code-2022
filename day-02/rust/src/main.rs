use std::collections::HashMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn total_score(rounds: &[Vec<&str>]) -> i32 {
    let shape_map = HashMap::from([("x", 1), ("y", 2), ("z", 3)]);
    let outcome_map = HashMap::from([
        ("a", HashMap::from([("x", 3), ("y", 6), ("z", 0)])),
        ("b", HashMap::from([("x", 0), ("y", 3), ("z", 6)])),
        ("c", HashMap::from([("x", 6), ("y", 0), ("z", 3)])),
    ]);

    rounds
        .iter()
        .map(|moves| {
            shape_map.get(moves[1]).unwrap()
                + outcome_map.get(moves[0]).unwrap().get(moves[1]).unwrap()
        })
        .sum()
}

fn part_two(rounds: &[Vec<&str>]) -> i32 {
    let move_map = HashMap::from([
        ("a", HashMap::from([("x", "z"), ("y", "x"), ("z", "y")])),
        ("b", HashMap::from([("x", "x"), ("y", "y"), ("z", "z")])),
        ("c", HashMap::from([("x", "y"), ("y", "z"), ("z", "x")])),
    ]);
    total_score(
        &rounds
            .iter()
            .map(|instructions| {
                vec![
                    instructions[0],
                    move_map
                        .get(instructions[0])
                        .unwrap()
                        .get(instructions[1])
                        .unwrap(),
                ]
            })
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
    .unwrap()
    .to_lowercase();

    let rounds: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect();

    let part_one_solution = total_score(&rounds);
    let part_two_solution = part_two(&rounds);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 10310);
    assert_eq!(part_two_solution, 14859);
}
