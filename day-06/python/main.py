import pathlib


def first_marker_position(characters: str, marker_length: int) -> int:
    for character_count in range(1, len(characters)):
        if (
            len(set(characters[character_count - marker_length : character_count]))
            >= marker_length
        ):
            return character_count


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    characters = f.read()


part_one_solution = first_marker_position(characters, marker_length=4)
part_two_solution = first_marker_position(characters, marker_length=14)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 1702
assert part_two_solution == 3559
