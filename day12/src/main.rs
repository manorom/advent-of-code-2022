use std::collections::BTreeSet;
use std::io::{self, BufRead};
use std::vec;

fn height(c: char) -> i32 {
    if c == 'S' {
        0
    } else if c == 'E' {
        25
    } else {
        c.to_digit(36).unwrap() as i32 - 10
    }
}

fn neighbor_viable(grid: &Vec<Vec<char>>, cur: (usize, usize), next: (usize, usize)) -> bool {
    let next_char = grid[next.0][next.1];
    let cur_char = grid[cur.0][cur.1];
    let height_diff = height(cur_char) - height(next_char);
    if height_diff <= 1 {
        true
    } else {
        false
    }
}

fn neighbors<F>(grid: &Vec<Vec<char>>, pos: (usize, usize), mut f: F)
where
    F: FnMut(usize, usize),
{
    let isize = grid.len();
    let jsize = grid[0].len();

    if pos.0 > 0 && neighbor_viable(grid, pos, (pos.0 - 1, pos.1)) {
        f(pos.0 - 1, pos.1);
    }

    if pos.1 > 0 && neighbor_viable(grid, pos, (pos.0, pos.1 - 1)) {
        f(pos.0, pos.1 - 1);
    }

    if pos.0 + 1 < isize && neighbor_viable(grid, pos, (pos.0 + 1, pos.1)) {
        f(pos.0 + 1, pos.1);
    }

    if pos.1 + 1 < jsize && neighbor_viable(grid, pos, (pos.0, pos.1 + 1)) {
        f(pos.0, pos.1 + 1);
    }
}

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> (i32, i32) {
    let isize = grid.len();
    let jsize = grid[0].len();

    let mut q = BTreeSet::new();

    for i in 0..isize {
        for j in 0..jsize {
            q.insert((i, j));
        }
    }

    let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; jsize]; isize];

    dist[end.0][end.1] = 0;

    while !q.is_empty() {
        let u = *q
            .iter()
            .min_by(|(i, j), (ii, jj)| dist[*i][*j].cmp(&dist[*ii][*jj]))
            .unwrap();

        q.remove(&u);

        neighbors(grid, u, |i, j| {
            if q.contains(&(i, j)) {
                let alt = dist[u.0][u.1].saturating_add(1);
                if dist[i][j] > alt {
                    dist[i][j] = alt;
                }
            }
        })
    }

    let fewest_from_start = dist[start.0][start.1];
    let fewest_from_a = (0..isize)
        .map(move |i| (0..jsize).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| grid[*i][*j] == 'a')
        .map(|(i, j)| dist[i][j])
        .min()
        .unwrap();

    (fewest_from_start, fewest_from_a)
}

fn flood_plane(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> (u32, u32) {
    let isize = grid.len();
    let jsize = grid[0].len();
    let mut flood: Vec<Vec<Option<u32>>> = vec![vec![None; jsize]; isize];

    flood[end.0][end.1] = Some(0);

    let mut updated = true;
    while updated {
        updated = false;
        for i in 0..isize {
            for j in 0..jsize {
                if let Some(s) = flood[i][j] {
                    let next_step = s + 1;
                    neighbors(grid, (i, j), |ii, jj| {
                        if flood[ii][jj].is_none() {
                            flood[ii][jj] = Some(next_step);
                            updated = true;
                        }
                    });
                }
            }
        }
    }

    let fewest_from_a = (0..isize)
        .map(|i| (0..jsize).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| grid[*i][*j] == 'a' || grid[*i][*j] == 'S')
        .filter(|(i, j)| flood[*i][*j].is_some())
        .min_by(|(i, j), (ii, jj)| flood[*i][*j].unwrap().cmp(&flood[*ii][*jj].unwrap()))
        .unwrap();

    (
        flood[start.0][start.1].unwrap(),
        flood[fewest_from_a.0][fewest_from_a.1].unwrap(),
    )
}

fn find_element(grid: &Vec<Vec<char>>, find: char) -> (usize, usize) {
    let isize = grid.len();
    let jsize = grid[0].len();

    (0..isize)
        .flat_map(|i| (0..jsize).map(move |j| (i, j, grid[i][j])))
        .find(|(_, _, c)| *c == find)
        .map(|(i, j, _)| (i, j))
        .unwrap()
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start = find_element(&input, 'S');
    let end = find_element(&input, 'E');

    {
        let now = std::time::Instant::now();
        let (from_start, from_a) = flood_plane(&input, start, end);
        let time = now.elapsed();
        println!(
            "Flood approach: Fewest rom start {}, Fewest from a {}. took {:?}",
            from_start, from_a, time
        );
    }

    {
        let now = std::time::Instant::now();
        let (from_start, from_a) = dijkstra(&input, start, end);
        let time = now.elapsed();
        println!(
            "Dijkstra approach: Fewest rom start {}, Fewest from a {}. took {:?}",
            from_start, from_a, time
        );
    }
}
