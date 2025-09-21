use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines().map(str::trim).filter(|s| !s.is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let [left_part, right_part]: [&str; 2] = parts.as_slice().try_into()?;
        let a: i32 = left_part.parse()?;
        let b: i32 = right_part.parse()?;
        left.push(a);
        right.push(b);
    }

    left.sort_unstable();
    right.sort_unstable();

    let result: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum();

    println!("Result Part One: {}", result);

    let mut total_similarity: Vec<i32> = Vec::new();

    left.iter().for_each(|l| {
        let count = right.iter().filter(|r| *r == l).count();
        let similarity = l * count as i32;
        total_similarity.push(similarity as i32);
    });

    println!("Result Part Two: {}", total_similarity.into_iter().sum::<i32>());

    Ok(())
}
