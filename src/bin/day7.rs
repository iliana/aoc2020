#![no_std]

use aoc2020::*;
use heapless::consts::{U1024, U8};
use heapless::{FnvIndexMap, Vec};
use itertools::Itertools;

#[derive(Debug)]
struct Rules(FnvIndexMap<&'static str, Vec<(u8, &'static str), U8>, U1024>);

impl Rules {
    fn from_str(s: &'static str) -> Rules {
        let mut map = FnvIndexMap::new();
        for line in s.lines() {
            let (color, line) = line.splitn(2, " bags contain ").collect_tuple().unwrap();
            map.insert(
                color,
                if line == "no other bags." {
                    Vec::new()
                } else {
                    let mut v = Vec::new();
                    for s in line.split(", ") {
                        let (count, s) = s.splitn(2, ' ').collect_tuple().unwrap();
                        let (color, _) = s.splitn(2, " bag").collect_tuple().unwrap();
                        v.push((count.parse().unwrap(), color)).unwrap();
                    }
                    v
                },
            );
        }
        Rules(map)
    }

    fn paths_to(&self, to: &'static str) -> usize {
        self.0
            .iter()
            .filter(|(from, v)| self.has_path_to(from, to))
            .count()
    }

    fn has_path_to(&self, from: &'static str, to: &'static str) -> bool {
        self.0
            .get(from)
            .unwrap()
            .iter()
            .any(|(_, color)| to == *color || self.has_path_to(color, to))
    }
}

#[test]
fn test() {
    let rules = Rules::from_str(input!());
    assert_eq!(rules.paths_to("shiny gold"), 4);
}

fn main() {
    let rules = Rules::from_str(input!());
    libc_println!("part 1: {}", rules.paths_to("shiny gold"));
}
