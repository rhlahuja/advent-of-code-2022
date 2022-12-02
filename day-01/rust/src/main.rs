use std::fs;
use std::path::Path;

fn part_one(sums: &[i32]) -> i32 {
    *sums.iter().max().unwrap()
}

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
                .collect::<Vec<_>>()
                .iter()
                .sum()
        })
        .collect();

    let part_one_solution = part_one(&sums);
    let part_two_solution = part_two(&mut sums.clone());

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 75622);
    assert_eq!(part_two_solution, 213159);
}
