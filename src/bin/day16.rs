#![feature(let_chains)]
use std::collections::VecDeque;

use aoc24::grid::Direction;
use aoc24::grid::Grid;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../../inputs/day16.txt");
    let (part1, part2) = solve(input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn solve(input: &str) -> (u64, usize) {
    let grid = Grid::view(input);
    let (y, x, _) = grid.iter().find(|&(.., c)| c == b'S').unwrap();
    let mut bfs = VecDeque::new();
    bfs.push_back((y, x, Direction::Right, 0u64, Vec::new()));

    let mut min = None;
    let mut min_unique_positions = FxHashSet::default();
    let mut seen = FxHashMap::default();

    while let Some((y, x, dir, score, mut path)) = bfs.pop_front() {
        if min.is_some_and(|min| score > min) {
            continue;
        }

        path.push((y, x));
        if grid.at(y, x) == Some(b'E') {
            if min.is_none_or(|min| score < min) {
                min = Some(score);
                min_unique_positions.clear();
            }

            if min == Some(score) {
                min_unique_positions.extend(path);
            }
            continue;
        }

        if let Some(&dist) = seen.get(&(y, x, dir))
            && dist < score
        {
            continue;
        }
        seen.insert((y, x, dir), score);

        for (ny, nx, c, ndir) in grid.direct_neighbors_with_direction(y, x) {
            if c != b'#' && ndir.opposite() != dir {
                let score = score + if dir == ndir { 1 } else { 1001 };
                bfs.push_back((ny, nx, ndir, score, path.clone()));
            }
        }
    }

    (min.unwrap(), min_unique_positions.len())
}
