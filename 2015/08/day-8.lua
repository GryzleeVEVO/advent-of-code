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

print("Part 1 is: " .. part1)
print("Part 2 is: " .. part2)

file:close()
