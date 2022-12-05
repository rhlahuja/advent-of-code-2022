use std::collections::VecDeque;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn part_one_mover(
    stacks: &mut [VecDeque<char>],
    source_index: usize,
    destination_index: usize,
    quantity: usize,
) {
    for _ in 0..quantity {
        let character = stacks[source_index].pop_back().unwrap();
        stacks[destination_index].push_back(character);
    }
}

fn part_two_mover(
    stacks: &mut [VecDeque<char>],
    source_index: usize,
    destination_index: usize,
    quantity: usize,
) {
    let source = &stacks[source_index].clone();
    let crates = source.iter().rev().take(quantity).rev();

    stacks[destination_index].extend(crates);
    for _ in 0..quantity {
        stacks[source_index].pop_back();
    }
}

fn move_crates(
    crates: &[Vec<char>],
    instructions: &[&str],
    mover: &dyn Fn(&mut [VecDeque<char>], usize, usize, usize),
) -> String {
    let mut stacks = (0..9)
        .map(|index| {
            VecDeque::from(
                crates
                    .iter()
                    .map(|row| row[index])
                    .filter(|character| *character != ' ')
                    .rev()
                    .collect_vec(),
            )
        })
        .collect_vec();

    for line in instructions {
        let instruction: (&str, &str) = line.split(" from ").collect_tuple().unwrap();
        let quantity = &instruction
            .0
            .split("move ")
            .collect_tuple::<(&str, &str)>()
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let (source_index, destination_index) = &instruction
            .1
            .split(" to ")
            .map(|i| i.parse::<usize>().unwrap() - 1)
            .collect_tuple()
            .unwrap();

        mover(&mut stacks, *source_index, *destination_index, *quantity);

        // println!("instruction: {:?}", &instruction);
        // println!("quantity: {:?}", &quantity);
        // println!("source index: {:?}", &source_index);
        // println!("dest index: {:?}", &destination_index);
    }

    stacks
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    let lines = &input.lines().collect_vec();
    let crates = lines[..8]
        .iter()
        .map(|line| {
            line.chars().collect_vec()[1..]
                .iter()
                .copied()
                .step_by(4)
                .collect_vec()
        })
        .collect_vec();
    let instructions = &lines[10..];

    let part_one_solution = move_crates(&crates.as_slice(), &instructions, &part_one_mover);
    let part_two_solution = move_crates(&crates.as_slice(), &instructions, &part_two_mover);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, "SHMSDGZVC");
    assert_eq!(part_two_solution, "VRZGHDFBQ");
}
