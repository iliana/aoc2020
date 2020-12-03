#![no_std]

use aoc2020::*;
use heapless::{consts::U512, Vec};

struct Map(Vec<&'static [u8], U512>);

impl Map {
    fn from_str(s: &'static str) -> Map {
        Map(s.lines().map(str::as_bytes).collect())
    }

    fn trees_for_path(&self, step_right: usize, step_down: usize) -> usize {
        self.0
            .iter()
            .step_by(step_down)
            .enumerate()
            .filter(|(i, row)| row[(i * step_right) % row.len()] == b'#')
            .count()
    }
}

fn part1(map: &Map) -> usize {
    map.trees_for_path(3, 1)
}

fn part2(map: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(step_right, step_down)| map.trees_for_path(*step_right, *step_down))
        .product()
}

#[test]
fn test() {
    let map = Map::from_str(include_str!(test_input_name!()));
    assert_eq!(part1(&map), 7);
    assert_eq!(part2(&map), 336);
}

fn main() {
    let map = Map::from_str(include_str!(input_name!()));
    libc_println!("part 1: {}", part1(&map));
    libc_println!("part 2: {}", part2(&map));
}
