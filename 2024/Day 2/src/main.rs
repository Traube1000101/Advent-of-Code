use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut safe_reports: [u16; 2] = [0, 0];

    for line in input.lines().filter(|s| !s.is_empty()) {
        let levels: Vec<i8> = line
            .split_whitespace()
            .map(|s| s.parse::<i8>().unwrap())
            .collect();

        if is_safe(&levels, false) {
            safe_reports[0] += 1;
        }
        if is_safe(&levels, true) {
            safe_reports[1] += 1;
        }
    }

    println!("Result Part One: {}", safe_reports[0]);

    println!("Result Part Two: {}", safe_reports[1]);

    Ok(())
}

fn is_safe(levels: &[i8], skip: bool) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut direction: Option<bool> = None;
    let mut skipped: bool = false;

    for pair in levels.windows(2) {
        let difference = pair[1] - pair[0];

        if difference == 0 || difference.abs() > 3 {
            if !skip || skipped {
                return false;
            } else {
                skipped = true;
                continue;
            }
        }

        let current_direction = difference > 0;
        if let Some(expected_direction) = direction {
            if current_direction != expected_direction {
                if !skip || skipped {
                    return false;
                } else {
                    skipped = true;
                    continue;
                }
            }
        } else {
            direction = Some(current_direction);
        }
    }

    true
}
