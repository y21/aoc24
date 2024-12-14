#![feature(iter_next_chunk)]

use std::thread;
use std::time::Duration;

fn main() {
    let input = include_str!("../../inputs/day14.txt");
    println!("Part 1: {}", part1(input, 100, 101, 103));
    part2(input, 101, 103);
}

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn parse(input: &str) -> impl Iterator<Item = Robot> + '_ {
    input.lines().map(|line| {
        let [px, py, vx, vy] = line
            .split(" ")
            .flat_map(|v| v[2..].split(','))
            .map(|n| n.parse::<i32>().unwrap())
            .next_chunk()
            .unwrap();

        Robot { px, py, vx, vy }
    })
}

fn part1(input: &str, seconds: i32, w: i32, h: i32) -> u32 {
    let mut quadrants = [0, 0, 0, 0];
    let qw = w / 2;
    let qh = h / 2;
    for Robot { px, py, vx, vy } in parse(input) {
        let px = (px + vx * seconds).rem_euclid(w);
        let py = (py + vy * seconds).rem_euclid(h);
        if px < qw && py < qh {
            quadrants[0] += 1;
        } else if px >= w - qw && py < qh {
            quadrants[1] += 1;
        } else if px < qw && py >= h - qh {
            quadrants[2] += 1;
        } else if px >= w - qw && py >= h - qh {
            quadrants[3] += 1;
        }
    }
    quadrants.into_iter().product()
}
fn part2(input: &str, w: i32, h: i32) {
    let mut robots = parse(input).collect::<Vec<_>>();
    let mut image = vec![0u8; (w * h + h) as usize];
    for second in 1.. {
        image.iter_mut().enumerate().for_each(|(i, v)| {
            *v = if i % (w + 1) as usize == 0 {
                b'\n'
            } else {
                b' '
            }
        });

        for Robot { px, py, vx, vy } in robots.iter_mut() {
            *px = (*px + *vx).rem_euclid(w);
            *py = (*py + *vy).rem_euclid(h);

            image[(*py * w + *px + *py) as usize] = b'#';
        }

        let possibly_christmas_tree = image.windows(8).any(|w| w == b"########");

        if possibly_christmas_tree {
            println!("{}", std::str::from_utf8(&image).unwrap());
            println!("Second {second}");
            thread::sleep(Duration::from_millis(500));
        }
    }
}
