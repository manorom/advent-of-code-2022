use std::{
    collections::VecDeque,
    io::{self, BufRead},
    ops::Rem,
};

fn main() {
    let numbers = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut order = (0..numbers.len()).collect::<VecDeque<_>>();

    let modulus = numbers.len() as i64 - 1;
    let rounds = 10;
    let key = 811589153;

    for _ in 0..rounds {

        for i in 0..numbers.len() {
            let cur_pos = order.iter().position(|p| *p == i).unwrap();
            let elem = numbers[i] * key;
            let new_pos = (cur_pos as i64 + elem).rem_euclid(modulus) as usize;
            order.remove(cur_pos);
            order.insert(new_pos, i);
        }
    }

    let ordered_numbers = order.iter().map(|p| numbers[*p]).collect::<Vec<_>>();
    let zero_idx = ordered_numbers.iter().position(|n| *n == 0).unwrap();
    dbg!([1000, 2000, 3000]
        .iter()
        .map(|add| ordered_numbers[(zero_idx + add) % ordered_numbers.len()] as i64 * key)
        .sum::<i64>());
}
