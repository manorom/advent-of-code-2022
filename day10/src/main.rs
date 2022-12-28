use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Instr {
    Noop,
    Addx(i32),
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        if value == "noop" {
            Self::Noop
        } else {
            let (_, add) = value.split_once(' ').unwrap();
            Self::Addx(add.parse::<i32>().unwrap())
        }
    }
}

fn draw(cycle: i32, x: i32) {
    if cycle % 40 == 0 {
        println!("");
    }
    let cur_pos = cycle % 40;
    if cur_pos == x || cur_pos == x - 1 || cur_pos == x + 1 {
        print!("#");
    } else {
        print!(".")
    }
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| Instr::from(l.unwrap().as_str()))
        .collect::<Vec<_>>();

    let mut sum_signal_strenght = 0;

    let exec = input
        .iter()
        .flat_map(|i| match i {
            Instr::Noop => std::iter::once((i.clone(), true)).chain(None),
            Instr::Addx(_) => std::iter::once((i.clone(), false)).chain(Some((i.clone(), true))),
        })
        .enumerate()
        .scan(1, |x, (cycle, (instr, step))| {
            let x_before = *x;
            *x = if let (Instr::Addx(i), true) = (&instr, step) {
                x_before + i
            } else {
                x_before
            };
            Some((cycle as i32 + 1, x_before))
        });

    for (cycle, x_before) in exec {
        draw(cycle - 1, x_before);

        if (cycle - 20) % 40 == 0 {
            sum_signal_strenght += cycle * x_before;
        }
    }

    println!("\nSum of signal strengths = {sum_signal_strenght}");
}
