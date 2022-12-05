import pathlib
from collections import deque
from typing import Callable, NoReturn


def part_one(source: deque, destination: deque, quantity: int):
    for _ in range(quantity):
        destination.appendleft(source.popleft())


def part_two(source: deque, destination: deque, quantity: int):
    destination.extendleft(reversed(list(source)[:quantity]))
    for _ in range(quantity):
        source.popleft()


def move_crates(lines: list[str], mover: Callable[[deque, deque, int], NoReturn]):
    crates = [line[i] for i in range(1, 36, 4) for line in lines[:9]]
    stacks = [
        deque(position for position in crates[i : i + 8] if position != ' ')
        for i in range(0, len(crates), 9)
    ]

    for line in lines[10:]:
        instruction = line.split(' from ')
        quantity = int(instruction[0].split('move ')[1])
        source, destination = [
            stacks[int(stack_number) - 1]
            for stack_number in instruction[1].split(' to ')
        ]
        mover(source, destination, quantity)
    return ''.join(stack[0] for stack in stacks)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    lines = f.read().splitlines()


part_one_solution = move_crates(lines, part_one)
part_two_solution = move_crates(lines, part_two)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 'SHMSDGZVC'
assert part_two_solution == 'VRZGHDFBQ'
