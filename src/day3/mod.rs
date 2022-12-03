use std::fs;

pub fn sol() {
    let contents = fs::read_to_string("./src/day3/input_data").expect("Failed to open file");
    let lines: Vec<_> = contents.lines().collect();
    let mut sum = 0;
    for triplet in lines.chunks(3) {
        for char in triplet[0].chars() {
            if triplet[1].contains(char) && triplet[2].contains(char) {
                if char as u32 > 96 {
                    sum += char as u32 - 96
                }
                else {
                    sum += char as u32 - 38;
                }
                break;
            }
        }
    }
    println!("{sum}");
}