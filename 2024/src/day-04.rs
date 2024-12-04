use std::fs;

fn main() {
    let content = fs::read_to_string("data/input-04.txt").expect("Couldn't open input");
    let word_search: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    // Part 1
    let mut res1 = 0;

    for (i, outer) in word_search.iter().enumerate() {
        for (j, _) in outer.iter().enumerate() {
            if j + 3 < outer.len() {
                let hor_candidate = String::from_iter(&word_search[i][j..=j + 3]);

                if hor_candidate == "XMAS" || hor_candidate == "SAMX" {
                    res1 += 1;
                }
            }

            if i + 3 < word_search.len() {
                let vert_candidate = String::from_iter([
                    word_search[i][j],
                    word_search[i + 1][j],
                    word_search[i + 2][j],
                    word_search[i + 3][j],
                ]);

                if vert_candidate == "XMAS" || vert_candidate == "SAMX" {
                    res1 += 1;
                }

                if j + 3 < outer.len() {
                    let ltr_candidate = String::from_iter([
                        word_search[i][j],
                        word_search[i + 1][j + 1],
                        word_search[i + 2][j + 2],
                        word_search[i + 3][j + 3],
                    ]);

                    if ltr_candidate == "XMAS" || ltr_candidate == "SAMX" {
                        res1 += 1
                    }
                }

                if j >= 3 {
                    let rtl_candidate = String::from_iter([
                        word_search[i][j],
                        word_search[i + 1][j - 1],
                        word_search[i + 2][j - 2],
                        word_search[i + 3][j - 3],
                    ]);

                    if rtl_candidate == "XMAS" || rtl_candidate == "SAMX" {
                        res1 += 1;
                    }
                }
            }
        }
    }

    println!("Part one is {res1}");

    // Part 2
    let mut res2 = 0;

    for (i, outer) in word_search.iter().enumerate() {
        for (j, _) in outer.iter().enumerate() {
            if i + 2 < word_search.len() && j + 2 < outer.len() && word_search[i + 1][j + 1] == 'A'
            {
                let ltr = String::from_iter([
                    word_search[i][j],
                    word_search[i + 1][j + 1],
                    word_search[i + 2][j + 2],
                ]);

                let rtl = String::from_iter([
                    word_search[i + 2][j],
                    word_search[i + 1][j + 1],
                    word_search[i][j + 2],
                ]);

                if (ltr == "MAS" || ltr == "SAM") && (rtl == "MAS" || rtl == "SAM") {
                    res2 += 1;
                }
            }
        }
    }

    println!("Part two is {res2}");
}
