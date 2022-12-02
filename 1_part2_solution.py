top1 = 0 
top2 = 0
top3 = 0
temp = 0

with open("input.txt") as f:
    input = f.read().splitlines()
    
    for line in input:
        if line:
            temp += int(line)
        else:
            if temp >= top1:
                top1, top2, top3 = temp, top1, top2
            elif temp >= top2:
                top2, top3 = temp, top2
            elif temp >= top3:
                top3 = temp
            print(temp)
            temp = 0

print(top1)
print(top2)
print(top3)
print(f"The amount of calories carried by the top 3 is {top1+top2+top3}.")



