import sys

stacks = [
        ['N','R','J','T','Z','B','D','F'],
        ['H','J','N','S','R'],
        ['Q','F','Z','G','J','N','R','C'],
        ['Q','T','R','G','N','V','F'],
        ['F','Q','T','L'],
        ['N','G','R','B','Z','W','C','Q'],
        ['M','H','N','S','L','C','F'],
        ['J','T','M','Q','N','D'],
        ['S','G','P']
        ]

for line in sys.stdin:
    _, move_amount, _, from_stack, _, to_stack = line.rstrip().split(' ')
    move_amount, from_stack, to_stack = int(move_amount), int(from_stack), int(to_stack)
    for i in range(move_amount):
        stacks[to_stack-1].insert(0, stacks[from_stack-1].pop(0))

for i in range(len(stacks)):    
    print(stacks[i][0])