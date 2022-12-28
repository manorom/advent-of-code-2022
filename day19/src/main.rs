use std::{
    collections::{HashSet, VecDeque},
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
struct Robot {
    costs: [u32; 3],
    produces: usize,
}

impl Robot {
    fn can_build(&self, resources: &[u32; 4]) -> bool {
        for (i, c) in self.costs.iter().enumerate() {
            if *c > resources[i] {
                return false;
            }
        }
        true
    }

    fn build(&self, resources: &[u32; 4]) -> (usize, [u32; 4]) {
        let mut resources = resources.clone();
        for (i, c) in self.costs.iter().enumerate() {
            resources[i] -= *c;
        }
        (self.produces, resources)
    }
}

fn parse_blueprint(line: &str) -> [Robot; 4] {
    let mut iter = line.split(' ');
    let ore_cost = iter.nth(6).unwrap().parse::<u32>().unwrap();
    let ore_robot = Robot {
        costs: [ore_cost, 0, 0],
        produces: 0,
    };

    let ore_cost = iter.nth(5).unwrap().parse::<u32>().unwrap();
    let clay_robot = Robot {
        costs: [ore_cost, 0, 0],
        produces: 1,
    };

    let ore_cost = iter.nth(5).unwrap().parse::<u32>().unwrap();
    let clay_cost = iter.nth(2).unwrap().parse::<u32>().unwrap();
    let obsidian_robot = Robot {
        costs: [ore_cost, clay_cost, 0],
        produces: 2,
    };

    let ore_cost = iter.nth(5).unwrap().parse::<u32>().unwrap();
    let obsidian_cost = iter.nth(2).unwrap().parse::<u32>().unwrap();

    let geode_robot = Robot {
        costs: [ore_cost, 0, obsidian_cost],
        produces: 3,
    };

    [ore_robot, clay_robot, obsidian_robot, geode_robot]
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct SearchState {
    minute: u32,
    robots: [u32; 4],
    resources: [u32; 4],
    skipped: [bool; 4],
}

impl SearchState {
    fn next_states<'a>(
        &'a self,
        blueprint: &'a [Robot; 4],
    ) -> impl Iterator<Item = (SearchState, Option<usize>)> + 'a {
        blueprint
            .iter()
            .rev()
            .filter(|r| !self.skipped[r.produces])
            .filter(|r| r.can_build(&self.resources))
            .map(|r| {
                let minute = self.minute + 1;
                let (product, mut resources) = r.build(&self.resources);
                for (robot_idx, num) in self.robots.iter().enumerate() {
                    resources[robot_idx] += num;
                }
                let mut robots = self.robots.clone();
                robots[product] += 1;

                (SearchState {
                    minute,
                    robots,
                    resources,
                    skipped: [false, false, false, false],
                }, Some(r.produces))
            })
            .chain(std::iter::once({
                let minute = self.minute + 1;
                let robots = self.robots.clone();
                let mut resources = self.resources.clone();
                let mut skipped = [false, false, false, false];
                for r in blueprint {
                    if r.can_build(&resources) {
                        skipped[r.produces] = true;
                    }
                }
                for (robot_idx, num) in self.robots.iter().enumerate() {
                    resources[robot_idx] += num;
                }

                (SearchState {
                    minute,
                    robots,
                    resources,
                    skipped,
                }, None)
            }))
    }
}

fn simulate(blueprint: &[Robot; 4], max_minutes: u32) -> u32 {
    let mut bfs_queue: VecDeque<SearchState> = VecDeque::new();

    bfs_queue.push_back(SearchState {
        minute: 1,
        robots: [1, 0, 0, 0],
        resources: [0, 0, 0, 0],
        skipped: [false, false, false, false],
    });

    let mut cache: HashSet<SearchState> = HashSet::new();

    let mut max_geods = 0;

    while !bfs_queue.is_empty() {
        let cur_state = bfs_queue.pop_front().unwrap();
        if cur_state.minute == max_minutes {
            let mut resources = cur_state.resources;
            for (robot_idx, num) in cur_state.robots.iter().enumerate() {
                resources[robot_idx] += num;
            }
            max_geods = max_geods.max(resources[3]);
            continue;
        }

        for (next_state, prod) in cur_state.next_states(blueprint) {
            if !cache.contains(&next_state) {
                bfs_queue.push_back(next_state.clone());
                cache.insert(next_state);
                if prod == Some(3) {
                    break;
                }
            }
        }
    }

    max_geods
}

fn simulate2(blueprint: &[Robot; 4], max_minutes: u32) -> u32 {
    let mut queue = VecDeque::new();
    let mut cache = vec![0; max_minutes as usize + 1];
    queue.push_back(SearchState {
        minute: 0,
        robots: [1, 0, 0, 0],
        resources: [0, 0, 0, 0],
        skipped: [false, false, false, false],
    });

    while !queue.is_empty() {
        let cur_state = queue.pop_front().unwrap();

        let prev_best = cache[cur_state.minute as usize];

        if cur_state.resources[3] < prev_best {
            continue;
        }

        cache[cur_state.minute as usize] = cur_state.resources[3];

        if cur_state.minute == max_minutes {
            continue;
        }

        for (next_state, prod) in cur_state.next_states(blueprint) {
            queue.push_back(next_state.clone());
            if prod == Some(3) {
                break;
            }
        }
    }

    cache[max_minutes as usize]
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| parse_blueprint(&l.unwrap()))
        .collect::<Vec<_>>();

    let mut quality_level = 0;
    for (idx, blueprint) in input.iter().enumerate() {
        //dbg!(simulate(blueprint, 24, &[1, 0, 0, 0], &[0, 0, 0, 0], &[false, false, false, false]));
        quality_level += (idx as u32 + 1) * simulate(blueprint, 24);
    }

    dbg!(quality_level);

    let mut mult = 1;
    
    for blueprint in input.iter().take(3) {
        let n = simulate2(blueprint, 32);
        dbg!(n);
        mult *= n;
    }

    dbg!(mult);
}
