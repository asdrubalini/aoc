passwords: list = []

with open("input.txt", "r") as f:
    for line in f.readlines():
        line = line.replace("\n", "")

        rule: str = line.split(":")[0]
        password: str = line.split(":")[1].replace(" ", "")

        passwords.append([rule, password])


def is_valid_password(rule: str, password: str) -> bool:
    print("Parsing", password, "with rule:", rule)

    positions_str: str = rule.split(" ")[0]
    wanted_char: str = rule.split(" ")[1]

    positions: tuple = tuple([int(position) - 1 for position in positions_str.split("-")])

    valid_positions_count: int = 0

    for position in positions:
        if len(password) < position:
            continue

        if password[position] == wanted_char:
            valid_positions_count += 1
    
    if valid_positions_count == 1:
        return True

    return False

valids: int = 0

for password in passwords:
    if is_valid_password(password[0], password[1]):
        valids += 1

print(valids)
