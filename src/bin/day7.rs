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
            let (color, line) = line
                .trim()
                .splitn(2, " bags contain ")
                .collect_tuple()
                .unwrap();
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
            )
            .unwrap();
        }
        Rules(map)
    }

    fn paths_to(&self, to: &'static str) -> usize {
        self.0
            .iter()
            .filter(|(from, _)| self.has_path_to(from, to))
            .count()
    }

    fn has_path_to(&self, from: &'static str, to: &'static str) -> bool {
        self.0
            .get(from)
            .unwrap()
            .iter()
            .any(|(_, color)| to == *color || self.has_path_to(color, to))
    }

    fn inner_count(&self, color: &'static str) -> usize {
        self.0
            .get(color)
            .unwrap()
            .iter()
            .map(|(n, color)| (1 + self.inner_count(color)) * usize::from(*n))
            .sum()
    }
}

#[test]
fn test() {
    let rules = Rules::from_str(
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.",
    );
    assert_eq!(rules.paths_to("shiny gold"), 4);
    assert_eq!(rules.inner_count("shiny gold"), 32);

    let rules = Rules::from_str(
        "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.",
    );
    assert_eq!(rules.inner_count("shiny gold"), 126);
}

fn main() {
    let rules = Rules::from_str(input!());
    libc_println!("part 1: {}", rules.paths_to("shiny gold"));
    libc_println!("part 2: {}", rules.inner_count("shiny gold"));
}
