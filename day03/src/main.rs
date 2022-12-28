use std::collections::HashSet;
use std::io::{self, BufRead};

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c.to_digit(36).unwrap() - 9
    } else {
        c.to_digit(36).unwrap() - 9 + 26
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .map(|mut v| {
            let v2 = v.split_off(v.len() / 2);
            (
                HashSet::from_iter(v.into_iter()),
                HashSet::from_iter(v2.into_iter()),
            )
        })
        .collect::<Vec<(HashSet<char>, HashSet<char>)>>();
    {
        let mut score = 0;

        for line in &lines {
            for i in line.0.intersection(&line.1) {
                score += priority(*i);
            }
        }

        println!("task1: {}", score);
    }

    {
        use std::ops::BitAnd;


        let mut badge_type_sum = 0;

        let lines = lines
            .into_iter()
            .map(|(l1, l2)| {
                let l3 = l1.union(&l2).cloned().collect::<HashSet<char>>();
                (l1, l2, l3)
            })
            .collect::<Vec<_>>();
        for group in lines.chunks(3) {
            let mut intersection = group.iter().fold(group.iter().next().unwrap().2.clone(), |set1, (_, _, set2)| {
                set1.bitand(set2)
            });

            if intersection.len() > 1 {
                panic!();
            }

            let badge_type = intersection.iter().next().unwrap();


            badge_type_sum += priority(*badge_type);
        }

        println!("task2: {}", badge_type_sum);
    }
}
