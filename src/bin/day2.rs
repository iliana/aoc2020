#![no_std]

use aoc2020::*;

#[derive(Debug)]
struct Input {
    a: usize,
    b: usize,
    letter: char,
    password: &'static str,
}

impl Input {
    fn from_str(s: &'static str) -> Option<Input> {
        let minus = s.find('-')?;
        let space = s.find(' ')?;
        let colon = s.find(": ")?;

        let a = s[0..minus].parse().ok()?;
        let b = s[(minus + 1)..space].parse().ok()?;
        let letter = s[(space + 1)..colon].chars().next()?;
        let password = &s[(colon + 2)..];

        Some(Input {
            a,
            b,
            letter,
            password,
        })
    }
}

fn part1<I: Iterator<Item = Input>>(iter: I) -> usize {
    iter.filter(|i| {
        let count = i.password.chars().filter(|c| *c == i.letter).count();
        i.a <= count && count <= i.b
    })
    .count()
}

fn part2<I: Iterator<Item = Input>>(iter: I) -> usize {
    iter.filter(|i| {
        let a = i.password.chars().nth(i.a - 1).unwrap();
        let b = i.password.chars().nth(i.b - 1).unwrap();
        (a == i.letter) ^ (b == i.letter)
    })
    .count()
}

#[test]
fn test() {
    let lines = test_input!(Input);
    assert_eq!(part1(lines.clone()), 2);
    assert_eq!(part2(lines), 1);
}

fn main() {
    let lines = input!(Input);
    libc_println!("part 1: {}", part1(lines.clone()));
    libc_println!("part 2: {}", part2(lines));
}
