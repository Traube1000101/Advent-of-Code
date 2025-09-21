use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut total: u32 = 0;

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    for captures in regex.captures_iter(&input) {
        let a: u32 = captures[1].parse()?;
        let b: u32 = captures[2].parse()?;
        total += a * b;
    }

    println!("Result Part One: {}", total);

    Ok(())
}
