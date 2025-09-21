use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut total_p1: u32 = 0;

    // Part One
    let regex_p1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    for captures in regex_p1.captures_iter(&input) {
        let a: u32 = captures[1].parse()?;
        let b: u32 = captures[2].parse()?;
        total_p1 += a * b;
    }

    println!("Result Part One: {}", total_p1);

    // Part Two
    let regex_p2 = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|do\(\)|don't\(\)")?;
    let mut enabled = true;
    let mut total_p2: u32 = 0;

    for caps in regex_p2.captures_iter(&input) {
        if let Some(whole_match) = caps.get(0) {
            let token = whole_match.as_str();
            if token.starts_with("mul(") {
                if enabled {
                    let a: u32 = caps[1].parse()?;
                    let b: u32 = caps[2].parse()?;
                    total_p2 += a * b;
                }
            } else if token == "do()" {
                enabled = true;
            } else {
                enabled = false;
            }
        }
    }

    println!("Result Part Two: {}", total_p2);

    Ok(())
}
