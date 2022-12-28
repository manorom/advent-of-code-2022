use std::io::{self, BufRead};
use std::iter::successors;

#[derive(Debug)]
enum DirEntry {
    File(String, u32),
    Dir(Vec<DirEntry>, String, u32),
}

impl DirEntry {
    fn root(children: Vec<DirEntry>) -> Self {
        let size = children.iter().map(|c| c.size()).sum::<u32>();
        Self::Dir(children, "/".to_owned(), size)
    }
    fn size(&self) -> u32 {
        match self {
            Self::File(_, s) => *s,
            Self::Dir(_, _, s) => *s,
        }
    }
    fn sum_of_small_dirs(&self) -> u32 {
        if let Self::Dir(c, _, s) = self {
            let small_subdir_size = c.iter().map(|d| d.sum_of_small_dirs()).sum();
            if *s <= 100_000 {
                s + small_subdir_size
            } else {
                small_subdir_size
            }
        } else {
            0
        }
    }

    fn find_smallest(&self, min: u32) -> u32 {
        if let Self::Dir(c, _, s) = self {
            if *s <= min {
                *s
            } else {
                c.iter()
                    .map(|c| c.find_smallest(min))
                    .filter(|s| *s >= min)
                    .min()
                    .unwrap_or(*s)
                    .min(*s)
            }
        } else {
            0
        }
    }
}

fn build_dir_tree(
    lines: &mut impl Iterator<Item = String>,
    cur_dir_node: &mut Vec<DirEntry>,
) -> u32 {
    let mut size = 0;
    while let Some(line) = lines.next() {
        if line.starts_with("$ ls") {
            //
        } else if line.starts_with("$ cd") {
            let (_, name) = line.rsplit_once(' ').unwrap();
            if name == ".." {
                return size;
            } else {
                for d in cur_dir_node.iter_mut() {
                    if let DirEntry::Dir(ref mut v, dname, ref mut dir_size) = d {
                        if name == dname {
                            *dir_size = build_dir_tree(lines, v);
                            size += *dir_size;
                            break;
                        }
                    }
                }
            }
        } else if line.starts_with("dir") {
            let (_, name) = line.rsplit_once(' ').unwrap();
            cur_dir_node.push(DirEntry::Dir(Vec::new(), name.to_owned(), 0));
        } else {
            let (local_size, name) = line.rsplit_once(' ').unwrap();
            let local_size = local_size.parse::<u32>().unwrap();
            cur_dir_node.push(DirEntry::File(name.to_owned(), local_size));
            size += local_size;
        }
    }
    size
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(|l| l.unwrap());

    // Building the directory tree
    let mut root_dir = Vec::new();
    lines.next();
    let root_size = build_dir_tree(&mut lines, &mut root_dir);
    let root_node = DirEntry::root(root_dir);
    dbg!(root_node.sum_of_small_dirs());
    let min_required = 30000000 - (70000000 - root_size);
    dbg!(root_node.find_smallest(min_required));
}
