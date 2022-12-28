use std::io::{self, BufRead};

fn snafu_to_dec(snafu: impl DoubleEndedIterator<Item = char>) -> i64 {
    snafu
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            let n = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };
            n * 5i64.pow(idx as u32)
        })
        .sum()
}


fn dec_to_snafu(dec: i64) -> Vec<char> {
    let mut snafu = Vec::new();
    let mut cur_val = 0;
    let mut cur_idx = 0;

    while cur_val < dec {
        snafu.push('2');
        cur_val += 2 * 5i64.pow(cur_idx);
        cur_idx += 1;
    }

    let snafu_len = snafu.len() - 1;
    
    while cur_idx > 0 {
        cur_idx -= 1;
        if dec <= cur_val - 4 * 5i64.pow(cur_idx) {
            cur_val -= 4 * 5i64.pow(cur_idx);
            snafu[snafu_len - cur_idx as usize] = '=';
        } else if dec <= cur_val - 3 * 5i64.pow(cur_idx) {
            cur_val -= 3 * 5i64.pow(cur_idx);
            snafu[snafu_len - cur_idx as usize] = '-';
        } else if dec <= cur_val - 2 * 5i64.pow(cur_idx) {
            cur_val -= 2 * 5i64.pow(cur_idx);
            snafu[snafu_len - cur_idx as usize] = '0';
        } else if dec <= cur_val - 1 * 5i64.pow(cur_idx) {
            cur_val -= 5i64.pow(cur_idx);
            snafu[snafu_len - cur_idx as usize] = '1';
        }

        if dec == cur_val {
            break;
        }
    }

    snafu
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let decimal_sum: i64 = input.iter().map(|s| snafu_to_dec(s.iter().cloned())).sum();

    dbg!(decimal_sum);
    dbg!(dec_to_snafu(decimal_sum).into_iter().collect::<String>());
}
