import pathlib


def part_one(sums: list[int]) -> int:
    return max(sums)


def part_two(sums: list[int]) -> int:
    return sum(sorted(sums, reverse=True)[0:3])


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    sums = [
        sum(int(c) for c in calories.splitlines())
        for calories in f.read().split('\n\n')
    ]


part_one = part_one(sums)
part_two = part_two(sums)

print('Part One:', part_one)
print('Part Two:', part_two)

assert part_one == 75622
assert part_two == 213159
