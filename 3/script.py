path: list = []

with open("input.txt", "r") as f:
    for line in f.readlines():
        line = line.replace("\n", "")
        path.append(line)


def is_next_move_tree(top: int, left: int) -> bool:
    current_line = path[top]
    current_object = current_line[left % 31]

    print(f"Reading top={top} left={left} = {current_object}")

    return current_object == "#"


current_left: int = 0
trees_count: int = 0


for current_top in range(len(path)):
    is_tree: bool = is_next_move_tree(top=current_top, left=current_left)

    if is_tree:
        trees_count += 1
    
    current_left += 3

print("There was", trees_count, "trees")
