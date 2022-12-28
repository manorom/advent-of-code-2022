use std::{
    collections::{BTreeSet, HashMap},
    io::{self, BufRead},
};

struct Shape {
    pos: Vec<(i64, i64)>,
    right_bound: i64,
    height: i64,
}

fn height_profile(filled: &BTreeSet<(i64, i64)>, height: i64) -> [i64; 7] {
    let mut r = [0; 7];
    for i in 0..7 {
        r[i] = height
            - filled
                .iter()
                .filter(|(x, _)| *x == i as i64)
                .map(|(_x, y)| *y)
                .max()
                .unwrap_or(0)
    }
    r
}

fn top_rows(filled: &BTreeSet<(i64, i64)>, n: i64) -> String {
    let top_height = *filled.iter().map(|(x, y)| y).max().unwrap();
    let mut rows = String::new();
    for y in ((top_height - n)..=top_height).rev() {
        for x in 0..7 {
            if filled.contains(&(x, y)) {
                rows.push('#');
            } else {
                rows.push('.');
            }
        }
    }

    rows
}

fn print(filled: &BTreeSet<(i64, i64)>, top: i64) {
    for y in (0..top).rev() {
        for x in 0..7 {
            if filled.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn print_with_figure(shape: &Shape, shape_pos: (i64, i64), filled: &BTreeSet<(i64, i64)>) {
    let mut filled = filled.clone();
    filled.extend(
        shape
            .pos
            .iter()
            .map(|(x, y)| (x + shape_pos.0, y + shape_pos.1)),
    );
    let top = *filled.iter().map(|(_, y)| y).max().unwrap() + 1;
    for y in (0..top).rev() {
        for x in 0..7 {
            if filled.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn would_intersect(shape: &Shape, cur_pos: (i64, i64), filled: &BTreeSet<(i64, i64)>) -> bool {
    for pos in shape
        .pos
        .iter()
        .map(|(x, y)| (x + cur_pos.0, y + cur_pos.1))
    {
        if filled.contains(&pos) {
            return true;
        }
    }
    false
}

fn drop_rock(
    shape: &Shape,
    filled: &mut BTreeSet<(i64, i64)>,
    jets: &mut impl Iterator<Item = (usize, char)>,
) -> usize {
    let top = filled.iter().map(|(_, y)| *y).max().unwrap_or(-1);
    let mut shape_pos = (2, top + 4);
    loop {
        let (hidx, hdir) = jets
            .next()
            .map(|(idx, c)| {
                (
                    idx,
                    match c {
                        '<' => -1,
                        '>' => 1,
                        _ => panic!(),
                    },
                )
            })
            .unwrap();

        if shape_pos.0 + hdir >= 0
            && shape_pos.0 + hdir + shape.right_bound <= 8
            && !would_intersect(shape, (shape_pos.0 + hdir, shape_pos.1), filled)
        {
            shape_pos.0 += hdir;
        } else {
        }

        if shape_pos.1 == 0 || would_intersect(shape, (shape_pos.0, shape_pos.1 - 1), filled) {
            // settle
            filled.extend(
                shape
                    .pos
                    .iter()
                    .map(|(x, y)| (x + shape_pos.0, y + shape_pos.1)),
            );
            return hidx;
        } else {
            shape_pos.1 -= 1;
        }
    }
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let shapes = [
        Shape {
            pos: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            right_bound: 5,
            height: 1,
        },
        Shape {
            pos: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            right_bound: 4,
            height: 3,
        },
        Shape {
            pos: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            right_bound: 4,
            height: 3,
        },
        Shape {
            pos: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            right_bound: 2,
            height: 4,
        },
        Shape {
            pos: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            right_bound: 3,
            height: 2,
        },
    ];

    let top_count = shapes.len() as i64 * shapes.iter().map(|x| x.height).max().unwrap_or(0);

    let mut filled_positions: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut jets = input.iter().cloned().enumerate().cycle();
    let mut height = 0;
    let mut cycle = None;
    let mut cache: HashMap<(usize, usize, [i64; 7]), (usize, i64)> = HashMap::new();
    let mut add_virtual_height: i64 = 0;

    let mut rock = 0;
    while rock < 1_000_000_000_000 {
        let shape_idx = rock % 5;
        let shape = &shapes[shape_idx];
        let hidx = drop_rock(&shape, &mut filled_positions, &mut jets);
        height = *filled_positions.iter().map(|(_, y)| y).max().unwrap() + 1;
        rock += 1;

        if cycle.is_none() {
            let height_profile = height_profile(&filled_positions, height);
            if cache.contains_key(&(shape_idx, hidx, height_profile)) {
                let (prev_rock, prev_height) = cache
                    .get(&(shape_idx, hidx, height_profile))
                    .unwrap()
                    .clone();
                cycle = Some((prev_rock, rock, prev_height, height));
                let cycle_size = rock - prev_rock;
                let rocks_left = 1_000_000_000_000 - rock;
                let cycle_height = height - prev_height;
                let add_cycles = rocks_left / cycle_size;
                rock += add_cycles * cycle_size;
                add_virtual_height = add_cycles as i64 * cycle_height;

                dbg!(&cycle);
            } else {
                cache.insert((shape_idx, hidx, height_profile), (rock, height));
            }
        }
    }

    dbg!(height + add_virtual_height);
}
