fn main() {
    let input = include_str!("../../inputs/day4.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let line_len = input.iter().position(|&x| x == b'\n').unwrap();
    let lines = input.len() / line_len;

    let at = |y: usize, x: usize| input.get((y * line_len) + x + y).copied().unwrap_or(0);

    let mut count = 0;

    for y in 0..lines as isize {
        for x in 0..line_len as isize {
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
                    .filter_map(|(y, x)| usize::try_from(x).ok().zip(usize::try_from(y).ok()))
                    .map(|(y, x)| at(x, y))
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
    let input = input.as_bytes();
    let line_len = input.iter().position(|&x| x == b'\n').unwrap();
    let lines = input.len() / line_len;

    let at = |y: usize, x: usize| input.get((y * line_len) + x + y).copied().unwrap_or(0);

    let mut count = 0;

    for y in 0..lines as isize {
        for x in 0..line_len as isize {
            if at(y as usize, x as usize) == b'A' {
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
                        .filter_map(|(y, x)| usize::try_from(x).ok().zip(usize::try_from(y).ok()))
                        .map(|(y, x)| at(x, y))
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
