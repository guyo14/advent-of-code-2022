use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("input5.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut status = 0;
    let mut stacks : Vec<Vec<char>> = Vec::new();
    
    for line in lines {
        if let Ok(value) = line {
            if status == 0 {
                let total_stacks = (value.len() + 1) / 4;
                for _ in 0..total_stacks {
                    stacks.push(Vec::new());
                }
                status = 1;
            }
            if status == 1 {
                if value.is_empty() {
                    status = 2;
                    continue;
                }
                let stacks_number = stacks.len();
                let mut chars = value.chars();
                for i in 0..stacks_number {
                    if let Some(c) = chars.nth(1) {
                        if c.is_ascii_uppercase() {
                            stacks[i].insert(0, c);
                        }
                    }
                    chars.nth(1);
                }
            } else {
                let mut tokens = value.split(" ");
                let n = tokens.nth(1).unwrap().parse::<usize>().unwrap();
                let from = tokens.nth(1).unwrap().parse::<usize>().unwrap();
                let to = tokens.nth(1).unwrap().parse::<usize>().unwrap();
                for _ in 0..n {
                    if let Some(v) = stacks[from - 1].pop() {
                        stacks[to - 1].push(v);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let res = stacks.iter().filter_map(|s| s.last()).collect::<String>();

    println!("The crates at top are {res}");
    Ok(())
}
