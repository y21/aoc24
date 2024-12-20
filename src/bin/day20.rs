#![feature(let_chains, array_windows, cmp_minmax)]

use std::cmp::minmax;

use aoc24::grid::Coord;
use aoc24::grid::Direction;
use aoc24::grid::Grid;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

fn main() {
    let input = include_str!("../../inputs/day20.txt").trim();
    let (part1, part2) = solve(input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn populate_dist_map_from(
    map: &mut FxHashMap<(usize, usize), u32>,
    grid: &Grid<'_>,
    y: usize,
    x: usize,
    dir: Direction,
) -> u32 {
    if grid.at(y, x) == Some(b'E') {
        map.insert((y, x), 0);
        return 0;
    }

    let (ny, nx, _, nd) = grid
        .direct_neighbors_with_direction(y, x)
        .find(|&(.., c, d)| c != b'#' && d != dir.opposite())
        .unwrap();
    let dist = populate_dist_map_from(map, grid, ny, nx, nd) + 1;
    map.insert((y, x), dist);
    dist
}

fn solve(input: &str) -> (u32, u32) {
    const PART1_CHEATS: usize = 2;
    const PART2_CHEATS: usize = 20;

    let grid = Grid::view(input);
    let (start_y, start_x, _) = grid.iter().find(|&(.., c)| c == b'S').unwrap();
    let (.., start_dir) = grid
        .direct_neighbors_with_direction(start_y, start_x)
        .find(|&(_, _, c, _)| c == b'.')
        .unwrap();

    let mut dist_map = FxHashMap::default();
    populate_dist_map_from(&mut dist_map, &grid, start_y, start_x, start_dir);

    let mut cheats_used = FxHashSet::default();
    let mut y = start_y;
    let mut x = start_x;
    let mut dir = start_dir;

    let mut part1 = 0;
    let mut part2 = 0;

    // No bfs necessary as we have one clear path.
    while grid.at(y, x) != Some(b'E') {
        let normal_dist = dist_map[&(y, x)];
        let (ny, nx, _, nd) = grid
            .direct_neighbors_with_direction(y, x)
            .find(|&(.., c, d)| c != b'#' && d != dir.opposite())
            .unwrap();

        for cheat in 1..=PART2_CHEATS {
            let edges = {
                let y = y as isize;
                let x = x as isize;
                let cheat = cheat as isize;
                [
                    (y - cheat, x - cheat),
                    (y - cheat, x + cheat),
                    (y + cheat, x + cheat),
                    (y + cheat, x - cheat),
                    (y - cheat, x - cheat),
                ]
            };

            for &[(fy, fx), (ty, tx)] in edges.array_windows() {
                let [fy, ty] = minmax(fy, ty);
                let [fx, tx] = minmax(fx, tx);

                for ny in fy..=ty {
                    for nx in fx..=tx {
                        if let Ok(ny) = usize::try_from(ny)
                            && let Ok(nx) = usize::try_from(nx)
                            && grid.at(ny, nx).is_some_and(|v| v != b'#')
                        {
                            let start_coord = Coord { y, x };
                            let end_coord = Coord { y: ny, x: nx };
                            if cheats_used.contains(&(start_coord, end_coord)) {
                                continue;
                            }
                            let extra_steps = y.abs_diff(ny) + x.abs_diff(nx);
                            if extra_steps > PART2_CHEATS {
                                continue;
                            }
                            let cheat_dist = dist_map[&(ny, nx)] + extra_steps as u32;
                            if cheat_dist < normal_dist {
                                let saved = normal_dist - cheat_dist;
                                cheats_used.insert((start_coord, end_coord));

                                if saved >= 100 {
                                    if extra_steps <= PART1_CHEATS {
                                        part1 += 1;
                                    }
                                    part2 += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        y = ny;
        x = nx;
        dir = nd;
    }
    (part1, part2)
}
