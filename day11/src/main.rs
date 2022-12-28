use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum MonkeyOp {
    Add(u64),
    AddOld,
    Mult(u64),
    MultOld,
}

impl MonkeyOp {
    fn worry_level(&self, old: u64) -> u64 {
        match self {
            Self::Add(i) => old + i,
            Self::Mult(i) => old * i,
            Self::AddOld => old + old,
            Self::MultOld => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: MonkeyOp,
    divisible: u64,
    goto_true: usize,
    goto_false: usize,
    inspect: u64,
}

fn read_input_monkey(input: &mut impl Iterator<Item = String>) -> Monkey {
    let _monkey_id = input.next();
    
    let items = input
        .next()
        .unwrap()
        .trim()
        .trim_start_matches("Starting items:")
        .split(',')
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<VecDeque<_>>();
    let op = {
        let line = input.next().unwrap();
        let mut iter = line
            .trim()
            .split_once(' ')
            .unwrap()
            .1
            .trim_start_matches("new = ")
            .split(' ');
        let _left_operand = iter.next().unwrap();
        let op = iter.next().unwrap();
        let right_op = iter.next().unwrap();
        match (op, right_op) {
            ("+", "old") => MonkeyOp::AddOld,
            ("*", "old") => MonkeyOp::MultOld,
            ("+", a) => MonkeyOp::Add(a.parse::<u64>().unwrap()),
            ("*", a) => MonkeyOp::Mult(a.parse::<u64>().unwrap()),
            _ => panic!(),
        }
    };
    let divisible = input
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let goto_true = input
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    let goto_false = input
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    input.next();
    Monkey {
        items,
        op,
        divisible,
        goto_true,
        goto_false,
        inspect: 0,
    }
}

fn main() {
    let mut input = io::stdin().lock().lines().map(|l| l.unwrap()).peekable();
    let mut monkeys = Vec::new();
    while input.peek().is_some() {
        monkeys.push(read_input_monkey(&mut input));
    }

    let divisor = monkeys
        .iter()
        .map(|m| m.divisible)
        .fold(1, |prev, m| prev * m);


    for _ in 0..10_000 {
        for id in 0..monkeys.len() {
            while let Some(old_worry_level) = monkeys[id].items.pop_front() {
                monkeys[id].inspect += 1;
                let new_worry_level = monkeys[id].op.worry_level(old_worry_level) % divisor;
                let next_id = if new_worry_level % monkeys[id].divisible == 0 {
                    monkeys[id].goto_true
                } else {
                    monkeys[id].goto_false
                };
                monkeys[next_id].items.push_back(new_worry_level);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| std::cmp::Reverse(m.inspect));
    dbg!(monkeys[0].inspect * monkeys[1].inspect);
}
