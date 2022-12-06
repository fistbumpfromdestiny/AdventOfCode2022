import sys

stream = []
counter = 0

with open("input.txt") as f:
    while True:
        char = f.read(1)
        if not char:
            break
        if len(stream) < 14:
            stream.append(char)
        else:
            if(len(set(stream)) != len(stream)):
                stream.pop(0)
                stream.append(char)
            else: 
                break
        counter += 1
   
print(f"{counter}")
    
