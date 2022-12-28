use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    leads_to: Vec<String>,
}

fn parse_line(line: &str) -> Valve {
    let mut iter = line.split(&[' ', '=', ';', ',']);
    let name = iter.nth(1).unwrap().to_owned();
    let flow_rate = iter.nth(3).unwrap().parse().unwrap();
    let leads_to = iter
        .skip(5)
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect::<Vec<_>>();
    Valve {
        name,
        flow_rate,
        leads_to,
    }
}

fn task1_gain<'a>(
    cur: usize,
    time_left: u32,
    valves: &'a BTreeMap<usize, Valve>,
    dists: &'a BTreeMap<(usize, usize), u32>,
    opened: &BTreeSet<usize>,
    path: &mut Vec<(usize, u32)>,
) -> Option<u32> {
    if time_left == 0 {
        return None;
    }

    let cur_valve = valves.get(&cur).unwrap();

    let local_gain = time_left * cur_valve.flow_rate;

    if let Some((_next_valve, next_gain, next_path)) = valves
        .iter()
        .filter(|(k, v)| v.flow_rate > 0 && !opened.contains(&k))
        .filter_map(|(k, _v)| {
            let dist = dists.get(&(cur, *k))?;

            let then_time_left = time_left.checked_sub(dist + 1)?;

            let mut opened = opened.clone();
            opened.insert(*k);

            let mut path = path.clone();
            path.push((*k, *dist));

            let next_gain = task1_gain(*k, then_time_left, valves, dists, &mut opened, &mut path);

            Some((k, next_gain?, path))
        })
        .max_by_key(|(_, g, _)| *g)
    {
        *path = next_path;
        Some(next_gain + local_gain)
    } else {
        Some(local_gain)
    }
}

#[derive(Debug, Clone)]
struct SearchState {
    pos: usize,
    time_left: u32,
    opened: BTreeSet<usize>,
    gain: u32,
}

impl SearchState {
    fn next_states<'a>(
        &'a self,
        dists: &'a BTreeMap<(usize, usize), u32>,
        valves: &'a BTreeMap<usize, Valve>,
    ) -> impl Iterator<Item = SearchState> + 'a {
        valves
            .iter()
            .filter(|(k, v)| v.flow_rate > 0 && !self.opened.contains(&k))
            .filter_map(|(k, v)| {
                dists
                    .get(&(self.pos, *k))
                    .map(|d| (k, v.flow_rate, *d))
            })
            .filter_map(|(k, f, d)| {
                if let Some(n) = self.time_left.checked_sub(d + 1) {
                    let gain = self.gain + n as u32 * f;
                    Some((k, n, gain))
                } else {
                    None
                }
            })
            .map(|(pos, time_left, gain)| {
                let mut opened = self.opened.clone();
                opened.insert(*pos);
                SearchState {
                    pos: *pos,
                    time_left,
                    opened,
                    gain,
                }
            })
    }
}

fn task2a_gain<'a>(
    start: usize,
    valves: &'a BTreeMap<usize, Valve>,
    dists: &'a BTreeMap<(usize, usize), u32>,
) {
    let mut queue = VecDeque::new();
    let mut cache: BTreeMap<BTreeSet<usize>, u32> = BTreeMap::new();

    queue.push_back(SearchState {
        pos: start,
        time_left: 26,
        opened: BTreeSet::new(),
        gain: 0
    });

    while let Some(cur_state) = queue.pop_front() {
        for next_state in cur_state.next_states(dists, valves) {
            if let Some(i) = cache.get(&next_state.opened).cloned() {
                if i < next_state.gain {
                    cache.insert(next_state.opened.clone(), next_state.gain);
                }
            } else {
                cache.insert(next_state.opened.clone(), next_state.gain);
            }
            queue.push_back(next_state);
        }
    }

    //cache.iter().map(|x| cache.iter().map(move |y| (x, y))).flatten();
    let mut m = 0;
    for (ik, iv) in cache.iter() {
        for (jk, jv) in cache.iter() {
            if ik.is_disjoint(jk) {
                m = m.max(iv + jv);
            }
        }
    }

    dbg!(m);
}

// FW all pairs sortest path
fn compute_shortest_paths<'a>(
    valves: &BTreeMap<usize, Valve>,
    dist: &mut BTreeMap<(usize, usize), u32>,
) {
    let valve_idx_map = valves
        .iter()
        .map(|(idx, valve)| (valve.name.clone(), *idx))
        .collect::<BTreeMap<_, usize>>();
    for (idx, valve) in valves {
        dist.insert((*idx, *idx), 0);
        for to in &valve.leads_to {
            let to = valve_idx_map.get(to).unwrap();
            dist.insert((*idx, *to), 1);
        }
    }

    for k in valves.keys().cloned() {
        for i in valves.keys().cloned() {
            for j in valves.keys().cloned() {
                let old = dist.get(&(i, j));
                let new1 = dist.get(&(i, k));
                let new2 = dist.get(&(k, j));
                match (old, new1, new2) {
                    (_, None, None) | (_, None, _) | (_, _, None) => (),
                    (None, Some(n1), Some(n2)) => {
                        dist.insert((i, j), n1 + n2);
                    }
                    (Some(o), Some(n1), Some(n2)) => {
                        if *o > *n1 + *n2 {
                            dist.insert((i, j), n1 + n2);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(idx, l)| (idx, parse_line(&l.unwrap())))
        .collect::<BTreeMap<_, _>>();

    let mut shortest_paths = BTreeMap::new();

    compute_shortest_paths(&input, &mut shortest_paths);

    let aa_idx = *input.iter().find(|(_, v)| v.name == "AA").unwrap().0;

    let mut path = vec![(aa_idx, 0)];

    let max_gain = task1_gain(
        aa_idx,
        30,
        &input,
        &shortest_paths,
        &mut BTreeSet::new(),
        &mut path,
    );

    dbg!(max_gain);

    task2a_gain(aa_idx, &input, &shortest_paths);
}
