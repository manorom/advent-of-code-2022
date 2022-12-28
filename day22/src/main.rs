use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Solid,
}
#[derive(Debug, Clone)]
enum Path {
    Move(i32),
    R,
    L,
}

fn parse_path(input: &str) -> Vec<Path> {
    let mut desc = Vec::new();
    let mut iter = input.chars().enumerate().peekable();
    while iter.peek().is_some() {
        match iter.peek().unwrap().clone() {
            (_, 'R') => {
                desc.push(Path::R);
                iter.next();
            }
            (_, 'L') => {
                desc.push(Path::L);
                iter.next();
            }
            (startidx, l) if l.is_numeric() => {
                let mut len = 0;
                while iter.peek().map(|(_, c)| c.is_numeric()).unwrap_or(false) {
                    len += 1;
                    iter.next();
                }
                desc.push(Path::Move(
                    input[startidx..(startidx + len)].parse::<i32>().unwrap(),
                ))
            }
            _ => {
                panic!();
            }
        }
    }
    desc
}

fn step(
    pos: (i32, i32),
    orientation: (i32, i32),
    map: &HashMap<(i32, i32), Tile>,
) -> Option<((i32, i32), (i32, i32))> {
    let mut next_pos = (pos.0 + orientation.0, pos.1 + orientation.1);
    let mut next_orientation = orientation;
    match (next_pos, next_orientation) {
        ((150, y), (1, 0)) if y < 50 => {
            next_pos = (99, 149 - y);
            next_orientation =  (-1, 0);
        },
        ((100, y), (1, 0)) if y >= 50 && y < 100 => {
            next_pos = (100 + y - 50, 49);
            next_orientation = (0, -1);
        },
        ((100, y), (1, 0)) if y >= 100 && y < 150 => {
            next_pos = (149, 149 - y);
            next_orientation = (-1, 0);
        },
        ((50, y), (1, 0)) if y >= 150 => {
            next_pos = (y - 150 + 50, 149);
            next_orientation = (0, -1);
        },
        ((49, y), (-1, 0)) if y < 50 => {
            next_pos = (0, 149 - y);
            next_orientation = (1, 0);
        }
        ((49, y), (-1, 0)) if y >= 50 && y < 100 => {
            next_orientation = (0, 1);
            next_pos = (y - 50, 100);
        }
        ((-1, y), (-1, 0)) if y >= 100 && y < 150 => {
            next_orientation = (1, 0);
            next_pos = (50, 49 - (y - 100));
        }
        ((-1, y), (-1, 0)) if y >= 150 && y < 200 => {
            next_orientation = (0, 1);
            next_pos = (y - 150 + 50, 0);
        }
        ((x, 99), (0, -1)) if x >= 0 && x < 50 => {
            next_orientation = (1, 0);
            next_pos = (50, 50 + x);
        }
        ((x, -1), (0, -1)) if x >= 50 && x < 100 => {
            next_orientation = (1, 0);
            next_pos = (0, 150 + x - 50);
        }
        ((x, -1), (0, -1)) if x >= 100 && x < 150 => {
            next_orientation = (0, -1);
            next_pos = (x - 100, 199);
        }
        ((x, 200), (0, 1)) if x >= 0 && x < 50 => {
            next_orientation = (0, 1);
            next_pos = (x + 100, 0);
        }
        ((x, 150), (0, 1)) if x >= 50 && x < 100 => {
            next_orientation = (-1, 0);
            next_pos = (49, x - 50 + 150);
        }
        ((x, 50), (0, 1)) if x >= 100 && x < 150 => {
            next_orientation = (-1, 0);
            next_pos = (99, x - 100 + 50);
        }
        
        _ => (),
    }

    match map.get(&next_pos) {
        Some(Tile::Open) => Some((next_pos, next_orientation)),
        Some(Tile::Solid) => None,
        None => panic!("{:?} is out of range", next_pos)
    }  
}

fn move_to2(
    n: i32,
    pos: (i32, i32),
    orientation: (i32, i32),
    map: &HashMap<(i32, i32), Tile>
) -> ((i32, i32), (i32, i32)) {
    let mut next_pos = pos;
    let mut next_orientation = orientation;
    for _ in 0..n {
        if let Some((p, o)) = step(next_pos, next_orientation, map) {
            next_pos = p;
            next_orientation = o;
        } else {
            break;
        }
    }

    (next_pos, next_orientation)
}

fn move_to(
    n: i32,
    pos: (i32, i32),
    orientation: (i32, i32),
    map: &HashMap<(i32, i32), Tile>,
) -> (i32, i32) {
    let mut cur_pos = pos;
    let mut i = 0;
    while i < n {
        let next_pos = {
            let mut peek = (cur_pos.0 + orientation.0, cur_pos.1 + orientation.1);
            match map.get(&peek) {
                Some(Tile::Solid) => None,
                Some(Tile::Open) => Some(peek),
                None => {
                    while map.contains_key(&(peek.0 - orientation.0, peek.1 - orientation.1)) {
                        peek = (peek.0 - orientation.0, peek.1 - orientation.1);
                    }
                    match map.get(&peek) {
                        Some(Tile::Open) => Some(peek),
                        Some(Tile::Solid) => None,
                        _ => panic!(),
                    }
                }
            }
        };

        if let Some(p) = next_pos {
            cur_pos = p;
        } else {
            return cur_pos;
        }
        i += 1;
    }
    cur_pos
}

fn main() {
    let mut input = io::stdin().lock().lines().map(|l| l.unwrap());
    let board = input
        .by_ref()
        .take_while(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_x, c)| !c.is_ascii_whitespace())
                .map(|(x, c)| {
                    (
                        (x as i32, y as i32),
                        match c {
                            '#' => Tile::Solid,
                            '.' => Tile::Open,
                            _ => panic!(),
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i32, i32), Tile>>();

    let instr = parse_path(&input.next().unwrap());

    let mut pos = board
        .keys()
        .map(|(x, y)| (*x, *y))
        .filter(|(_x, y)| *y == 0)
        .min_by_key(|(x, _y)| *x)
        .unwrap();
    let mut orientation: (i32, i32) = (1, 0);
    for i in &instr {
        match i {
            Path::Move(n) => {
                (pos, orientation) = move_to2(*n, pos, orientation, &board);
            }
            Path::L => {
                orientation = match orientation {
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    _ => panic!(),
                }
            }
            Path::R => {
                orientation = match orientation {
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    (0, 1) => (-1, 0),
                    (0, -1) => (1, 0),
                    _ => panic!(),
                }
            }
        }
    }
    let facing = match orientation {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!(),
    };

    dbg!(facing);
    dbg!(pos.1 + 1);
    dbg!(pos.0 + 1);
    dbg!(1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing);
}
