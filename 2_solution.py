import sys

score_game_one = 0
score_game_two = 0

def my_choice(me):
    match (me):
        case 'X':
            return 1
        case 'Y':
            return 2
        case 'Z':
            return 3


def game_one(elf, me):
    match(elf, me):
        case ('A', 'X'): return 3
        case ('B', 'X'): return 0
        case ('C', 'X'): return 6
        case ('A', 'Y'): return 6
        case ('B', 'Y'): return 3
        case ('C', 'Y'): return 0
        case ('A', 'Z'): return 0
        case ('B', 'Z'): return 6
        case ('C', 'Z'): return 3

def game_two(elf, me):
    match(elf, me):
        case('A', 'X'): return (my_choice('Z') + 0)
        case('A', 'Y'): return (my_choice('X') + 3)
        case('A', 'Z'): return (my_choice('Y') + 6)
        case('B', 'X'): return (my_choice('X') + 0)
        case('B', 'Y'): return (my_choice('Y') + 3)
        case('B', 'Z'): return (my_choice('Z') + 6)
        case('C', 'X'): return (my_choice('Y') + 0)
        case('C', 'Y'): return (my_choice('Z') + 3)
        case('C', 'Z'): return (my_choice('X') + 6)

for line in sys.stdin:
    
    elf, me = line.rstrip().split(' ')
    score_game_one += my_choice(me)
    score_game_one += game_one(elf, me)
    score_game_two += game_two(elf, me)
    

print(f"Total score game one: {score_game_one}")
print(f"Total score game two: {score_game_two}")
