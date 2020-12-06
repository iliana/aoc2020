#![no_std]

use aoc2020::*;

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut buf = [false; 26];
            for c in group.bytes() {
                if c.is_ascii_lowercase() {
                    buf[(c - b'a') as usize] = true;
                }
            }
            buf.iter().filter(|b| **b).count()
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(part1(input!()), 11);
}

fn main() {
    libc_println!("part 1: {}", part1(input!()));
}
