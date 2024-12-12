use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct PageOrder {
    rules: HashMap<u32, Vec<u32>>,
}

impl PageOrder {
    fn new() -> Self {
        PageOrder {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, before: u32, after: u32) {
        self.rules.entry(before).or_insert_with(Vec::new).push(after);
    }

    fn is_valid_order(&self, update: &[u32]) -> bool {
        let mut positions = HashMap::new();
        for (index, &page) in update.iter().enumerate() {
            positions.insert(page, index);
        }

        for (&before, afters) in &self.rules {
            if let Some(&before_pos) = positions.get(&before) {
                for &after in afters {
                    if let Some(&after_pos) = positions.get(&after) {
                        if before_pos > after_pos {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn reorder_update(&self, update: &mut Vec<u32>) {
        update.sort_by(|&a, &b| {
            if self.rules.get(&a).map_or(false, |afters| afters.contains(&b)) {
                std::cmp::Ordering::Less
            } else if self.rules.get(&b).map_or(false, |afters| afters.contains(&a)) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file");
    let lines = io::BufReader::new(file).lines();

    let mut section = 0;
    let mut page_order = PageOrder::new();
    let mut updates = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                section += 1;
                continue;
            }

            match section {
                0 => {
                    let parts: Vec<&str> = line.split('|').collect();
                    let before = parts[0].parse::<u32>().unwrap();
                    let after = parts[1].parse::<u32>().unwrap();
                    page_order.add_rule(before, after);
                }
                1 => {
                    let update: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                    updates.push(update);
                }
                _ => unreachable!(),
            }
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;

    for mut update in updates {
        if page_order.is_valid_order(&update) {
            let middle = update[update.len() / 2];
            part_1 += middle;
        } else {
            page_order.reorder_update(&mut update);
            let middle = update[update.len() / 2];
            part_2 += middle;
        }
    }

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

