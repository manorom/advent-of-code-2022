use std::io::{self, BufRead};
use std::iter::Peekable;

#[derive(Clone, Debug)]
enum PacketEntry {
    Item(i32),
    List(Vec<PacketEntry>),
}

fn parse_packet2<'a>(
    input_iter: &mut impl Iterator<Item = &'a str>
) -> Vec<PacketEntry> {
    let mut packet = Vec::new();
    while let Some(token) = input_iter.next() {
        match token {
            "," => (),
            "[" => {
                packet.push(PacketEntry::List(parse_packet2(input_iter)));
            },
            "]" => return packet,
            num => {
                dbg!(num);
                packet.push(PacketEntry::Item(num.parse::<i32>().unwrap()));
            }
        }
    }

    packet
}

struct Tokenize<'a>(&'a str);

impl<'a> Iterator for Tokenize<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item>{
        let mut iter = self.0.chars().enumerate().peekable();
        match iter.next() {
            Some((_, ',')) | Some((_, '[')) | Some((_, ']')) => {
                let ret = &self.0[..1];
                self.0 = &self.0[1..];
                return Some(ret);
            },
            _ => ()
        }
        while let Some((idx, c)) = iter.peek() {
            if *c == ',' || *c == '[' || *c == ']' {
                let ret = &self.0[..(*idx)];
                self.0 = &self.0[(*idx)..];
                return Some(ret);
            }
        }

        None
    }
}


fn parse_packet(
    input_str: &str,
    input_iter: &mut Peekable<impl Iterator<Item = (usize, char)>>,
) -> Vec<PacketEntry> {
    let mut packet = Vec::new();
    input_iter.next();
    while let Some((peek_idx, peek_char)) = input_iter.peek() {
        if *peek_char == ',' {
            input_iter.next();
            continue;
        } else if *peek_char == '[' {
            packet.push(PacketEntry::List(parse_packet(input_str, input_iter)));
            input_iter.next();
            continue;
        } else if peek_char.is_numeric() {
            let start_idx = *peek_idx;
            let mut end_idx = *peek_idx;
            while input_iter
                .peek()
                .map(|(_, c)| c.is_numeric())
                .unwrap_or(false)
            {
                end_idx += 1;
                input_iter.next();
            }
            packet.push(PacketEntry::Item(
                input_str[start_idx..end_idx].parse::<i32>().unwrap(),
            ));
        } else if *peek_char == ']' {
            break;
        } else {
            panic!("{}", peek_char);
        }
    }

    packet
}

fn compare(left: &PacketEntry, right: &PacketEntry) -> Option<bool> {
    match (left, right) {
        (PacketEntry::Item(i), PacketEntry::Item(j)) => {
            if i > j {
                Some(false)
            } else if i < j {
                Some(true)
            } else {
                None
            }
        }
        (PacketEntry::Item(i), PacketEntry::List(_)) => {
            let tmp = PacketEntry::List(vec![PacketEntry::Item(*i)]);
            compare(&tmp, right)
        }
        (PacketEntry::List(_), PacketEntry::Item(j)) => {
            let tmp = PacketEntry::List(vec![PacketEntry::Item(*j)]);
            compare(left, &tmp)
        }
        (PacketEntry::List(l), PacketEntry::List(r)) => {
            let mut liter = l.iter();
            let mut riter = r.iter();
            loop {
                match (liter.next(), riter.next()) {
                    (None, None) => break,
                    (None, _) => return Some(true),
                    (_, None) => return Some(false),
                    (Some(ll), Some(rr)) => {
                        match compare(ll, rr) {
                            Some(b) => return Some(b),
                            None => (),
                        }
                    }
                }
            }
            None
        }
    }
}

fn is_divider(packet: &PacketEntry, i: i32) -> bool {
    match packet {
        PacketEntry::List(l) if l.len() == 1 => {
            match l.get(0) {
                Some(PacketEntry::List(ll)) if ll.len() == 1 => {
                    match ll.get(0) {
                        Some(PacketEntry::Item(p)) if *p == i => {
                            return true;
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        },
        _ => ()
    }
    false
}

fn main() {
    let mut line_iter = io::stdin().lock().lines().map(|l| l.unwrap()).peekable();
    let mut pairs = Vec::new();
    while line_iter.peek().is_some() {
        let line1 = line_iter.next().unwrap();
        let line2 = line_iter.next().unwrap();

        pairs.push((
            PacketEntry::List(parse_packet2(&mut line1.split(&[',', '[', ']']))),
            PacketEntry::List(parse_packet2(&mut line2.split_inclusive(&[',', '[', ']']))),
        ));
        line_iter.next();
    }

    let mut idx_sum = 0;

    for (idx, pair) in pairs.iter().enumerate() {
        if compare(&pair.0, &pair.1) == Some(true) {
            idx_sum += idx + 1;
        }
    }

    dbg!(idx_sum);

    let mut all_packets = Vec::new();
    for pair in pairs {
        all_packets.push(pair.0);
        all_packets.push(pair.1);
    }
    let divider1 = PacketEntry::List(vec![PacketEntry::List(vec![PacketEntry::Item(2)])]);
    let divider2 = PacketEntry::List(vec![PacketEntry::List(vec![PacketEntry::Item(6)])]);
    all_packets.push(divider1);
    all_packets.push(divider2);


    all_packets.sort_by(|x, y| {
        match compare(x, y) {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            _ => panic!(),
        }
    });

    let decoder_key: usize = all_packets.iter().enumerate().filter_map(|(idx, p)| {
        if is_divider(p, 2) || is_divider(p, 6) {
            Some(idx + 1)
        } else {
            None
        }
    }).product();

    dbg!(decoder_key);
}
