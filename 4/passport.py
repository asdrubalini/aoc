import string

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

        if key not in required_fields:
            continue
        
        if key == "byr":
            birth_year: int = int(value)

            if birth_year < 1920 or birth_year > 2002:
                continue
        
        elif key == "iyr":
            issue_year: int = int(value)

            if issue_year < 2010 or issue_year > 2020:
                continue
        
        elif key == "eyr":
            expiration_year: int = int(value)

            if expiration_year < 2020 or expiration_year > 2030:
                continue
        
        elif key == "hgt":
            if "cm" in value:
                height_cm = int(value.split("cm")[0])

                if height_cm < 150 or height_cm > 193:
                    continue
            
            elif "in" in value:
                height_in = int(value.split("in")[0])

                if height_in < 59 or height_in > 76:
                    continue
            
            else:
                continue

        elif key == "hcl":
            if value[0] != "#":
                continue

            hair_color = value.split("#")[1]
            hair_color_valid: bool = True
            
            for char in hair_color:
                if char not in string.hexdigits:
                    hair_color_valid = False

            if not hair_color_valid:
                continue
        
        elif key == "ecl":
            eye_color = value
            valid_colors = ("amb", "blu", "brn", "gry", "grn", "hzl", "oth")

            if eye_color not in valid_colors:
                continue
        
        elif key == "pid":
            pid = value
            pid_valid: bool = True

            if len(pid) != 9:
                pid_valid = False
            
            try:
                pid_int: int = int(pid)
            except:
                pid_valid = False
            
            if not pid_valid:
                continue
        
        valid_count += 1
    
    return valid_count == len(required_fields)


valid_count: int = 0

for passport in passports:
    if is_valid_passport(passport):
        valid_count += 1

print(valid_count)
