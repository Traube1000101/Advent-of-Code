use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    for line in input.lines().filter(|s| !s.is_empty()) {}

    println!("Result Part One: {}", "");

    //println!("Result Part Two: {}", "");

    Ok(())
}
