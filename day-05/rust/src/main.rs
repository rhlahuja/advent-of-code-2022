use std::fs;
use std::path::Path;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    // println!("Part One: {}", part_one_solution);
    // println!("Part Two: {}", part_two_solution);

    // assert_eq!(part_one_solution, "SHMSDGZVC');
    // assert_eq!(part_two_solution, "VRZGHDFBQ);
}
