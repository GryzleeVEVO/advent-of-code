-- Part 1:
--  - Goal: # chars code - # chars in-memory
--  - All strings enclosed in qootes "" -> min 2 chars code, 0 in memory
--  - \? escaped characters count as one in memory
--  - \x?? escaped characters count as one in memory too
--  - Chars code is length string
--  - Chars in memory
-- Part 2:
--  - Goal: # chars reencoded - # chars of code
--  - Reencoded is just the same string quoted again + inside quotes and backslashes escaped

local file = io.open("input.txt")

if file == nil then
    print("Couldn´t open input")
    return 1
end

local part1 = 0
local part2 = 0

for line in file:lines() do
    local unquoted = line:match("^\"(.*)\"$");
    local representation = #line
    local in_memory = #unquoted
    local reencoded = representation + 2 -- account for extra unescaped quotes

    for escaped in unquoted:gmatch("\\[^x]") do
        in_memory = in_memory - 1
    end

    for hex in unquoted:gmatch("\\x[a-fA-F0-9][a-fA-F0-9]") do
        in_memory = in_memory - 3
    end

    for reenc in line:gmatch("\"") do
        reencoded = reencoded + 1
    end

    for reenc in line:gmatch("\\") do
        reencoded = reencoded + 1
    end


    part1 = part1 + representation - in_memory
    part2 = part2 + reencoded - representation
end

-- Should be 1333
print("Part 1 is: " .. part1)

-- Should be 2046
print("Part 2 is: " .. part2)

file:close()
