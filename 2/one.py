passwords: list = []

with open("input.txt", "r") as f:
    for line in f.readlines():
        line = line.replace("\n", "")

        rule: str = line.split(":")[0]
        password: str = line.split(":")[1].replace(" ", "")

        passwords.append([rule, password])


def is_valid_password(rule: str, password: str) -> bool:
    print("Parsing", password, "with rule:", rule, end="\t")

    count: str = rule.split(" ")[0]
    valid: str = rule.split(" ")[1]

    min_count: int = int(count.split("-")[0])
    max_count: int = int(count.split("-")[1])

    actual_count: int = len([char for char in password if char == valid])
    
    print("valid:", valid, "min_count:", min_count, "max_count:", max_count, "actual_count", actual_count)
    
    return actual_count >= min_count and actual_count <= max_count


valids: int = 0

for password in passwords:
    if is_valid_password(password[0], password[1]):
        valids += 1

print(valids)
