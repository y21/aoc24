#![feature(iter_collect_into)]
use std::cmp::Ordering;
use std::mem;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day2.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 1: {}", part2(input));
}

fn inputs(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + Clone + '_> + '_ {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|level| level.parse::<u32>().unwrap())
    })
}

fn try_line(line: impl Iterator<Item = u32>) -> bool {
    let mut increasing = None::<bool>;

    line.tuple_windows().all(|(prev, next)| {
        (1..=3).contains(&prev.abs_diff(next))
            && match next.cmp(&prev) {
                Ordering::Greater => {
                    matches!(mem::replace(&mut increasing, Some(true)), None | Some(true))
                }
                Ordering::Less => matches!(
                    mem::replace(&mut increasing, Some(false)),
                    None | Some(false)
                ),
                Ordering::Equal => false,
            }
    })
}

fn part1(input: &str) -> u32 {
    inputs(input)
        .map(try_line)
        .filter(|&v| v)
        .count()
        .try_into()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut line = Vec::new();
    inputs(input)
        .map(|line_iter| {
            line.clear();
            line_iter.collect_into(&mut line);

            (0..line.len())
                .any(|begin| try_line(line[..begin].iter().chain(&line[begin + 1..]).copied()))
        })
        .filter(|&v| v)
        .count()
        .try_into()
        .unwrap()
}
