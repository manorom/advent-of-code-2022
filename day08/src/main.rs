use std::io::{self, BufRead};

fn scenic_score(grid: &Vec<Vec<u32>>, posx: usize, posy: usize) -> usize {
    let tree_height = grid[posx][posy];

    let mut left = 0;
    for n in grid[posx].iter().take(posy).rev() {
        left += 1;
        if *n >= tree_height {
            break;
        }
    }

    let mut right = 0;
    for n in grid[posx].iter().skip(posy + 1) {
        right += 1;
        if *n >= tree_height {
            break;
        }
    }

    let mut top = 0;
    for n in  grid.iter().map(|v| v[posy]).take(posx).rev() {
        top += 1;
        if n >= tree_height {
            break;
        }
    }

    let mut bottom = 0;
    for n in grid.iter().map(|v| v[posy]).skip(posx + 1) {
        bottom += 1;
        if n >= tree_height {
            break;
        }
    }

    left * right * top * bottom
}

fn main() {
    let grid = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let gridlen = grid.len();
    let mut visible = 0;
    let mut max_score = 0;
    for y in 1..(gridlen - 1) {
        for x in 1..(gridlen - 1) {
            let height = grid[x][y];
            max_score = max_score.max(scenic_score(&grid, x, y));


            if grid[x].iter().take(y).all(|v| *v < height) {
                visible += 1;
                continue;
            }
            if grid[x].iter().skip(y + 1).all(|v| *v < height) {
                visible += 1;
                continue;
            }
            if grid.iter().map(|v| v[y]).skip(x + 1).all(|v| v < height) {
                visible += 1;
                continue;
            }
            if grid.iter().map(|v| v[y]).take(x).all(|v| v < height) {
                visible += 1;
                continue;
            }
        }
    }

    dbg!(visible + gridlen + gridlen + 2 * (gridlen - 2));
    dbg!(max_score);
}
