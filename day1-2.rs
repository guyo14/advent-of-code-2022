use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut acc = 0;
    let mut max = [0; 3];
    
    for line in lines {
        if let Ok(value) = line {
            if value.is_empty() {
                for i in 0..3 {
                    if acc > max[i] {
                        for j in (i..2).rev() {
                            max[j + 1] = max[j];
                        }
                        max[i] = acc;
                        break;
                    }
                }
                acc = 0;
            } else {
                let num: i32 = value.parse().expect(&format!("Error converting '{value}'"));
                acc += num;
            }
        }
    };
    for i in 0..3 {
        if acc > max[i] {
            for j in (i..2).rev() {
                max[j + 1] = max[j];
            }
            max[i] = acc;
            break;
        }
    }
    let mut sum = 0;
    for i in max {
        sum += i;
    }
    println!("The three elves with more calories carry a total of {sum} calories");
    Ok(())
}