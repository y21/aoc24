use aoc24::grid::Grid;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../../inputs/day8.txt");

    println!("Part 1: {}", solve(input, Part::One));
    println!("Part 2: {}", solve(input, Part::Two));
}

enum Part {
    One,
    Two,
}

fn solve(input: &str, part: Part) -> u32 {
    let grid = Grid::view(input);

    let mut antennas = FxHashMap::default();
    let mut antinodes = FxHashSet::default();

    for (y, x, c) in grid.iter() {
        if c != b'.' {
            antennas
                .entry(c)
                .or_insert(Vec::new())
                .push((y as isize, x as isize));
        }
    }

    for (_, antennas) in antennas {
        let combinations = antennas.iter().copied().combinations(2);
        for combination in combinations {
            let [(ay, ax), (by, bx)] = combination.try_into().unwrap();
            let dx = ax - bx;
            let dy = ay - by;

            for sign in [1, -1] {
                let mut x = ax;
                let mut y = ay;

                while x >= 0 && x < grid.width() as isize && y >= 0 && y < grid.height() as isize {
                    match part {
                        Part::One => {
                            if ![ax, bx].contains(&x) && ![ay, by].contains(&y) {
                                antinodes.insert((y, x));
                                break;
                            }
                        }
                        Part::Two => drop(antinodes.insert((y, x))),
                    }
                    x += sign * dx;
                    y += sign * dy;
                }
            }
        }
    }

    antinodes.len().try_into().unwrap()
}

#[allow(dead_code)]
fn vis(grid: &Grid<'_>, antinodes: &FxHashSet<(isize, isize)>) {
    for (y, x, c) in grid.iter() {
        if y > 0 && x == 0 {
            println!();
        }

        if antinodes.contains(&(y as isize, x as isize)) {
            print!("#");
        } else {
            print!("{}", c as char);
        }
    }
    println!();
}
