import pathlib
from collections import deque
from typing import Callable, NoReturn


def part_one_mover(source: deque, destination: deque, quantity: int):
    for _ in range(quantity):
        destination.append(source.pop())


def part_two_mover(source: deque, destination: deque, quantity: int):
    destination.extend(list(source)[-quantity:])
    for _ in range(quantity):
        source.pop()


def move_crates(
    crates: list[str],
    instructions: list[str],
    mover: Callable[[deque, deque, int], NoReturn],
):
    stacks = [
        deque(reversed([position for position in crates[i : i + 8] if position != ' ']))
        for i in range(0, len(crates), 9)
    ]
    for line in instructions:
        instruction = line.split(' from ')
        quantity = int(instruction[0].split('move ')[1])
        source, destination = (
            stacks[int(stack_number) - 1]
            for stack_number in instruction[1].split(' to ')
        )
        mover(source, destination, quantity)
    return ''.join(stack[-1] for stack in stacks)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    lines = f.read().splitlines()
    crates = [line[i] for i in range(1, 36, 4) for line in lines[:9]]
    instructions = lines[10:]


part_one_solution = move_crates(
    crates=crates, instructions=instructions, mover=part_one_mover
)
part_two_solution = move_crates(
    crates=crates, instructions=instructions, mover=part_two_mover
)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 'SHMSDGZVC'
assert part_two_solution == 'VRZGHDFBQ'
