use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data/input-03.txt").expect("Couldn't open input");

    // Part 1
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Couldn't create mul regex");
    let res1 = mul_regex
        .captures_iter(&contents)
        .map(|c| c.extract())
        .map(|(_, [op1, op2])| {
            (
                op1.parse::<i32>().expect("Couldn't parse op1"),
                op2.parse::<i32>().expect("Couldn't parse op2"),
            )
        })
        .fold(0, |acc, (op1, op2)| acc + op1 * op2);

    println!("Part one is {res1}");

    // Part 2
    let do_regex = Regex::new(r"do\(\)").expect("Couldn't create do regex");
    let dont_regex = Regex::new(r"don't\(\)").expect("Couldn't create don't regex");

    // I find it easier to first get all valid instructions since there is no easy way to check for
    // multiple inputs at once
    let p2_valid: Vec<_> = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")
        .expect("Couldn't create cleanup regex")
        .find_iter(&contents)
        .map(|x| x.as_str())
        .collect();

    let mut enabled = true;
    let mut res2 = 0;

    for inst in p2_valid {
        if mul_regex.find(inst).is_some() {
            if !enabled {
                continue;
            }

            let (op1, op2) = mul_regex
                .captures(inst)
                .map(|c| c.extract())
                .map(|(_, [op1, op2])| {
                    (
                        op1.parse::<i32>().expect("Couldn't parse op1"),
                        op2.parse::<i32>().expect("Couldn't parse op2"),
                    )
                })
                .unwrap();

            res2 += op1 * op2;
        } else if do_regex.find(inst).is_some() {
            enabled = true;
        } else if dont_regex.find(inst).is_some() {
            enabled = false;
        } else {
            panic!("Something went wrong, inst doesn't match any regex");
        }
    }

    println!("Part two is {res2}");
}
