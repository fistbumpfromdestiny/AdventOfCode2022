import sys

calories = 0
temp = 0

for line in sys.stdin:
    
    if line.strip() != "":
        temp += int(line)
    else:
        if temp > calories:
            calories = temp
        temp = 0
print(f"The highest amount of calories carried by an elf is {calories}.")
