use std::collections::HashSet;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn filtered_range_count(
    ranges: &Vec<Vec<HashSet<i32>>>,
    condition: &dyn Fn(Vec<&HashSet<i32>>) -> bool,
) -> i32 {
    ranges
        .iter()
        .filter(|pair| condition(pair.iter().collect_vec()))
        .collect_vec()
        .len() as i32
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    let ranges = input
        .lines()
        .map(|line| {
            line.split(",")
                .into_iter()
                .map(|bounded_range| {
                    bounded_range
                        .split("-")
                        .into_iter()
                        .map(|bound| bound.parse().unwrap())
                        .collect_vec()
                })
                .collect_vec()
                .iter()
                .map(|bounds| HashSet::from_iter(bounds[0]..bounds[1] + 1))
                .collect_vec()
        })
        .collect();

    let part_one_solution = filtered_range_count(&ranges, &|pair| {
        (&pair[0]).is_subset(&pair[1]) || (&pair[1]).is_subset(&pair[0])
    });
    let part_two_solution = filtered_range_count(&ranges, &|pair| {
        !(&pair[0]).intersection(&pair[1]).collect_vec().is_empty()
    });

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 532);
    assert_eq!(part_two_solution, 854);
}
