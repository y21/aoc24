use aoc24::grid::Grid;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../../inputs/day6.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Part1Res {
    distinct_coords: FxHashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

fn part1_parts(input: &str) -> Part1Res {
    let grid = Grid::view(input);
    let mut guard_x = None;
    let mut guard_y = None;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.at(y, x) == Some(b'^') {
                guard_x = Some(x);
                guard_y = Some(y);
            }
        }
    }

    let mut guard_x = guard_x.unwrap();
    let mut guard_y = guard_y.unwrap();
    let start_x = guard_x;
    let start_y = guard_y;
    let mut direction = Direction::Up;

    let mut distinct_coords = FxHashSet::from_iter([(start_y, start_x)]);

    'outer: loop {
        match direction {
            Direction::Up => loop {
                if guard_y == 0 {
                    break 'outer;
                }
                if grid.at(guard_y - 1, guard_x) == Some(b'#') {
                    direction = Direction::Right;
                    break;
                }
                guard_y -= 1;
                distinct_coords.insert((guard_y, guard_x));
            },
            Direction::Right => loop {
                if guard_x == grid.width() - 1 {
                    break 'outer;
                }
                if grid.at(guard_y, guard_x + 1) == Some(b'#') {
                    direction = Direction::Down;
                    break;
                }
                guard_x += 1;
                distinct_coords.insert((guard_y, guard_x));
            },
            Direction::Left => loop {
                if guard_x == 0 {
                    break 'outer;
                }
                if grid.at(guard_y, guard_x - 1) == Some(b'#') {
                    direction = Direction::Up;
                    break;
                }
                guard_x -= 1;
                distinct_coords.insert((guard_y, guard_x));
            },
            Direction::Down => loop {
                if guard_y == grid.height() - 1 {
                    break 'outer;
                }
                if grid.at(guard_y + 1, guard_x) == Some(b'#') {
                    direction = Direction::Left;
                    break;
                }
                guard_y += 1;
                distinct_coords.insert((guard_y, guard_x));
            },
        }
    }

    Part1Res {
        start_x,
        start_y,
        distinct_coords,
    }
}

fn part1(input: &str) -> u32 {
    part1_parts(input).distinct_coords.len().try_into().unwrap()
}

fn part2(input: &str) -> u32 {
    fn is_loop(grid: &Grid<'_>, start_y: usize, start_x: usize, co_y: usize, co_x: usize) -> bool {
        let is_construction = |y, x| (y == co_y && x == co_x) || grid.at(y, x) == Some(b'#');
        let mut guard_y = start_y;
        let mut guard_x = start_x;
        let mut direction = Direction::Up;
        let mut seen = FxHashSet::default();
        'outer: loop {
            match direction {
                Direction::Up => loop {
                    if guard_y == 0 {
                        break 'outer;
                    }
                    if is_construction(guard_y - 1, guard_x) {
                        if !seen.insert((guard_y, guard_x, direction)) {
                            return true;
                        }
                        direction = Direction::Right;
                        break;
                    }
                    guard_y -= 1;
                },
                Direction::Right => loop {
                    if guard_x == grid.width() - 1 {
                        break 'outer;
                    }
                    if is_construction(guard_y, guard_x + 1) {
                        if !seen.insert((guard_y, guard_x, direction)) {
                            return true;
                        }
                        direction = Direction::Down;
                        break;
                    }
                    guard_x += 1;
                },
                Direction::Left => loop {
                    if guard_x == 0 {
                        break 'outer;
                    }
                    if is_construction(guard_y, guard_x - 1) {
                        if !seen.insert((guard_y, guard_x, direction)) {
                            return true;
                        }
                        direction = Direction::Up;
                        break;
                    }
                    guard_x -= 1;
                },
                Direction::Down => loop {
                    if guard_y == grid.height() - 1 {
                        break 'outer;
                    }
                    if is_construction(guard_y + 1, guard_x) {
                        if !seen.insert((guard_y, guard_x, direction)) {
                            return true;
                        }
                        direction = Direction::Left;
                        break;
                    }
                    guard_y += 1;
                },
            }
        }

        false
    }

    let Part1Res {
        distinct_coords,
        start_x,
        start_y,
    } = part1_parts(input);

    let grid = Grid::view(input);
    distinct_coords
        .into_iter()
        .filter(|&(y, x)| is_loop(&grid, start_y, start_x, y, x))
        .count()
        .try_into()
        .unwrap()
}
