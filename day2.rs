use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut score = 0;
    
    for line in lines {
        if let Ok(value) = line {
            let play : Vec<char> = value.split(' ').map(|s| s.chars().next().unwrap()).collect();
            let points = match play[1] {
                'X' => 1 + match play[0] {
                    'B' => 0,
                    'C' => 6,
                    _ => 3,
                },
                'Y' => 2 + match play[0] {
                    'C' => 0,
                    'A' => 6,
                    _ => 3,
                },
                _ => 3 + match play[0] {
                    'A' => 0,
                    'B' => 6,
                    _ => 3,
                },
            };
            score += points;
        }
    }

    println!("The score is {score}");
    Ok(())
}