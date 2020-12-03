#![no_std]

use aoc2020::*;

fn part1<I: Iterator<Item = u64> + Clone>(iter: I) -> Option<u64> {
    for a in iter.clone() {
        for b in iter.clone() {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }
    None
}

fn part2<I: Iterator<Item = u64> + Clone>(iter: I) -> Option<u64> {
    for a in iter.clone() {
        for b in iter.clone() {
            for c in iter.clone() {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

#[test]
fn test() {
    let iter = test_input!(u64);
    assert_eq!(part1(iter.clone()), Some(1721 * 299));
    assert_eq!(part2(iter), Some(979 * 366 * 675));
}

fn main() {
    let iter = input!(u64);
    libc_println!("part 1: {}", part1(iter.clone()).unwrap());
    libc_println!("part 2: {}", part2(iter).unwrap());
}
