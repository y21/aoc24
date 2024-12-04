use aoc24::grid::Grid;

fn main() {
    let input = include_str!("../../inputs/day4.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let grid = Grid::view(input);
    let mut count = 0;

    for y in 0..grid.height() as isize {
        for x in 0..grid.width() as isize {
            let directions = [
                [(y, x), (y, x + 1), (y, x + 2), (y, x + 3)], // right
                [(y, x), (y + 1, x + 1), (y + 2, x + 2), (y + 3, x + 3)], // down-right
                [(y, x), (y + 1, x), (y + 2, x), (y + 3, x)], // down
                [(y, x), (y + 1, x - 1), (y + 2, x - 2), (y + 3, x - 3)], // down-left
                [(y, x), (y, x - 1), (y, x - 2), (y, x - 3)], // left
                [(y, x), (y - 1, x - 1), (y - 2, x - 2), (y - 3, x - 3)], // up-left
                [(y, x), (y - 1, x), (y - 2, x), (y - 3, x)], // up
                [(y, x), (y - 1, x + 1), (y - 2, x + 2), (y - 3, x + 3)], // up-right
            ];

            for direction in directions {
                if direction
                    .iter()
                    .copied()
                    .filter_map(|(y, x)| {
                        usize::try_from(x)
                            .ok()
                            .zip(usize::try_from(y).ok())
                            .and_then(|(y, x)| grid.at(y, x))
                    })
                    .eq(*b"XMAS")
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> u32 {
    let grid = Grid::view(input);
    let mut count = 0;

    for y in 0..grid.height() as isize {
        for x in 0..grid.width() as isize {
            if grid.at(y as usize, x as usize) == Some(b'A') {
                let directions = [
                    [(y - 1, x - 1), (y + 1, x + 1)], // top left to bottom right
                    [(y + 1, x - 1), (y - 1, x + 1)], // bottom left to top right
                    [(y - 1, x + 1), (y + 1, x - 1)], // top right to bottom left
                    [(y + 1, x + 1), (y - 1, x - 1)], // bottom right to top left
                ];

                let mut local = 0;
                for direction in directions {
                    if direction
                        .iter()
                        .copied()
                        .filter_map(|(y, x)| {
                            usize::try_from(y)
                                .ok()
                                .zip(usize::try_from(x).ok())
                                .and_then(|(y, x)| grid.at(y, x))
                        })
                        .eq(*b"MS")
                    {
                        local += 1;
                    }
                }
                if local >= 2 {
                    count += 1;
                }
            }
        }
    }

    count
}
