use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("data/input-05.txt").expect("Couldn't open input");

    // Values are list of pages that can't go BEFORE key
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();

    for rule in content
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>()
    {
        let ids = rule.split("|").collect::<Vec<&str>>();
        let (left, right) = (ids[0], ids[1]);

        if rules.contains_key(left) {
            rules.get_mut(left).unwrap().push(right);
        } else {
            rules.insert(left, vec![right]);
        }
    }

    let updates: Vec<Vec<&str>> = content
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect();

    // Part 1
    let mut good = Vec::new();

    for batch in updates {
        let mut good_batch = true;

        for (i, page) in batch.iter().enumerate() {
            // If no rules for this page or first page, OK
            if i == 0 || !rules.contains_key(page) {
                continue;
            }

            // Otherwise, check if any of the previous values are out of order
            let check = batch.get(0..i).unwrap();
            let invalid = rules.get(page).unwrap();

            if check.iter().any(|id| invalid.contains(id)) {
                good_batch = false;
                break;
            }
        }

        if !good_batch {
            continue;
        }

        good.push(
            batch[batch.len() / 2]
                .parse::<i32>()
                .expect("Failed to parse middle page"),
        );
    }

    let res1: i32 = good.iter().sum();

    println!("Part one is {res1}");
}
