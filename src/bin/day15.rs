use aoc24::grid::Direction;
use aoc24::grid::MutableGrid;

fn main() {
    let input = include_str!("../../inputs/day15.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Input<'a> {
    grid: &'a str,
    moves: Vec<Direction>,
}

fn parse(input: &str) -> Input<'_> {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = grid.trim();
    let moves = moves
        .bytes()
        .filter(|&b| b != b'\n')
        .map(|b| match b {
            b'>' => Direction::Right,
            b'<' => Direction::Left,
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            _ => unreachable!("{b}"),
        })
        .collect();

    Input { grid, moves }
}

fn part1(input: &str) -> u32 {
    fn make_move(grid: &mut MutableGrid<'_>, dir: Direction, y: usize, x: usize) -> (usize, usize) {
        if grid.at(y, x) == Some(b'#') {
            return (y, x);
        }

        let (ny, nx) = match dir {
            Direction::Up => (y - 1, x),
            Direction::Right => (y, x + 1),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
        };

        if grid.at(ny, nx) == Some(b'O') {
            make_move(grid, dir, ny, nx);
        }

        if grid.at(ny, nx) == Some(b'.') {
            grid.swap(ny, nx, y, x);
            (ny, nx)
        } else {
            (y, x)
        }
    }

    let Input { grid, moves } = parse(input);
    let mut grid = grid.as_bytes().to_vec();
    let mut grid = MutableGrid::new(&mut grid);
    let (mut y, mut x, _) = grid.imm().iter().find(|&(.., c)| c == b'@').unwrap();

    for dir in moves {
        let (ny, nx) = make_move(&mut grid, dir, y, x);
        y = ny;
        x = nx;
    }

    grid.imm()
        .iter()
        .filter_map(|(y, x, c)| (c == b'O').then_some(100 * y + x))
        .sum::<usize>()
        .try_into()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    fn attempt_move(
        grid: &mut MutableGrid<'_>,
        y: usize,
        x: usize,
        dir: Direction,
        apply: bool,
    ) -> Option<(usize, usize)> {
        if grid.at(y, x) == Some(b'#') {
            return None;
        }

        let begin_y = y;
        let begin_x = x;

        let (y, x) = match dir {
            Direction::Up => (y - 1, x),
            Direction::Right => (y, x + 1),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
        };

        let result = match (grid.at(y, x), dir) {
            (Some(b'['), Direction::Up | Direction::Down) => {
                attempt_move(grid, y, x, dir, apply).is_some()
                    && attempt_move(grid, y, x + 1, dir, apply).is_some()
            }
            (Some(b']'), Direction::Up | Direction::Down) => {
                attempt_move(grid, y, x, dir, apply).is_some()
                    && attempt_move(grid, y, x - 1, dir, apply).is_some()
            }
            (Some(b'[' | b']'), Direction::Left | Direction::Right) => {
                attempt_move(grid, y, x, dir, apply).is_some()
            }
            (Some(b'@'), _) => attempt_move(grid, y, x, dir, apply).is_some(),
            (Some(b'#'), _) => false,
            (Some(b'.'), _) => true,
            o => unreachable!("{o:?}"),
        };

        if result && apply {
            grid.swap(begin_y, begin_x, y, x);
        }

        result.then_some((y, x))
    }

    let Input { grid, moves } = parse(input);
    let mut buf = Vec::<u8>::with_capacity(grid.len() * 2);
    for b in grid.bytes() {
        match b {
            b'#' => buf.extend(b"##"),
            b'.' => buf.extend(b".."),
            b'@' => buf.extend(b"@."),
            b'O' => buf.extend(b"[]"),
            b'\n' => buf.push(b'\n'),
            _ => unreachable!("{b}"),
        }
    }
    let mut grid = MutableGrid::new(&mut buf);
    let (mut y, mut x, _) = grid.imm().iter().find(|&(.., c)| c == b'@').unwrap();

    for dir in moves {
        if let Some((ny, nx)) = attempt_move(&mut grid, y, x, dir, false) {
            attempt_move(&mut grid, y, x, dir, true);
            y = ny;
            x = nx;
        }
    }

    grid.imm()
        .iter()
        .filter_map(|(y, x, c)| (c == b'[').then_some(100 * y + x))
        .sum::<usize>()
        .try_into()
        .unwrap()
}
