cals = sorted([sum(map(int, elf.split("\n"))) for elf in open("input.txt").read().strip().split("\n\n")])
print(cals[-1])
print(sum(cals[-3:]))