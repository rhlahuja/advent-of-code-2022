import pathlib


SHAPE_MAP = {'x': 1, 'y': 2, 'z': 3}
OUTCOME_MAP = {
    'a': {'x': 3, 'y': 6, 'z': 0},
    'b': {'x': 0, 'y': 3, 'z': 6},
    'c': {'x': 6, 'y': 0, 'z': 3},
}


def total_score(rounds: list[list[str]]) -> int:
    return sum(
        SHAPE_MAP[my_shape] + OUTCOME_MAP[opponent_shape][my_shape]
        for opponent_shape, my_shape in rounds
    )


def part_two(rounds: list[list[str]]) -> int:
    move_map = {
        'a': {'x': 'z', 'y': 'x', 'z': 'y'},
        'b': {'x': 'x', 'y': 'y', 'z': 'z'},
        'c': {'x': 'y', 'y': 'z', 'z': 'x'},
    }
    return total_score(
        [
            [opponent_shape, move_map[opponent_shape][instructed_outcome]]
            for opponent_shape, instructed_outcome in rounds
        ]
    )


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    rounds = [line.split() for line in f.read().lower().splitlines()]


part_one_solution = total_score(rounds)
part_two_solution = part_two(rounds)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 10310
assert part_two_solution == 14859
