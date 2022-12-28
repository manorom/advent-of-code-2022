use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn has_neighbors(pos: (i32, i32), map: &HashSet<(i32, i32)>) -> bool {
    for p in [(-1, 1), (-1, 0), (-1, -1), (0, 1), (0, -1), (1, -1), (1, 0), (1, 1)] {
        if map.contains(&(pos.0 + p.0, pos.1 + p.1)) {
            return true;
        }
    }
    false
}

fn can_move_direction(dir: i32, pos: (i32, i32), map: &HashSet<(i32, i32)>) -> bool {
    if dir == 0 {
        return !map.contains(&(pos.0, pos.1 - 1))
            && !map.contains(&(pos.0 + 1, pos.1 - 1))
            && !map.contains(&(pos.0 - 1, pos.1 - 1));
    }
    if dir == 1 {
        return !map.contains(&(pos.0, pos.1 + 1))
            && !map.contains(&(pos.0 + 1, pos.1 + 1))
            && !map.contains(&(pos.0 - 1, pos.1 + 1));
    }
    if dir == 2 {
        return !map.contains(&(pos.0 - 1, pos.1))
            && !map.contains(&(pos.0 - 1, pos.1 - 1))
            && !map.contains(&(pos.0 - 1, pos.1 + 1));
    }
    if dir == 3 {
        return !map.contains(&(pos.0 + 1, pos.1))
            && !map.contains(&(pos.0 + 1, pos.1 - 1))
            && !map.contains(&(pos.0 + 1, pos.1 + 1));
    }
    panic!()
}

const directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn find_move(from: (i32, i32), dir: usize, map: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    if !has_neighbors(from, map) {
        return None;
    }
    for d in (0..4).cycle().skip(dir).take(4) {
        if can_move_direction(d, from, map) {
            return Some((from.0 + directions[d as usize].0, from.1 + directions[d as usize].1));
        }
    }
    None
}

fn print(map: &HashSet<(i32, i32)>) {
    let left = map.iter().map(|(x, _y)| *x).min().unwrap().min(0);
    let right = map.iter().map(|(x, _y)| *x).max().unwrap().max(4);
    let top = map.iter().map(|(_x, y)| *y).min().unwrap().min(0);
    let bottom = map.iter().map(|(_x, y)| *y).max().unwrap().max(5);
    for y in top..=bottom {
        for x in left..=right {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _c)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    let mut cur_dir = 0;

    
    for rounds in 0.. {
        let mut moved = false;
        // first round
        let mut proposed_moves = HashMap::new();
        let mut new_pos: HashMap<(i32, i32), i32> = HashMap::new();
        for &elf in &input {
            if let Some(next_pos) = find_move(elf, cur_dir, &input) {
                proposed_moves.insert(elf, next_pos);
                *new_pos.entry(next_pos).or_insert(0) += 1;
            }
        }

        // second round
        for (cur, next) in &proposed_moves {
            if new_pos.get(next).cloned().unwrap_or(0) <= 1 {
                input.remove(&cur);
                moved = true;
            }
        }

        for (_, &next) in &proposed_moves {
            if new_pos.get(&next).cloned().unwrap_or(0) <= 1 {
                input.insert(next);
            }
        }
        cur_dir = (cur_dir + 1) % 4;
        if !moved {
            dbg!(rounds + 1);
            break;
        }
    }

    let left = input.iter().map(|(x, _y)| *x).min().unwrap();
    let right = input.iter().map(|(x, _y)| *x).max().unwrap();
    let top = input.iter().map(|(_x, y)| *y).min().unwrap();
    let bottom = input.iter().map(|(_x, y)| *y).max().unwrap();
    
    let mut cnt = 0;
    for y in top..=bottom {
        for x in left..=right {
            if !input.contains(&(x, y)) {
                cnt += 1;
            }
        }
    }
    dbg!(cnt);
}
