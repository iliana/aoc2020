#![no_std]

use aoc2020::*;
use core::ops::Range;

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
    libc_println!(
        "part 1: {}",
        input!().lines().map(|l| seat_id(l)).max().unwrap()
    );
}
