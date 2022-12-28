use std::{
    collections::HashMap,
    io::{self, BufRead}
};

fn parse_input(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut iter = line.split(' ');
    let sensor_x = iter.nth(2).unwrap()[2..]
        .trim_end_matches(',')
        .parse::<i32>()
        .unwrap();
    let sensor_y = iter.next().unwrap()[2..]
        .trim_end_matches(':')
        .parse::<i32>()
        .unwrap();

    let beacon_x = iter.nth(4).unwrap()[2..]
        .trim_end_matches(',')
        .parse::<i32>()
        .unwrap();
    let beacon_y = iter.next().unwrap()[2..].parse::<i32>().unwrap();

    ((sensor_x, sensor_y), (beacon_x, beacon_y))
}

//const Y: i32 = 10;
const Y: i32 = 2000000;

fn task1(input: &HashMap<(i32,i32), (i32, i32)>) {
    let mut x_blocks: Vec<(i32, i32)> = Vec::new();
    for (sensor, beacon) in input {
        let sensor_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let x_span = sensor_distance - (Y - sensor.1).abs();

        if x_span < 0 {
            continue;
        }

        x_blocks.push((sensor.0 - x_span, sensor.0 + x_span))
    }

    x_blocks.sort_unstable_by_key(|(from, _)| *from);

    let mut current_block = x_blocks[0];
    let mut cnt = 0;

    for block in x_blocks.iter().skip(1) {
        if current_block.0 <= block.0 && current_block.1 >= block.1 {
            continue;
        }

        if current_block.0 <= block.0 && current_block.1 <= block.1 && current_block.0 <= block.1 {
            current_block.1 = block.1;
        } else {
            dbg!(current_block);
            cnt += current_block.1 - current_block.0;
            current_block = *block;
        }

    }

    cnt += current_block.1 - current_block.0;

    dbg!(cnt);

}

fn task2(input: &HashMap<(i32,i32), (i32, i32)>) {

    let sensors = input.iter().map(|(k, v)| {
        (k.0, k.1, (k.0 - v.0).abs() + (k.1 - v.1).abs())
    }).collect::<Vec<(i32, i32, i32)>>();

    for l in 0..=4_000_000 {
        let mut x_blocks: Vec<(i32, i32)> = Vec::new();
        for (x, y, dist) in &sensors {
            let x_span = dist - (l - y).abs();

            if x_span < 0 {
                continue;
            }

            x_blocks.push(((x - x_span).max(0), (x + x_span).min(4_000_000)))
        }

        x_blocks.sort_unstable_by_key(|(from, _)| *from);

        let mut current_block = x_blocks[0];

        'inner: for block in x_blocks.iter().skip(1) {
            if current_block.0 <= block.0 && current_block.1 >= block.1 {
                continue 'inner;
            }

            if current_block.0 <= block.0 && current_block.1 >= block.0 && current_block.1 <= block.1 {
                current_block.1 = block.1;
            } else {
                println!("found: {}", l as u64 + (4_000_000u64 * (current_block.1 as u64 + 1)));
                return;
            }
        }

    }
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_input(&l))
        .collect::<HashMap<(i32, i32), (i32, i32)>>();

    //task1(&input);
    task2(&input);

}
