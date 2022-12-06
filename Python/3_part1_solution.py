import sys

shared_item = []

for line in sys.stdin:
    sack_a, sack_b = line[:len(line)//2], line[len(line)//2:]
    shared_item.append(''.join(set(sack_a).intersection(sack_b)))

priority = [ord(x) - 38 if x.isupper() else ord(x) - 96 for x in shared_item]    
print(f"{sum(priority)}")


