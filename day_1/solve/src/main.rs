use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap; 



//part1 
fn calculate_total_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    // absolute value and also iterator cool asf
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

//part 2 

fn calculate_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    
    let mut right_counts = HashMap::new(); 

    // count occurences in right list
    for &num in right {

        *right_counts.entry(num).or_insert(0) +=1; 
    }

    // calculating score 
    let mut total = 0; 

    for &left_num in left{
        // get the count of current number in right list, default is 0 (if no count) 
        let count = *right_counts.get(&left_num).unwrap_or(&0);
        
        total += left_num * count;
    }

    total

}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parsing that i plagiarised
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left_list.push(left);
                right_list.push(right);
            }
        }
    }

    let total_distance = calculate_total_distance(&mut left_list, &mut right_list);
    println!("Total distance: {}", total_distance);
    let similarity_score = calculate_similarity_score(&mut left_list, &mut right_list); 
    println!("Similarity Score: {}", similarity_score);
    Ok(())
}
