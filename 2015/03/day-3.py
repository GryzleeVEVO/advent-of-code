x, y = 0, 0
x1, y1 = 0, 0
x2, y2 = 0, 0
homes_visited = {(x, y): 1}
homes_visited_2 = {(x1, y1): 2}
santa_turn = True

with open("input.txt") as f:
    for line in f:
        for c in line:
            match c:
                case ">":
                    x += 1
                    x1 += 1 if santa_turn else 0
                    x2 += 1 if not santa_turn else 0
                case "<":
                    x -= 1
                    x1 -= 1 if santa_turn else 0
                    x2 -= 1 if not santa_turn else 0
                case "^":
                    y += 1
                    y1 += 1 if santa_turn else 0
                    y2 += 1 if not santa_turn else 0
                case "v":
                    y -= 1
                    y1 -= 1 if santa_turn else 0
                    y2 -= 1 if not santa_turn else 0
                case _:
                    continue

            homes_visited[(x, y)] = (
                homes_visited[(x, y)] + 1 if (x, y) in dict.keys(homes_visited) else 1
            )

            if santa_turn:
                homes_visited_2[(x1, y1)] = (
                    homes_visited_2[(x1, y1)] + 1
                    if (x1, y1) in dict.keys(homes_visited_2)
                    else 1
                )
            else:
                homes_visited_2[(x2, y2)] = (
                    homes_visited_2[(x2, y2)] + 1
                    if (x2, y2) in dict.keys(homes_visited_2)
                    else 1
                )

            santa_turn = not santa_turn

print("Output for part 1 is", len(homes_visited), "homes visited")
print("Output for part 2 is", len(homes_visited_2), "homes visited")
