use std::io::{self, BufRead};
use std::collections::{BTreeSet, VecDeque};

fn inbounds(cube: [i32;3], upper_left: [i32; 3], lower_right: [i32; 3]) -> bool {
    for i in 0..3 {
        if cube[i] < upper_left[i] {
            return false;
        }

        if cube[i] > lower_right[i] {
            return false;
        }
    }
    true
}

fn for_neightbors<F>(cube: [i32; 3], mut f: F) where F: FnMut([i32; 3]) {
    let [x, y, z] = cube;
    f([x + 1, y, z]);
    f([x - 1, y, z]);
    f([x, y + 1, z]);
    f([x, y - 1, z]);
    f([x, y, z + 1]);
    f([x, y, z - 1]);
}

fn flood_bfs(start: [i32; 3], end: [i32; 3], cubes: &BTreeSet<[i32; 3]>) -> i32 {
    let mut cnt = 0;
    let mut visited: BTreeSet<[i32; 3]> = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();
        if inbounds(cube, start, end) && !visited.contains(&cube) {
            visited.insert(cube);
            for_neightbors(cube, |n| {
                if cubes.contains(&n) {
                    cnt += 1;
                } else {
                    queue.push_back(n);
                }
            });
        }
    }
    cnt
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.splitn(3, ',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect::<BTreeSet<[i32; 3]>>();
    
    let mut surface_area = 0;

    for p in &input {
        let [x, y, z] = p;
        if !input.contains(&[x+ 1, *y, *z]) {
            surface_area += 1;
        }
        if !input.contains(&[x - 1, *y, *z]) {
            surface_area += 1;
        }
        if !input.contains(&[*x, y + 1, *z]) {
            surface_area += 1;
        }
        if !input.contains(&[*x, y - 1, *z]) {
            surface_area += 1;
        }
        if !input.contains(&[*x, *y, z + 1]) {
            surface_area += 1;
        }
        if !input.contains(&[*x, *y, z - 1]) {
            surface_area += 1;
        }
    }

    dbg!(&surface_area);

    let lower_right = {
        let x = input.iter().map(|[x, y, z]| x).max().unwrap() + 1;
        let y = input.iter().map(|[x, y, z]| y).max().unwrap() + 1;
        let z = input.iter().map(|[x, y, z]| z).max().unwrap() + 1;
        [x, y, z]
    };

    dbg!(flood_bfs([-1, -1, -1], lower_right, &input));
    
}
