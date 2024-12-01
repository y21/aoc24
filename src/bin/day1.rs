use std::iter;

use aoc24::parse_u32;
use rustc_hash::FxBuildHasher;
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("../../inputs/day1.txt");
const FULL_INPUT_DIGITS: usize = {
    // all numbers have the same number of digits,
    // precompute it here and use a custom u32 parser that takes advantage of it
    let mut i = 0;
    let bytes = INPUT.as_bytes();
    while bytes[i].is_ascii_digit() {
        i += 1;
    }
    i
};

fn main() {
    let input = include_str!("../../inputs/day1.txt");

    println!("Part 1: {}", part1::<FULL_INPUT_DIGITS>(input));
    println!("Part 2: {}", part2::<FULL_INPUT_DIGITS>(input));
}

fn inputs<const DIGITS: usize>(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    let mut line = 0;
    let mut pos = 0;

    iter::from_fn(move || {
        if line < 1000 {
            let left = parse_u32::<DIGITS>(input, &mut pos);
            pos += 3;
            let right = parse_u32::<DIGITS>(input, &mut pos);
            pos += 1;
            line += 1;
            Some((left, right))
        } else {
            None
        }
    })
}

fn part1<const DIGITS: usize>(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    inputs::<DIGITS>(input).for_each(|(l, r)| {
        left.push(l);
        right.push(r);
    });

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>()
}

fn part2<const DIGITS: usize>(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = FxHashMap::<u32, (u32, u32)>::with_capacity_and_hasher(1000, FxBuildHasher);

    inputs::<DIGITS>(input).for_each(|(l, r)| {
        left.push(l);
        right.entry(r).or_default().1 += 1;
    });

    for val in left.iter().copied() {
        right.entry(val).or_default().0 += 1;
    }

    right
        .into_iter()
        .fold(0, |p, (digit, (left_count, right_count))| {
            p + digit * left_count * right_count
        })
}
