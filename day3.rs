use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() -> std::io::Result<()> {
    let file = File::open("input3.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut res = 0;
    
    for line in lines {
        if let Ok(value) = line {
            let len = value.len();
            let mid = len / 2;
            let mut set : HashSet<char> = HashSet::from_iter(value[0..mid].chars());

            for c in value[mid..len].chars() {
                if !set.contains(&c) {
                    continue;
                }
                set.remove(&c);
                res += if c.is_lowercase() {
                    c as u32 - 96
                } else {
                    c as u32 - 38
                }
            }
        }
    }

    println!("The sum of the priorities is {res}");
    Ok(())
}