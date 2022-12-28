use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let split = l.split_once(' ').unwrap();
            (split.0.chars().next().unwrap(), split.1.chars().next().unwrap())
        })
        .collect::<Vec<(char, char)>>();

    {
        let mut score = 0;
        for line in &lines {
            let win_score = match (line.0, line.1) {
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                a => panic!("{:?}", a),
            };

            let choice_score = match line.1 {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!(),
            };

            score += win_score + choice_score;
        }

        println!("Task 1 {}", score);
    }

    {
        let mut score = 0;
        for line in &lines {
            let win_score = match line.1 {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                a => panic!("{:?}", a)
            };

            let choice_score = match (line.0, line.1) {
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1, // chose rock
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2, // chose paper
                ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3, // chose scisscors
                _ => panic!(),
            };

            println!("choice score: {}, win score: {}", choice_score, win_score);
            score += win_score + choice_score;
        }
        println!("Task 2 {}", score);

    }
}
