use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut calories_by_elf: Vec<u32> = Vec::new();
    calories_by_elf.push(0);
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            calories_by_elf.push(0);
        } else {
            *calories_by_elf.last_mut().unwrap() += line.parse::<u32>().unwrap();
        }
    }
    println!("Most calories: {}", calories_by_elf.iter().max().unwrap());

    calories_by_elf.sort_unstable();
    
    println!("Top 3 most calories: {}", calories_by_elf.iter().rev().take(3).sum::<u32>());
}
