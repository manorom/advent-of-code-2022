use std::{io::{self, BufRead}, collections::BTreeMap};

#[derive(Clone, Debug)]
enum Tile {
    Rock,
    Sand
}

fn place_rock((fromx, fromy): (i32, i32), (tox, toy): (i32, i32)) -> (i32, i32) {
    if fromx < tox {
        (fromx + 1, fromy)
    } else if fromx > tox {
        (fromx - 1, fromy)
    } else if fromy < toy {
        (fromx, fromy + 1)
    } else if fromy > toy {
        (fromx, fromy - 1)
    } else {
        panic!();
    }
}

fn drop_sand(map: &BTreeMap<(i32, i32), Tile>, abyss: i32) -> Option<(i32, i32)> {
    let mut sand_pos = (500, 0);
    loop {
        if !map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
            sand_pos.1 += 1;
        } else if !map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.0 -= 1;
            sand_pos.1 += 1;
        } else if !map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.0 += 1;
            sand_pos.1 += 1;
        } else {
            return Some(sand_pos);
        }
        if sand_pos.1 > abyss {
            return None;
        }
    }
}

fn drop_sand2(map: &BTreeMap<(i32, i32), Tile>, floor: i32) -> Option<(i32, i32)> {
    let mut sand_pos = (500, 0);
    loop {
        if sand_pos.1 + 1 == floor {
            return Some(sand_pos);
        }
        if !map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
            sand_pos.1 += 1;
        } else if !map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos.0 -= 1;
            sand_pos.1 += 1;
        } else if !map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos.0 += 1;
            sand_pos.1 += 1;
        } else {
            return Some(sand_pos);
        }
    }
}

fn main() {
    let mut map = BTreeMap::new();

    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let mut iter = line.split(" -> ").map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (
                x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap(),
            )
        });

        let mut prev = iter.next().unwrap();
        for next in iter {
            while prev != next {
                map.insert(prev, Tile::Rock);
                prev = place_rock(prev, next);
            }
            map.insert(prev, Tile::Rock);
        }
    }

    let mut map2 = map.clone();


    let abyss = map.keys().map(|(_x, y)| *y).max().unwrap() + 1;
    let mut task1_cnt = 0;
    while let Some(p) = drop_sand(&map, abyss) {
        task1_cnt += 1;
        map.insert(p, Tile::Sand);
    }

    dbg!(&task1_cnt);


    let floor = map2.keys().map(|(_x, y)| *y).max().unwrap() + 2;
    let mut task2_cnt = 0;
    while let Some(p) = drop_sand2(&map2, floor) {
        task2_cnt += 1;
        if p == (500, 0) {
            break;
        }
        map2.insert(p, Tile::Sand);
    }

    dbg!(&task2_cnt);
}
