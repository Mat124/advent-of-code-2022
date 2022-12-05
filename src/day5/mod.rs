use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("./src/day5/input_data").expect("Failed to open file");
    let lines: Vec<&str> = contents.lines().collect();

    let (grid, moves) = lines.split_at(lines.iter().position(|x| x.is_empty()).expect("no empty lines!"));

    let mut grid_iter = grid.iter().rev();
    let max_stack = (*(grid_iter.next().expect("wasnt a first element"))).chars().rev().nth(1).expect("iterator didnt have a second element").to_digit(10).expect("wasnt a digit");
    
    let mut stacks = vec![Vec::<char>::new(); max_stack as usize];

    for line in grid_iter {
        let chars: Vec<char> = (*line).chars().collect();
        let mut cnt = 0;
        let mut char_iter = chars.iter();
        while let Some(char) = char_iter.next() {
            if (*char) == '[' {
                stacks[cnt as usize].push(*char_iter.next().unwrap()); //eats letter and pushes to stack
                char_iter.next(); //eats '] '
                char_iter.next();
                cnt += 1;
            }
            else {
                char_iter.next(); //eats 3 spaces
                char_iter.next();
                char_iter.next();
                cnt += 1;
            }
        }
    }


    for line in moves.iter().skip(1) {
        let words: Vec<&str> = line.split(" ").collect();
        let previous = words[3].parse::<i32>().expect("words[3] was not a number") - 1;
        let next = words[5].parse::<i32>().expect("words[5] was not a number") - 1;
        for i in 0..words[1].parse::<i32>().expect("words[1] was not a number") {
            let to_push = stacks[previous as usize].pop().expect("there wasnt anything in stacks[previous]");
            stacks[next as usize].push(to_push);
        }
    }

    for i in 0..max_stack {
        print!("{}", stacks[(i) as usize].pop().expect("empty stack when printing"));
    }
    println!();
}

pub fn part2() {
    let contents = fs::read_to_string("./src/day5/input_data").expect("Failed to open file");
    let lines: Vec<&str> = contents.lines().collect();

    let (grid, moves) = lines.split_at(lines.iter().position(|x| x.is_empty()).expect("no empty lines!"));

    let mut grid_iter = grid.iter().rev();
    let max_stack = (*(grid_iter.next().expect("wasnt a first element"))).chars().rev().nth(1).expect("iterator didnt have a second element").to_digit(10).expect("wasnt a digit");
    
    let mut stacks = vec![Vec::<char>::new(); max_stack as usize];

    for line in grid_iter {
        let chars: Vec<char> = (*line).chars().collect();
        let mut cnt = 0;
        let mut char_iter = chars.iter();
        while let Some(char) = char_iter.next() {
            if (*char) == '[' {
                stacks[cnt as usize].push(*char_iter.next().unwrap()); //eats letter and pushes to stack
                char_iter.next(); //eats '] '
                char_iter.next();
                cnt += 1;
            }
            else {
                char_iter.next(); //eats 3 spaces
                char_iter.next();
                char_iter.next();
                cnt += 1;
            }
        }
    }

    let mut spare_stack = Vec::<char>::new();

    for line in moves.iter().skip(1) {
        let words: Vec<&str> = line.split(" ").collect();
        let previous = words[3].parse::<i32>().expect("words[3] was not a number") - 1;
        let next = words[5].parse::<i32>().expect("words[5] was not a number") - 1;
        for i in 0..words[1].parse::<i32>().expect("words[1] was not a number") {
            let to_push = stacks[previous as usize].pop().expect("there wasnt anything in stacks[previous]");
            spare_stack.push(to_push);
        }
        for i in 0..words[1].parse::<i32>().expect("words[1] was not a number") {
            stacks[next as usize].push(spare_stack.pop().unwrap());
        }
    }

    for i in 0..max_stack {
        print!("{}", stacks[(i) as usize].pop().expect("empty stack when printing"));
    }
    println!();
}