use regex::Regex;
use std::fs;

// part 1
fn part1(content: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // This regex looks for:
    // - 'mul('
    // - Captures first number (1-3 digits)
    // - Comma
    // - Captures second number (1-3 digits)
    // - ')'
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let total_sum: i32 = re.captures_iter(content)
        .filter_map(|cap| {
            match (cap.get(1), cap.get(2)) {
                (Some(x), Some(y)) => {
                    let x_num: i32 = x.as_str().parse().ok()?;
                    let y_num: i32 = y.as_str().parse().ok()?;
                    Some(x_num * y_num)
                }
                _ => None
            }
        })
        .sum();

    Ok(total_sum)
}

// part 2
fn part2(content: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // Combined regex to capture mul, do, and don't instructions
    let instruction_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?;

    let mut mul_enabled = true;
    let mut total_sum = 0;

    for cap in instruction_re.captures_iter(content) {
        if let Some(mul_cap) = cap.get(0) {
            let instr = mul_cap.as_str();
            if instr.starts_with("do(") {
                mul_enabled = true;
            } else if instr.starts_with("don't") {
                mul_enabled = false;
            } else if instr.starts_with("mul(") && mul_enabled {
                // Extract the two numbers and add their product to the sum
                if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                    let x_num: i32 = x.as_str().parse()?;
                    let y_num: i32 = y.as_str().parse()?;
                    total_sum += x_num * y_num;
                }
            }
        }
    }

    Ok(total_sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input.txt")?;
    let part1_result = part1(&content)?;
    println!("Part 1 - {}", part1_result);
    let part2_result = part2(&content)?;
    println!("Part 2 - {}", part2_result);

    Ok(())
}
