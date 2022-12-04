use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("./src/day4/input_data").expect("Failed to open file");
    let lines: Vec<_> = contents.lines().collect();
    let mut out = 0;
    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first: Vec<&str> = pairs[0].split("-").collect();
        let second: Vec<&str> = pairs[1].split("-").collect();
        if first[0].parse::<i32>().expect("bad parsing") == second[0].parse::<i32>().expect("bad parsing") {
            out += 1;
        }
        else if first[0].parse::<i32>().expect("bad parsing") < second[0].parse::<i32>().expect("bad parsing") {
            if first[1].parse::<i32>().expect("bad parsing") >= second[1].parse::<i32>().expect("bad parsing") {
                out += 1;
            }
        }
        else if first[1].parse::<i32>().expect("bad parsing") <= second[1].parse::<i32>().expect("bad parsing") {
            out += 1;
        }
    }
    println!("{out}");
}

pub fn part2() {
    let contents = fs::read_to_string("./src/day4/input_data").expect("Failed to open file");
    let lines: Vec<_> = contents.lines().collect();
    let mut out = 0;
    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first: Vec<&str> = pairs[0].split("-").collect();
        let second: Vec<&str> = pairs[1].split("-").collect();
        if first[0].parse::<i32>().expect("bad parsing") <= second[0].parse::<i32>().expect("bad parsing") && first[1].parse::<i32>().expect("bad parsing") >= second[0].parse::<i32>().expect("bad parsing") {
            out += 1;
        }
        else if second[0].parse::<i32>().expect("bad parsing") <= first[0].parse::<i32>().expect("bad parsing") && second[1].parse::<i32>().expect("bad parsing") >= first[0].parse::<i32>().expect("bad parsing") {
            out += 1;
        }
    }
    println!("{out}");
}