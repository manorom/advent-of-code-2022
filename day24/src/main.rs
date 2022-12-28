use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, BufRead},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct SearchState {
    x: i32,
    y: i32,
    blizzard: usize,
    phase: usize,
    minute: i32,
}

impl SearchState {
    fn next_states(
        &self,
        down_bound: i32,
        right_bound: i32,
        num_blizzards: usize,
    ) -> impl Iterator<Item = SearchState> + '_ {
        // go up
        let up = (self.x, self.y - 1);
        let down = (self.x, self.y + 1);
        let left = (self.x - 1, self.y);
        let right = (self.x + 1, self.y);
        let stay = (self.x, self.y);

        [up, down, left, right, stay]
            .into_iter()
            .filter(move |(x, y)| {
                (*x > 0 && *x <= right_bound && *y > 0 && *y <= down_bound)
                    || (*x == 1 && *y == 0)
                    || (*x == right_bound && *y == down_bound + 1)
            })
            .map(move |(x, y)| {
                let final_row = down_bound + 1;
                let phase = match (self.phase, x, y) {
                    (0, x, y) if x == right_bound && y == final_row => 1,
                    (1, 1, 0) => 2,
                    (p, _, _) => p,
                };
                SearchState {
                    x,
                    y,
                    blizzard: (self.blizzard + 1) % num_blizzards,
                    minute: self.minute + 1,
                    phase,
                }
            })
    }
}

fn evolve_blizzard(
    prev: &HashSet<(i32, i32, Direction)>,
    down: i32,
    right: i32,
) -> HashSet<(i32, i32, Direction)> {
    let mut next = HashSet::new();
    for (x, y, dir) in prev {
        let (next_x, next_y) = match dir {
            Direction::Up => {
                let mut y = y - 1;
                if y == 0 {
                    y = down;
                }
                (*x, y)
            }
            Direction::Down => {
                let mut y = y + 1;
                if y > down {
                    y = 1;
                }
                (*x, y)
            }
            Direction::Left => {
                let mut x = x - 1;
                if x == 0 {
                    x = right;
                }
                (x, *y)
            }
            Direction::Right => {
                let mut x = x + 1;
                if x > right {
                    x = 1;
                }
                (x, *y)
            }
        };
        next.insert((next_x, next_y, *dir));
    }
    next
}

fn print_field(map: &HashSet<(i32, i32, Direction)>, right_bound: i32, down_bound: i32) {
    for y in 1..=down_bound {
        for x in 1..=right_bound {
            let cnt = Direction::all()
                .filter(|d| map.contains(&(x, y, *d)))
                .count();
            if cnt > 0 {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let initial_blizzards = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .map(|(x, c)| (x, y, c))
                .collect::<Vec<_>>()
        })
        .filter(|(_, _, c)| *c != '#' && *c != '.')
        .map(|(x, y, c)| {
            (
                x as i32,
                y as i32,
                match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    a => panic!("{a}"),
                },
            )
        })
        .map(|(x, y, d)| (x, y, d))
        .collect::<HashSet<_>>();

    let down = *initial_blizzards.iter().map(|(_, y, _)| y).max().unwrap();
    let right = *initial_blizzards.iter().map(|(x, _, _)| x).max().unwrap();

    let num_blizzards = (down * right) as usize;

    let blizzards = std::iter::successors(Some(initial_blizzards), |b| {
        Some(evolve_blizzard(b, down, right))
    })
    .take(num_blizzards)
    .collect::<Vec<_>>();

    let mut cache: HashMap<(i32, i32, usize, usize), i32> = HashMap::new();
    let mut queue: VecDeque<SearchState> = VecDeque::from([SearchState {
        x: 1,
        y: 0,
        blizzard: 0,
        minute: 0,
        phase: 0,
    }]);

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();

        if let Some(prev_time) = cache.get(&(cur.x, cur.y, cur.blizzard, cur.phase)).cloned() {
            if prev_time > cur.minute {
                cache.insert((cur.x, cur.y, cur.blizzard, cur.phase), cur.minute);
            } else {
                continue;
            }
        } else {
            cache.insert((cur.x, cur.y, cur.blizzard, cur.phase), cur.minute);
        }

        for next in cur.next_states(down, right, num_blizzards) {
            // check next tile is blocked
            if Direction::all()
                .filter(|d| blizzards[next.blizzard].contains(&(next.x, next.y, *d)))
                .count()
                > 0
            {
                continue;
            }
            queue.push_back(next);
        }
    }
    dbg!(right, down);
    dbg!(cache
        .iter()
        .filter(|((x, y, _, _), _)| *x == right && *y == down + 1)
        .min_by_key(|((_, _, _, _), d)| *d));

    dbg!(cache
        .iter()
        .filter(|((x, y, _, p), _)| *x == right && *y == down + 1 && *p == 2)
        .min_by_key(|((_, _, _, _), d)| *d));
}
