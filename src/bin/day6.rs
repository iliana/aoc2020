#![no_std]

use aoc2020::*;

struct Group {
    counts: [u8; 26],
    len: usize,
}

impl Group {
    fn from_str(s: &str) -> Option<Group> {
        let mut counts = [0; 26];
        let len = s.lines().count();
        for c in s.bytes().filter(u8::is_ascii_lowercase) {
            counts[(c - b'a') as usize] += 1
        }
        Some(Group { counts, len })
    }
}

fn part1<I: Iterator<Item = Group>>(iter: I) -> usize {
    iter.map(|group| group.counts.iter().filter(|count| **count > 0).count())
        .sum()
}

fn part2<I: Iterator<Item = Group>>(iter: I) -> usize {
    iter.map(|group| {
        group
            .counts
            .iter()
            .filter(|count| usize::from(**count) == group.len)
            .count()
    })
    .sum()
}

#[test]
fn test() {
    let iter = input!(Group, "\n\n");
    assert_eq!(part1(iter.clone()), 11);
    assert_eq!(part2(iter), 6);
}

fn main() {
    let iter = input!(Group, "\n\n");
    libc_println!("part 1: {}", part1(iter.clone()));
    libc_println!("part 2: {}", part2(iter));
}
