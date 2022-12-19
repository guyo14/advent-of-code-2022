use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("input4.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut res = 0;
    
    for line in lines {
        if let Ok(value) = line {
            let mut ranges = value.split(",");
            let range1 = ranges.next().unwrap().split("-").map(|r| r.parse().unwrap()).collect::<Vec<u32>>();
            let range2 = ranges.next().unwrap().split("-").map(|r| r.parse().unwrap()).collect::<Vec<u32>>();

            if range1[0] <= range2[0] && range1[1] >= range2[1] || range1[0] >= range2[0] && range1[1] <= range2[1] {
                res += 1;
            }
        }
    }

    println!("The sum of the priorities is {res}");
    Ok(())
}
