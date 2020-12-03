from functools import reduce

path: list = []

with open("input.txt", "r") as f:
    for line in f.readlines():
        line = line.replace("\n", "")
        path.append(line)


def is_next_move_tree(top: int, left: int) -> bool:
    if top >= len(path):
        raise ValueError

    current_line = path[top]
    current_object = current_line[left % 31]

    print(f"Reading top={top} left={left} = {current_object}")

    return current_object == "#"


configurations = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
]

encountered_trees: list = []

for configuration in configurations:
    current_left: int = 0
    current_top: int = 0

    trees_count: int = 0

    while True:
        try:
            is_tree: bool = is_next_move_tree(top=current_top, left=current_left)
        except ValueError:
            break

        if is_tree:
            trees_count += 1
        
        current_left += configuration[0]
        current_top += configuration[1]
    
    encountered_trees.append(trees_count)


prod_trees = reduce(lambda x, y: x * y, encountered_trees)
print(prod_trees)
