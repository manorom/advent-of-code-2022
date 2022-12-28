use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
enum Job {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

impl Job {
    fn operands(&self) -> Option<(&String, &String)> {
        match self {
            Self::Number(_) => None,
            Self::Add(lhs, rhs) => Some((lhs, rhs)),
            Self::Sub(lhs, rhs) => Some((lhs, rhs)),
            Self::Mult(lhs, rhs) => Some((lhs, rhs)),
            Self::Div(lhs, rhs) => Some((lhs, rhs)),
        }
    }
}

fn parse_monkey(input: &str) -> (String, Job) {
    let mut iter = input.split(' ');
    let monkey = iter.next().unwrap().trim_end_matches(':').to_owned();
    let n = iter.next().unwrap();
    if let Ok(n) = n.parse::<i64>() {
        return (monkey, Job::Number(n));
    } else {
        let lhs = n.to_owned();
        let op = iter.next().unwrap();
        let rhs = iter.next().unwrap().to_owned();

        (
            monkey,
            match op {
                "+" => Job::Add(lhs, rhs),
                "-" => Job::Sub(lhs, rhs),
                "*" => Job::Mult(lhs, rhs),
                "/" => Job::Div(lhs, rhs),
                _ => panic!(),
            },
        )
    }
}

fn solve_iteratively(input: &HashMap<String, Job>, values: &mut HashMap<String, i64>) {
    for (monkey, i) in input.iter().filter_map(|(monkey, job)| match job {
        Job::Number(n) => Some((monkey.to_owned(), *n)),
        _ => None,
    }) {
        values.insert(monkey.to_owned(), i);
    }
    loop {
        let mut changed = false;
        for (name, job) in input {
            if !values.contains_key(name) {
                if let Some((Some(lhs), Some(rhs))) = job
                    .operands()
                    .map(|(lhs, rhs)| (values.get(lhs), values.get(rhs)))
                {
                    let val = match job {
                        Job::Number(_) => panic!(),
                        Job::Div(..) => lhs / rhs,
                        Job::Add(..) => lhs + rhs,
                        Job::Sub(..) => lhs - rhs,
                        Job::Mult(..) => lhs * rhs,
                    };
                    values.insert(name.clone(), val);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
}

fn solve_reverse(input: &HashMap<String, Job>, values: &mut HashMap<String, i64>) {
    loop {
        let mut changed = false;
        for (name, job) in input {
            let Some(val) = values.get(name).clone() else {
                continue;
            };
            let Some((lhs, rhs)) = job.operands() else {
                continue;
            };
            let lhs_val = values.get(lhs);
            let rhs_val = values.get(rhs);
            match (lhs_val, rhs_val) {
                (Some(lhs_val), None) => {
                    let rhs_val = match job {
                        Job::Add(..) => val - lhs_val,
                        Job::Mult(..) => val / lhs_val,
                        Job::Sub(..) => lhs_val - val, // val = lhs - rhs => val + rhs = lhs => rhs = lhs - val
                        Job::Div(..) => lhs_val / val, // val = lhs / rhs => val * rhs = lhs => lhs / val
                        _ => panic!(),
                    };
                    values.insert(rhs.to_owned(), rhs_val);
                    changed = true;
                }
                (None, Some(rhs_val)) => {
                    let lhs_val = match job {
                        Job::Add(..) => val - rhs_val,
                        Job::Mult(..) => val / rhs_val,
                        Job::Sub(..) => val + rhs_val, // val = lhs - rhs => val + rhs = lhs
                        Job::Div(..) => val * rhs_val, // val = lhs / rhs => lhs = val * rhs
                        _ => panic!(),
                    };
                    values.insert(lhs.to_owned(), lhs_val);
                    changed = true;
                }
                _ => {
                    continue;
                }
            }
        }
        if !changed {
            return;
        }
    }
}

fn task1(input: &HashMap<String, Job>) {
    let mut values = HashMap::new();
    solve_iteratively(input, &mut values);
    dbg!(values.get(&"root".to_owned()));
}

fn task2(input: &HashMap<String, Job>) {
    let mut input = input.clone();
    let (root_lhs, root_rhs) = input.get("root").unwrap().operands().unwrap().clone();
    let (root_lhs, root_rhs) = (root_lhs.to_owned(), root_rhs.to_owned());
    input.remove("root");
    input.remove("humn");

    let mut values = HashMap::new();
    solve_iteratively(&input, &mut values);

    let lhs_solved = values.get(&root_lhs);
    let rhs_solved = values.get(&root_rhs);
    match (lhs_solved, rhs_solved) {
        (Some(&i), None) => {
            values.insert(root_rhs.to_owned(), i);
        }
        (None, Some(&i)) => {
            values.insert(root_lhs.to_owned(), i);
        }
        _ => panic!(),
    }
    solve_reverse(&input, &mut values);
    dbg!(values.get("humn"));
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| parse_monkey(&l.unwrap()))
        .collect::<HashMap<_, _>>();

    task1(&input);
    task2(&input);
}
