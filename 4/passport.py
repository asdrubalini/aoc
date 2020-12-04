passports: list = []

required_fields = ("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid")

with open("input.txt", "r") as f:
    passports.append("")

    for line in f.readlines():
        if line == "\n":
            passports.append("")

        passports[-1] += line.replace("\n", " ")


def is_valid_passport(passport: str) -> bool:
    fields: list = [field for field in passport.split(" ") if field != ""]

    valid_count: int = 0

    for field in fields:
        key: str = field.split(":")[0]
        value: str = field.split(":")[1]

        if key in required_fields:
            valid_count += 1
    
    return valid_count == len(required_fields)


valid_count: int = 0

for passport in passports:
    if is_valid_passport(passport):
        valid_count += 1

print(valid_count)
