use std::cmp::max;
use std::cmp::min;

use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn iter(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

fn part1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = iter(input).unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| max(l, r) - min(l, r))
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let (left, mut right) = iter(input).fold(
        (Vec::<u32>::new(), FxHashMap::<u32, (u32, u32)>::default()),
        |(mut left_vec, mut right_map), (left, right)| {
            left_vec.push(left);
            right_map.entry(right).or_default().1 += 1;
            (left_vec, right_map)
        },
    );

    for n in left.iter().copied() {
        right.entry(n).or_default().0 += 1;
    }

    right
        .into_iter()
        .fold(0, |p, (digit, (left_count, right_count))| {
            p + (digit * left_count * right_count)
        })
}
