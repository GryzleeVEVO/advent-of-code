/*
    Part 1: Find out the total distance between two lists
        - Two columns holding unique ID's
        - Must pair up the smallest ID from the left column with the ID from the
            right list, then the second smallest, so on and so forth...
        - The distance is the difference between the two IDs
        - So, it's just ordering both lists and subtracting them

    Part 2: Calculate the similarity score
        - Multiply each left-side element by the number of times
*/

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

    // Should be 936063
    println!("Part one is {sum1}");

    // Part 2: Multiply each element from the left with the number of times
    // it appears on the right
    let sum2: i32 = left
        .iter()
        .map(|i| i * (right.iter().filter(|&j| i == j).count() as i32))
        .sum();

    // Should be 23150395
    println!("Part two is {sum2}");
}