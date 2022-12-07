use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("./src/day6/input_data").expect("Failed to open file");
    let mut lines: Vec<&str> = contents.lines().collect();
    let line: Vec<char> = lines.pop().expect("no items in lines").chars().collect();
    let mut back = 0;
    let mut front = 4;
    let mut letters = vec![0; 26];
    loop {
        letters = vec![0; 26];
        for i in back..front {
            letters[(line[i as usize] as u32 - 97) as usize] += 1;
        }
        if *letters.iter().max().expect("letters had no content") == 1 {
            println!("{}", front);
            break;
        }
        back += 1;
        front += 1;
    }
}

pub fn part2() {
    let contents = fs::read_to_string("./src/day6/input_data").expect("Failed to open file");
    let mut lines: Vec<&str> = contents.lines().collect();
    let line: Vec<char> = lines.pop().expect("no items in lines").chars().collect();
    let mut back = 0;
    let mut front = 14;
    let mut letters = vec![0; 26];
    loop {
        letters = vec![0; 26];
        for i in back..front {
            letters[(line[i as usize] as u32 - 97) as usize] += 1;
        }
        if *letters.iter().max().expect("letters had no content") == 1 {
            println!("{}", front);
            break;
        }
        back += 1;
        front += 1;
    }
}