use std::io::{self, BufRead};
use std::collections::HashSet;

fn move_head(head_pos: (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        'R' => (head_pos.0 + 1, head_pos.1),
        'L' => (head_pos.0 - 1, head_pos.1),
        'U' => (head_pos.0, head_pos.1 + 1),
        'D' => (head_pos.0, head_pos.1 - 1),
        _ => panic!()
    }
}

fn move_knot(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let tail_offset_x = tail_pos.0 - head_pos.0;
    let tail_offset_y = tail_pos.1 - head_pos.1;

    match (tail_offset_x, tail_offset_y) {
        (2, 0) => (tail_pos.0 - 1, tail_pos.1),
        (-2, 0) => (tail_pos.0 + 1, tail_pos.1),
        (0, 2) => (tail_pos.0, tail_pos.1 - 1),
        (0, -2) => (tail_pos.0, tail_pos.1 + 1),
        (2, 1) | (1, 2) | (2, 2) => (tail_pos.0 - 1, tail_pos.1 - 1),
        (-2, 1) | (-1, 2) | (-2, 2) => (tail_pos.0 + 1, tail_pos.1 - 1),
        (2, -1) | (1, -2) | (2, -2) => (tail_pos.0 - 1, tail_pos.1 + 1),
        (-2, -1) | (-1, -2) | (-2, -2) => (tail_pos.0 + 1, tail_pos.1 + 1),
        _ => tail_pos,
    }
}

fn task1(input: &[(char, u32)]) {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut all_tail_posittions = HashSet::new();
    all_tail_posittions.insert(tail_pos);

    for (direction, num) in input {
        for _ in 0..*num {
            head_pos = move_head(head_pos, *direction);

            tail_pos = move_knot(head_pos, tail_pos);

            //tail_pos = (tail_pos.0 + move_tail_x, tail_pos.1 + move_tail_y);
            all_tail_posittions.insert(tail_pos);
        }
    }

    dbg!(all_tail_posittions.len());
}

fn task2(input: &[(char, u32)]) {
    let mut knots = [(0, 0); 10];
    let mut all_tail_posittions = HashSet::new();
    all_tail_posittions.insert((0, 0));

    for (direction, num) in input {
        for _ in 0..*num {
            knots[0] =  move_head(knots[0], *direction);
            for i in 1..knots.len() {
                knots[i] = move_knot(knots[i-1], knots[i]);
            }
            all_tail_posittions.insert(knots[9]);
        }
    }
    dbg!(all_tail_posittions.len());
    
    
}

fn main() {
    let input = io::stdin().lock().lines().map(|l| l.unwrap()).map(|l| {
        let (direction, num) = l.split_once(' ').unwrap();
        let num = num.parse::<u32>().unwrap();
        (direction.chars().next().unwrap(), num)
    }).collect::<Vec<(char, u32)>>();

    task1(&input);
    task2(&input);
}
