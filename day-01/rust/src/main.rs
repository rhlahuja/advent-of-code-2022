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

    let sums: Vec<i32> = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|line| line.parse().unwrap())
                .collect::<Vec<i32>>()
                .iter()
                .sum()
        })
        .collect();

    let part_one = part_one(sums.as_slice());
    let part_two = part_two(&mut sums.clone());

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);

    assert_eq!(part_one, 75622);
    assert_eq!(part_two, 213159);
}
