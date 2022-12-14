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
                'X' => 0 + match play[0] {
                    'A' => 3,
                    'B' => 1,
                    _ => 2,
                },
                'Y' => 3 + match play[0] {
                    'A' => 1,
                    'B' => 2,
                    _ => 3,
                },
                _ => 6 + match play[0] {
                    'A' => 2,
                    'B' => 3,
                    _ => 1,
                },
            };
            score += points;
        }
    }

    println!("The score is {score}");
    Ok(())
}