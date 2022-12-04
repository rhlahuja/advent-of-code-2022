import pathlib
import string
from typing import Iterable


def common_character(strings: list[str]) -> str:
    return set.intersection(*map(set, strings)).pop()


def sum_priorities(strings: Iterable[list[str]]) -> int:
    priority_map = {
        letter: priority
        for priority, letter in enumerate(string.ascii_letters, start=1)
    }
    return sum(priority_map[common_character(group)] for group in strings)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    rucksacks = f.read().splitlines()


part_one_solution = sum_priorities(
    [rucksack[: len(rucksack) // 2], rucksack[len(rucksack) // 2 :]]
    for rucksack in rucksacks
)
part_two_solution = sum_priorities(
    rucksacks[i : i + 3] for i in range(0, len(rucksacks), 3)
)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 8018
assert part_two_solution == 2518
