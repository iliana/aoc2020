#![no_std]

use aoc2020::*;
use core::ops::RangeBounds;
use core::str::FromStr;
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

    fn is_complete(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| self.0.contains_key(k))
    }

    fn is_valid(&self) -> bool {
        self.check_int("byr", 1920..=2002)
            && self.check_int("iyr", 2010..=2020)
            && self.check_int("eyr", 2020..=2030)
            && self.check_height()
            && self.check_hair_color()
            && self.check_eye_color()
            && self.check_passport_number()
    }

    fn check_int(&self, key: &'static str, range: impl RangeBounds<u16>) -> bool {
        self.0
            .get(key)
            .and_then(|s| u16::from_str(s).ok())
            .map(|n| range.contains(&n))
            .unwrap_or_default()
    }

    fn check_height(&self) -> bool {
        let height = match self.0.get("hgt") {
            Some(v) => v,
            None => return false,
        };
        let (num, unit) = height.split_at(height.len() - 2);
        if let Ok(num) = u16::from_str(num) {
            match unit {
                "cm" => (150..=193).contains(&num),
                "in" => (59..=76).contains(&num),
                _ => false,
            }
        } else {
            false
        }
    }

    fn check_hair_color(&self) -> bool {
        match self.0.get("hcl").map(|s| s.as_bytes()) {
            Some(v) => {
                v.len() == 7
                    && v[0] == b'#'
                    && v[1..].iter().all(|b| {
                        b.is_ascii_digit() || (b.is_ascii_lowercase() && b.is_ascii_hexdigit())
                    })
            }
            None => false,
        }
    }

    fn check_eye_color(&self) -> bool {
        match self.0.get("ecl") {
            Some(v) => matches!(*v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
            None => false,
        }
    }

    fn check_passport_number(&self) -> bool {
        match self.0.get("pid") {
            Some(v) => v.len() == 9 && v.bytes().all(|b| b.is_ascii_digit()),
            None => false,
        }
    }
}

#[test]
fn test() {
    let input = input!();
    assert_eq!(Iter::new(input).filter(Passport::is_complete).count(), 2);

    assert_eq!(
        Iter::new(
            "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007"
        )
        .filter(Passport::is_valid)
        .count(),
        0,
    );

    assert_eq!(
        Iter::new(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        )
        .filter(Passport::is_valid)
        .count(),
        4,
    );
}

fn main() {
    let input = input!();
    libc_println!(
        "part 1: {}",
        Iter::new(input).filter(Passport::is_complete).count()
    );
    libc_println!(
        "part 2: {}",
        Iter::new(input).filter(Passport::is_valid).count()
    );
}
