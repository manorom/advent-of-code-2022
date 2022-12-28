use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut split = l
                .split(|c| c == ',' || c == '-')
                .map(|s| s.parse::<u32>().unwrap());
            [
                split.next().unwrap()..=split.next().unwrap(),
                split.next().unwrap()..=split.next().unwrap(),
            ]
        })
        .collect::<Vec<[std::ops::RangeInclusive<u32>; 2]>>();

    let fully_overlapping: usize = lines
        .iter()
        .filter(|[r1, r2]| {
            if (r1.contains(r2.start()) && r1.contains(r2.end()))
                || (r2.contains(r1.start()) && r2.contains(r1.end()))
            {
                true
            } else {
                false
            }
        })
        .count();

    dbg!(fully_overlapping);

    let partially_overlapping: usize = lines
        .iter()
        .filter(|[r1, r2]| {
            if r1.contains(r2.start()) || r1.contains(r2.end()) {
                true
            } else if r2.contains(r1.start()) || r2.contains(r1.end()) {
                true
            } else {
                false
            }
        })
        .count();

    dbg!(partially_overlapping);
}
