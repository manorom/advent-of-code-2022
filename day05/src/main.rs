use std::io::{self, BufRead};

fn parse_move(txt: &str) -> (usize, usize, usize) {
    let mut iter = txt.split(' ');
    iter.next();
    let num = iter.next().unwrap().parse::<usize>().unwrap();
    iter.next();
    let from = iter.next().unwrap().parse::<usize>().unwrap();
    iter.next();
    let to = iter.next().unwrap().parse::<usize>().unwrap();
    (num, from, to)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).map(|l| parse_move(&l)).collect::<Vec<_>>();
    
    let mut stacks = vec![
        vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'],
        vec!['J', 'R', 'V', 'L'],
        vec!['S', 'Q', 'F'],
        vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
        vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
        vec!['S', 'W', 'T', 'C', 'H', 'F'],
        vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'],
        vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
        vec!['J', 'B', 'W', 'V', 'P']
    ];
    let mut stacks2 = stacks.clone();

    /*
    let mut stacks = vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P']
    ];
    */

    /* 

    for mov in &lines {
        let from_stack_idx = mov.1 - 1;
        let to_stack_idx = mov.2 - 1;
        for _ in 0..mov.0 {
            let c = stacks[from_stack_idx].pop().unwrap();
            stacks[to_stack_idx].push(c);
        }
    }

    let mut top1 = String::new();
    for stack in stacks {
        top1.push(*stack.last().unwrap());
    }
    dbg!(top1);

    */

    for mov in &lines {
        let from_stack_idx = mov.1 - 1;
        let to_stack_idx = mov.2 - 1;
        let mut tmpstack = Vec::new();
        for _ in 0..mov.0 {
            let c = stacks2[from_stack_idx].pop().unwrap();
            tmpstack.push(c);
        }

        for t in tmpstack.iter().rev() {
            stacks2[to_stack_idx].push(*t);
        }
    }

    let mut top2 = String::new();
    for stack in stacks2 {
        top2.push(*stack.last().unwrap());
    }
    dbg!(top2);
}
