use std::fs;

fn main() {
    // Extract contents
    let content = fs::read_to_string("data/input-01.txt").expect("No input file found!");

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = content
        .lines()
        .map(|line| {
            let split: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().expect("Couldn't parse int"))
                .collect();

            (split[0], split[1])
        })
        .unzip();

    left.sort();
    right.sort();

    // Part 1: Calculate the difference between locations
    let sum1: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Part one is {sum1}");

    // Part 2: Multiply each element from the left with the number of times
    // it appears on the right
    let sum2: i32 = left
        .iter()
        .map(|i| i * (right.iter().filter(|&j| i == j).count() as i32))
        .sum();

    println!("Part two is {sum2}");
}
