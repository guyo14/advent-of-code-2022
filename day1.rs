use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut acc = 0;
    let mut max = 0;
    
    for line in lines {
        if let Ok(value) = line {
            if value.is_empty() {
                if acc > max {
                    max = acc;
                }
                acc = 0;
            } else {
                let num: i32 = value.parse().expect(&format!("Error converting '{value}'"));
                acc += num;
            }
        }
    };
    if acc > max {
        max = acc;
    }
    println!("The elf with more calories carries {max} calories");
    Ok(())
}