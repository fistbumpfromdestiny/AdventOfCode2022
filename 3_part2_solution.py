import sys

shared_item = []

with open("input.txt") as f:
    for elf1, elf2, elf3 in zip(f, f, f):
        shared_item.append(''.join(set(elf1).intersection(elf2).intersection(elf3)).strip())    

priority = [ord(x) - 38 if x.isupper() else ord(x) - 96 for x in shared_item]          
print(f"{sum(priority)}")
