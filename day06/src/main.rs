use std::io::{self};

fn is_unique_sequence(s: &[char]) -> bool {
    for c in s {
        if s.iter().filter(|f| *f == c).count() > 1 {
            return false;
        }
    }
    
    true
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap().chars().collect::<Vec<_>>();
    let mut counter = 4;
    for win in input.windows(4) {
        if is_unique_sequence(win) {
            break;
        }
        counter += 1;
    }

    dbg!(counter);

    let mut counter2 = 14;
    for win in input.windows(14) {
        if is_unique_sequence(win) {
            break;
        }
        counter2 += 1;
    }

    dbg!(counter2);
}
