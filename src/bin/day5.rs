#![no_std]

use aoc2020::*;
use core::ops::Range;
use heapless::{consts::U1024, Vec};
use itertools::Itertools;

fn seat_id(s: &str) -> u16 {
    let mut row = 0..128;
    let mut col = 0..8;

    for c in s[..7].chars() {
        match c {
            'F' => row = lower(row),
            'B' => row = upper(row),
            _ => {}
        }
    }

    for c in s[7..].chars() {
        match c {
            'L' => col = lower(col),
            'R' => col = upper(col),
            _ => {}
        }
    }

    row.start * 8 + col.start
}

fn lower(r: Range<u16>) -> Range<u16> {
    (r.start)..((r.end + r.start) / 2)
}

fn upper(r: Range<u16>) -> Range<u16> {
    ((r.end + r.start) / 2)..(r.end)
}

#[test]
fn test() {
    assert_eq!(seat_id("FBFBBFFRLR"), 357);
}

fn main() {
    let mut seats: Vec<u16, U1024> = input!().lines().map(|l| seat_id(l)).collect();
    seats.sort_unstable();
    libc_println!("part 1: {}", seats[seats.len() - 1]);
    for (a, b) in seats.iter().tuple_windows() {
        if b - a == 2 && (8..1016).contains(&(b - 1)) {
            libc_println!("part 2: {}", b - 1);
            break;
        }
    }
}
