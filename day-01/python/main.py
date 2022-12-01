import pathlib


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    sums = [
        sum(int(c) for c in calories.splitlines())
        for calories in f.read().split('\n\n')
    ]


part_one_solution = max(sums)
part_two_solution = sum(sorted(sums, reverse=True)[:3])

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 75622
assert part_two_solution == 213159
