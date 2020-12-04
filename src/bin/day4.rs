#![no_std]

use aoc2020::*;
use heapless::{consts::U8, LinearMap};
use itertools::Itertools;

struct Iter(core::str::Split<'static, &'static str>);

impl Iter {
    fn new(s: &'static str) -> Iter {
        Iter(s.split("\n\n"))
    }
}

impl Iterator for Iter {
    type Item = Passport;

    fn next(&mut self) -> Option<Passport> {
        self.0.next().map(Passport::from_str)
    }
}

#[derive(Default)]
struct Passport(LinearMap<&'static str, &'static str, U8>);

impl Passport {
    fn from_str(s: &'static str) -> Passport {
        Passport(
            s.split_whitespace()
                .map(|token| token.splitn(2, ':').collect_tuple().unwrap())
                .collect(),
        )
    }

    fn is_valid(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| self.0.contains_key(k))
    }
}

#[test]
fn test() {
    assert_eq!(Iter::new(input!()).filter(Passport::is_valid).count(), 2);
}

fn main() {
    libc_println!("part 1: {}", Iter::new(input!()).filter(Passport::is_valid).count());
}
