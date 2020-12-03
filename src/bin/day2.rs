#![no_std]

use aoc2020::*;
use core::str::FromStr;

struct Input {
    min: u8,
    max: u8,
    count: u8,
}

impl Input {
    fn load(s: &str) -> Option<Input> {
        let minus = s.find('-')?;
        let space = s.find(' ')?;
        let colon = s.find(": ")?;

        let min: u8 = s[0..minus].parse().ok()?;
        let max: u8 = s[(minus + 1)..space].parse().ok()?;
        let letter = s[(space + 1)..].chars().next()?;
        Some(Input {
            min,
            max,
            count: s[(colon + 2)..]
                .chars()
                .map(|c| if c == letter { 1 } else { 0 })
                .sum(),
        })
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Input, ()> {
        Input::load(s).ok_or(())
    }
}

fn part1<I: Iterator<Item = Input>>(iter: I) -> u16 {
    iter.map(|i| {
        if i.min <= i.count && i.count <= i.max {
            1
        } else {
            0
        }
    })
    .sum()
}

#[test]
fn test() {
    let lines = test_input!(Input);
    assert_eq!(part1(lines), 2);
}

fn main() {
    let lines = input!(Input);
    libc_println!("part 1: {}", part1(lines));
}
