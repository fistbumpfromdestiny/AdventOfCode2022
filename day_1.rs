use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cal_vec: Vec<u32> = Vec::new();
    let mut cals: u32 = 0;
    let file = File::open("input.txt")?;
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line?;
        if !line.is_empty() {
            cals += line.parse::<u32>().unwrap()
        } else if line.is_empty() {
            cal_vec.push(cals);
            cals = 0;
        }
    }
    cal_vec.sort();
    let mut top = 0;
    for _ in 0..3 {
        match cal_vec.pop() {
            Some(calories) => top += calories,
            None => top += 0,
        }
    }
    println!("{}", top);
    Ok(())
}
