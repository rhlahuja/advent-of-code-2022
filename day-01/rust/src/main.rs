use std::fs;
use std::path::Path;

use itertools::Itertools;

fn part_two(sums: &mut Vec<i32>) -> i32 {
    sums.sort();
    sums.reverse();
    sums[..3].iter().sum()
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    let sums: Vec<_> = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|line| line.parse().unwrap())
                .collect_vec()
                .iter()
                .sum()
        })
        .collect();

    let part_one_solution = *sums.iter().max().unwrap();
    let part_two_solution = part_two(&mut sums.clone());

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 75622);
    assert_eq!(part_two_solution, 213159);
}
