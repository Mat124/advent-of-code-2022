use std::fs;

pub fn sol() {
    let contents = fs::read_to_string("./src/day2/input_data").expect("Failed to open file");
    let lines: Vec<_> = contents.lines().collect();
    let mut score = 0;
    for line in lines {
        let vec = line.split(" ").collect::<Vec<&str>>();
        match vec[0] {
            "A" => {
                match vec[1] {
                    "X" => {
                        score += 3;
                    },
                    "Y" => {
                        score += 4;
                    },
                    _ => {
                        score += 8;
                    },
                }
            },
            "B" => {
                match vec[1] {
                    "X" => {
                        score += 1;
                    },
                    "Y" => {
                        score += 5;
                    },
                    _ => {
                        score += 9;
                    },
                }
            },
            _ => {
                match vec[1] {
                    "X" => {
                        score += 2;
                    },
                    "Y" => {
                        score += 6;
                    },
                    _ => {
                        score += 7;
                    },
                }
            }
        }
    }
    println!("{score}");
}