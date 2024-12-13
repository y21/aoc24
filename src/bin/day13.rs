#![feature(iter_next_chunk)]

fn main() {
    let input = include_str!("../../inputs/day13.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Copy, Clone)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn parse(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.split("\n\n").map(|machine| {
        let parse_line = |input: &str| {
            let (x, y) = input.split_once(": ").unwrap().1.split_once(", ").unwrap();
            (x[2..].parse().unwrap(), y[2..].parse().unwrap())
        };

        let mut lines = machine.lines();
        let (ax, ay) = parse_line(lines.next().unwrap());
        let (bx, by) = parse_line(lines.next().unwrap());
        let (px, py) = parse_line(lines.next().unwrap());
        Machine {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        }
    })
}

fn solve(m: Machine) -> Option<(i64, i64)> {
    // Equation 1: ax*A + bx*B = tx
    // Equation 2: ay*A + by*B = ty
    let cm = m.ax * m.ay;
    let e1mul = cm / m.ax;
    let e1bx = m.bx * e1mul;
    let e1r = m.px * e1mul;

    let e2mul = cm / m.ay;
    let e2by = m.by * e2mul;
    let e2r = m.py * e2mul;

    let e3r = e2r - e1r;
    let e3b = e2by - e1bx;

    let res_b = e3r / e3b;
    let res_a = (m.px - m.bx * res_b) / m.ax;

    if m.ax * res_a + m.bx * res_b == m.px && m.ay * res_a + m.by * res_b == m.py {
        Some((res_a, res_b))
    } else {
        None
    }
}

fn part1(input: &str) -> i64 {
    parse(input).filter_map(solve).map(|(a, b)| a * 3 + b).sum()
}

fn part2(input: &str) -> i64 {
    parse(input)
        .map(|m| Machine {
            px: m.px + 10000000000000,
            py: m.py + 10000000000000,
            ..m
        })
        .filter_map(solve)
        .map(|(a, b)| a * 3 + b)
        .sum()
}
