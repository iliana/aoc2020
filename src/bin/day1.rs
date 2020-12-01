use std::str::FromStr;

fn part1<I: IntoIterator<Item = u64> + Clone>(iter: I) -> u64 {
    for a in iter.clone() {
        for b in iter.clone() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!("no solution found");
}

fn part2<I: IntoIterator<Item = u64> + Clone>(iter: I) -> u64 {
    for a in iter.clone() {
        for b in iter.clone() {
            for c in iter.clone() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!("no solution found");
}

#[test]
fn test() {
    assert_eq!(part1(vec![1721, 979, 366, 299, 675, 1456]), 1721 * 299);
    assert_eq!(part2(vec![1721, 979, 366, 299, 675, 1456]), 979 * 366 * 675);
}

fn main() {
    let iter = include_str!("../../input/day1.txt")
        .lines()
        .map(|l| u64::from_str(l).unwrap());
    println!("part 1: {}", part1(iter.clone()));
    println!("part 2: {}", part2(iter));
}
