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
