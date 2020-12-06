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

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut buf = [0; 26];
            let lines = group.lines().count();
            for c in group.bytes() {
                if c.is_ascii_lowercase() {
                    buf[(c - b'a') as usize] += 1;
                }
            }
            buf.iter().filter(|c| **c == lines).count()
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(part1(input!()), 11);
    assert_eq!(part2(input!()), 6);
}

fn main() {
    libc_println!("part 1: {}", part1(input!()));
    libc_println!("part 2: {}", part2(input!()));
}
