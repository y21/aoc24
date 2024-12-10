use aoc24::grid::Grid;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../../inputs/day10.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

trait Set {
    fn insert(&mut self, y: usize, x: usize) -> bool;
}

impl Set for FxHashSet<(usize, usize)> {
    fn insert(&mut self, y: usize, x: usize) -> bool {
        self.insert((y, x))
    }
}

struct SinkSet;
impl Set for SinkSet {
    fn insert(&mut self, _: usize, _: usize) -> bool {
        true
    }
}

fn reachable_nines<S: Set>(grid: &Grid<'_>, seen: &mut S, y: usize, x: usize, c: u8) -> u32 {
    if c == b'9' {
        if seen.insert(y, x) {
            1
        } else {
            0
        }
    } else {
        let mut count = 0;
        for (ny, nx, nc) in grid.direct_neighbors(y, x) {
            if nc.is_ascii_digit() && nc == c + 1 {
                count += reachable_nines(grid, seen, ny, nx, nc);
            }
        }
        count
    }
}

fn part1(input: &str) -> u32 {
    let grid = Grid::view(input);

    let mut seen = FxHashSet::default();
    let mut sum = 0;
    for (y, x, c) in grid.iter() {
        if c == b'0' {
            sum += reachable_nines(&grid, &mut seen, y, x, c);
            seen.clear();
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let grid = Grid::view(input);

    let mut sum = 0;
    for (y, x, c) in grid.iter() {
        if c == b'0' {
            sum += reachable_nines(&grid, &mut SinkSet, y, x, c);
        }
    }

    sum
}
