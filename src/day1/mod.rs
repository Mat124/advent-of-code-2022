use std::fs;

pub fn sol() {
    let contents = fs::read_to_string("./src/day1/input_data").expect("Failed to open file");
    let lines: Vec<_> = contents.lines().collect();
    let mut max = -1;
    let mut max2 = -1;
    let mut max3 = -1;
    let mut curr = 0;
    for line in lines {
        let calories = line.parse::<i32>();
        match calories {
            Ok(x) => curr += x,
            _ => {
                if curr > max {
                    max3 = max2;
                    max2 = max;
                    max = curr;
                }
                else if curr > max2 {
                    max3 = max2;
                    max2 = curr;
                }
                else if curr > max3 {
                    max3 = curr;
                }
                curr = 0;
            }
        }
    }
    let total = max + max2 + max3;
    println!("{total}");
}
