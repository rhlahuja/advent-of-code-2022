use std::collections::HashMap;
use std::fs;
use std::path::Path;

// fn common_character(mut strings: Vec<String>) -> char {
//     let (common_character, others) = strings.split_at_mut(1);
//     let common_character = &mut common_character[0];
//     for other in others {
//         common_character.retain(|e| other.contains(e));
//     }
//     common_character.iter().next()
// }

// fn sum_priorities() -> HashMap<char, usize> {
//     let mut priority_map: HashMap<char, usize> = HashMap::new();
//     for (i, character) in ('a'..='z').chain('A'..='Z').into_iter().enumerate() {
//         priority_map.insert(character, i + 1);
//     }
//     priority_map
// }

fn main() {
    let rucksacks: Vec<_> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse::<String>().unwrap())
    .collect();

    // let part_one_solution = part_one(&rucksacks);
    // let part_two_solution = part_two(&rucksacks);

    // println!("Part One: {:?}", sum_priorities());
    // println!("Part Two: {}", part_two_solution);

    // assert_eq!(part_one_solution, 10310);
    // assert_eq!(part_two_solution, 14859);
}
