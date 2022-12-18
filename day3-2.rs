use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> std::io::Result<()> {
    let file = File::open("input3.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut res = 0;
    let mut set = HashSet::new();
    let mut set2 = HashSet::new();
    let mut status = 0;
    
    for line in lines {
        if let Ok(value) = line {
            if status == 0 {
                set = HashSet::from_iter(value.chars());
                status = 1;
            } else if status == 1 {
                for c in value.chars() {
                    if set.contains(&c) {
                        set2.insert(c);
                    }
                }
                status = 2;
            } else {
                for c in value.chars() {
                    if !set2.contains(&c) {
                        continue;
                    }
                    set2.remove(&c);
                    res += if c.is_lowercase() {
                        c as u32 - 96
                    } else {
                        c as u32 - 38
                    }
                }
                status = 0;
                set = HashSet::new();
                set2 = HashSet::new();
            }
        }
    }

    println!("The sum of the priorities is {res}");
    Ok(())
}