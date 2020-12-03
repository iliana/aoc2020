#![no_std]

use aoc2020::*;
use heapless::{consts::*, ArrayLength, Vec};

fn solve<N: ArrayLength<u64>>(data: &[u64]) -> Option<u64> {
    fn inner<N: ArrayLength<u64>>(data: &[u64], acc: &mut Vec<u64, N>) -> Option<u64> {
        for (i, n) in data.iter().enumerate() {
            acc.push(*n).unwrap();
            if acc.len() == acc.capacity() {
                if acc.iter().sum::<u64>() == 2020 {
                    return Some(acc.iter().product());
                }
            } else if let Some(v) = inner(&data[(i + 1)..], acc) {
                return Some(v);
            }
            acc.pop();
        }
        None
    }

    inner::<N>(data, &mut Vec::new())
}

#[test]
fn test() {
    let data: Vec<u64, U200> = input!(u64).collect();
    assert_eq!(solve::<U2>(&data), Some(1721 * 299));
    assert_eq!(solve::<U3>(&data), Some(979 * 366 * 675));
}

fn main() {
    let data: Vec<u64, U200> = input!(u64).collect();
    libc_println!("part 1: {}", solve::<U2>(&data).unwrap());
    libc_println!("part 2: {}", solve::<U3>(&data).unwrap());
}
