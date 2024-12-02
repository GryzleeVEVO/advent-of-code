/*
    Part 1: Get number of safe reports. A report is safe if:
        - The levels are arranged either in a increasing or decreasing order
        - Each level is higher/lower from the last by 1 or 2

    Part 2: Now one level can be wrong in each report
*/

use std::fs;

fn main() {
    // Extract data
    let content = fs::read_to_string("data/input-02.txt").expect("Couldn't open input file");

    let reports: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("Couldn't parse int"))
                .collect()
        })
        .collect();

    // Part 1
    let res_1 = reports
        .iter()
        .filter(|report| {
            report
                .windows(2)
                .all(|n| n[0] < n[1] && (n[0] - n[1]).abs() <= 3)
                || report
                    .windows(2)
                    .all(|n| n[0] > n[1] && (n[0] - n[1]).abs() <= 3)
        })
        .count() as i32;

    println!("Part one is {res_1}");
}
