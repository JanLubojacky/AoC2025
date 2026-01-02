import re
from pathlib import Path

from ortools.linear_solver import pywraplp


def read_input(path: Path):
    out = []

    with open(path) as fp:
        lines = fp.readlines()

    for line in lines:
        splits = line.split(" ")

        diagram = splits[0]
        buttons = splits[1:-1]
        joltage = splits[-1]

        diagram = re.sub(r"\[|\]", "", diagram)
        buttons = [list(map(int, re.sub(r"\(|\)", "", b).split(","))) for b in buttons]
        joltage = re.sub(r"\{|\}", "", joltage).strip("\n")
        joltage = [int(j) for j in joltage.split(",")]

        out.append((diagram, buttons, joltage))

    return out


def solve_joltage(row):
    """
    solution vector x are positive integers that signify the number of button presses
    x is limited to positive whole numbers
    a linear equation is constructed for each position in the diagram out of the available buttons
    """

    diagram = row[0]
    buttons = row[1]
    joltage = row[2]

    # Create the solver using the CBC backend (open-source)
    solver = pywraplp.Solver.CreateSolver("CBC")
    if not solver:
        print("Solver not available")
        return 0

    x = []
    for i in range(0, len(buttons)):
        x.append(solver.IntVar(0, solver.infinity(), f"button_{i}"))

    constraints = [[] for _ in range(len(diagram))]

    # linear equation for each joltage position
    for i, b in enumerate(buttons):
        for pos in b:
            constraints[pos].append(x[i])

    for c, j in zip(constraints, joltage):
        solver.Add(sum(c) == j)

    solver.Minimize(sum(x))

    status = solver.Solve()

    if status == pywraplp.Solver.OPTIMAL:
        return solver.Objective().Value()
        # print(f"Button presses {solver.Objective().Value()}")
        # for v in x:
        #     print(f"{v.solution_value()}")

    raise ValueError("Solution does not exist.")


def main():
    input = read_input(Path("inputs") / "input")

    result = 0
    for input_line in input:
        result += solve_joltage(input_line)

    print(f"Part 2: {result}")


if __name__ == "__main__":
    main()
