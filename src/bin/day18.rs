#![feature(iter_next_chunk)]
use std::cmp::max;
use std::collections::VecDeque;

use aoc24::grid::empty_grid;
use aoc24::grid::Grid;
use aoc24::grid::MutableGrid;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../../inputs/day18.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn visited_before(visited: &FxHashMap<(usize, usize), u32>, y: usize, x: usize, dist: u32) -> bool {
    visited.get(&(y, x)).is_some_and(|&d| dist >= d)
}

fn parse(input: &str) -> impl Iterator<Item = [usize; 2]> + '_ {
    input.lines().map(|line| {
        line.split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .next_chunk()
            .unwrap()
    })
}

fn find_path(grid: &Grid<'_>, w: usize, h: usize) -> Option<u32> {
    let mut bfs = VecDeque::new();
    bfs.push_back((0, 0, 0));

    let mut visited = FxHashMap::default();
    while let Some((y, x, dist)) = bfs.pop_front() {
        if y == h - 1 && x == w - 1 {
            return Some(dist);
        }

        if visited_before(&visited, y, x, dist) {
            continue;
        }
        visited.insert((y, x), dist);

        for (ny, nx, c) in grid.direct_neighbors(y, x) {
            if c != b'#' {
                bfs.push_back((ny, nx, dist + 1));
            }
        }
    }

    None
}

fn part1(input: &str) -> u32 {
    let (w, h, coords) =
        parse(input)
            .take(1024)
            .fold((0, 0, Vec::new()), |(max_x, max_y, mut coords), [x, y]| {
                coords.push((y, x));
                (max(x, max_x), max(y, max_y), coords)
            });
    let w = w + 1;
    let h = h + 1;

    let mut buf = empty_grid(w, h, b'.');
    let mut grid = MutableGrid::new(&mut buf);
    for (y, x) in coords {
        grid.set(y, x, b'#');
    }

    find_path(&grid.imm(), w, h).unwrap()
}

fn part2(input: &str) -> String {
    let (w, h, coords) =
        parse(input).fold((0, 0, Vec::new()), |(max_x, max_y, mut coords), [x, y]| {
            coords.push((y, x));
            (max(x, max_x), max(y, max_y), coords)
        });
    let w = w + 1;
    let h = h + 1;

    let mut buf = empty_grid(w, h, b'.');
    let mut grid = MutableGrid::new(&mut buf);
    for &(y, x) in coords[..=1024].iter() {
        grid.set(y, x, b'#');
    }

    for &(y, x) in coords[1024 + 1..].iter() {
        grid.set(y, x, b'#');
        if find_path(&grid.imm(), w, h).is_none() {
            return format!("{x},{y}");
        }
    }

    panic!("no solution found")
}
