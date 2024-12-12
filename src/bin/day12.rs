#![feature(let_chains)]
use std::collections::VecDeque;

use aoc24::grid::Coord;
use aoc24::grid::Grid;

fn main() {
    let input = include_str!("../../inputs/day12.txt");
    let (part1, part2) = solve(input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

struct Path {
    area: u32,
    perimeter: u32,
    turns: u32,
}

struct QueueItem {
    coord: Coord,
    path: usize,
}

fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::view(input);
    let mut paths = vec![Path {
        area: 0,
        perimeter: 0,
        turns: 0,
    }];
    let mut queue = VecDeque::from_iter([QueueItem {
        coord: Coord { y: 0, x: 0 },
        path: 0,
    }]);
    let mut visited = vec![false; grid.width() * grid.height()];

    while let Some(QueueItem { coord, path }) = queue.pop_front() {
        if visited[coord.y * grid.height() + coord.x] {
            continue;
        }

        visited[coord.y * grid.height() + coord.x] = true;
        paths[path].area += 1;

        let element = grid[coord];

        let y = coord.y as isize;
        let x = coord.x as isize;
        let patterns = [
            // ┌─ -- top and left empty
            &[(y - 1, x, false), (y, x - 1, false)] as &[_],
            // └ -- bottom and left empty
            &[(y + 1, x, false), (y, x - 1, false)],
            // ─┘ -- bottom and right empty
            &[(y + 1, x, false), (y, x + 1, false)],
            // ┐ -- top and right empty
            &[(y - 1, x, false), (y, x + 1, false)],
            // ┌─ -- bottom and right filled, diagonal right bottom empty
            &[(y + 1, x, true), (y, x + 1, true), (y + 1, x + 1, false)],
            // └ -- top and right filled, diagonal right top empty
            &[(y - 1, x, true), (y, x + 1, true), (y - 1, x + 1, false)],
            // ─┘ -- top and left filled, diagonal left top empty
            &[(y - 1, x, true), (y, x - 1, true), (y - 1, x - 1, false)],
            // ┐ -- bottom and left filled, bottom left empty
            &[(y + 1, x, true), (y, x - 1, true), (y + 1, x - 1, false)],
        ];
        for pattern in patterns {
            let works = pattern.iter().all(|&(py, px, fill)| {
                if let Ok(py) = usize::try_from(py)
                    && let Ok(px) = usize::try_from(px)
                {
                    if fill {
                        grid.at(py, px) == Some(element)
                    } else {
                        grid.at(py, px).is_none_or(|c| c != element)
                    }
                } else {
                    !fill
                }
            });

            if works {
                paths[path].turns += 1;
            }
        }

        let neighbors = grid.direct_neighbors_opt(coord.y, coord.x);
        for neighbor in neighbors {
            match neighbor {
                Some((oy, ox, oc)) => {
                    if oc == element {
                        queue.push_front(QueueItem {
                            coord: Coord { x: ox, y: oy },
                            path,
                        });
                    } else {
                        paths[path].perimeter += 1;
                        if !visited[oy * grid.height() + ox] {
                            let new_path_id = paths.len();
                            paths.push(Path {
                                area: 0,
                                perimeter: 0,
                                turns: 0,
                            });
                            queue.push_back(QueueItem {
                                coord: Coord { x: ox, y: oy },
                                path: new_path_id,
                            });
                        }
                    }
                }
                None => {
                    paths[path].perimeter += 1;
                }
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for path in paths {
        if path.area > 0 || path.perimeter > 0 {
            part1 += path.area * path.perimeter;
            part2 += path.area * path.turns;
        }
    }
    (part1, part2)
}

#[allow(dead_code)]
fn vis(grid: &Grid, seen: &[bool]) {
    for (y, x, c) in grid.iter() {
        if y > 0 && x == 0 {
            println!();
        }
        if seen[y * grid.height() + x] {
            print!("\x1b[0;33m{}\x1b[0m", c as char);
        } else {
            print!("\x1b[0;34m{}\x1b[0m", c as char);
        }
    }
    println!();
}
