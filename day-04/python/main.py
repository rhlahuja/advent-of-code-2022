import pathlib
from typing import Callable


def filtered_range_count(
    ranges: list[list[set[int]]], condition: Callable[[list[set[int]]], bool]
) -> int:
    return len(list(filter(condition, ranges)))


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    ranges = [
        [set(range(start, end + 1)) for start, end in interval]
        for interval in [
            [[int(bound) for bound in bounds.split('-')] for bounds in bounded_range]
            for bounded_range in [line.split(',') for line in f.read().splitlines()]
        ]
    ]


part_one_solution = filtered_range_count(
    ranges, lambda pair: pair[0].issubset(pair[1]) or pair[1].issubset(pair[0])
)
part_two_solution = filtered_range_count(
    ranges, lambda pair: pair[0].intersection(pair[1])
)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 532
assert part_two_solution == 854
