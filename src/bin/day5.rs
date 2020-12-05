#![no_std]

use aoc2020::*;
use core::str;
use heapless::{consts::U1024, Vec};
use itertools::Itertools;

fn seat_id(s: &str) -> u16 {
    let mut buf = [0; 10];
    for (i, c) in s.bytes().enumerate() {
        buf[i] = match c {
            b'F' | b'L' => b'0',
            b'B' | b'R' => b'1',
            _ => unreachable!(),
        };
    }
    u16::from_str_radix(str::from_utf8(&buf).unwrap(), 2).unwrap()
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
