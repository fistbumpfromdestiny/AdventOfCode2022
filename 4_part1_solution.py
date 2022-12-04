import sys

count = 0

def subset(a, b):
    return a[0] >= b[0] and a[0] <= b[1] and a[1] >= b[0] and a[1] <= b[1]

def overlap(a, b):
    return subset(a, b) or subset(b, a)

for line in sys.stdin:
    a, b = line.rstrip().split(',')
    assignment_a, assignment_b = a.split('-'), b.split('-')
    if overlap([int(assignment_a[0]), int(assignment_a[1])], [int(assignment_b[0]), int(assignment_b[1])]):
        count += 1

print(f"{count}")