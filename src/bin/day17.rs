#![feature(iter_next_chunk)]

use itertools::Itertools;

const ADV: u8 = 0;
const BXL: u8 = 1;
const BSL: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn main() {
    let input = include_str!("../../inputs/day17.txt").trim();
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Input {
    a: u64,
    program: Box<[u8]>,
}

fn parse(input: &str) -> Input {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let [a, _, _] = registers
        .split("\n")
        .map(|reg| reg.split_once(": ").unwrap().1.parse::<u64>().unwrap())
        .next_chunk()
        .unwrap();
    let program = program
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    Input { a, program }
}

#[derive(Debug, Clone)]
struct Executor<'a> {
    program: &'a [u8],
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
}

impl<'a> Executor<'a> {
    fn new(a: u64, program: &'a [u8]) -> Self {
        Self {
            a,
            b: 0,
            c: 0,
            program,
            ip: 0,
        }
    }

    fn next(&mut self) -> u8 {
        let v = self.program[self.ip];
        self.ip += 1;
        v
    }

    fn combo(&mut self) -> u64 {
        self.ip += 1;
        match self.program[self.ip - 1] {
            literal @ 0..=3 => literal as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("invalid combo operand 7!"),
            o => panic!("{o}"),
        }
    }

    fn literal(&mut self) -> u64 {
        let v = self.program[self.ip];
        self.ip += 1;
        v as u64
    }

    fn run(&mut self, buf: &mut Vec<u8>) {
        buf.clear();
        self.b = 0;
        self.c = 0;

        while self.ip < self.program.len() {
            match self.next() {
                ADV => self.a >>= self.combo(),
                BXL => self.b ^= self.literal(),
                BSL => self.b = self.combo() & 0b111,
                JNZ => {
                    let value = self.literal();
                    if self.a != 0 {
                        self.ip = value as usize;
                    }
                }
                BXC => {
                    self.next();
                    self.b ^= self.c;
                }
                OUT => buf.push((self.combo() & 0b111) as u8),
                BDV => self.b = self.a >> self.combo(),
                CDV => self.c = self.a >> self.combo(),
                o => panic!("{o}"),
            }
        }
    }
}

fn part1(input: &str) -> String {
    let input = parse(input);
    let mut out = Vec::with_capacity(16);
    Executor::new(input.a, &input.program).run(&mut out);
    out.iter().join(",")
}

fn part2(input: &str) -> u64 {
    let state = parse(input);
    assert_eq!(state.program.len(), 16, "must have length 16");

    let mut acc = 0;
    let mut buf = Vec::with_capacity(16);
    for i in (1..=16).rev() {
        let step = 2u64.pow(3 * (i - 1));

        loop {
            Executor::new(acc, &state.program).run(&mut buf);
            let digit = buf.get((i - 1) as usize);
            let expected_digit = state.program[(i - 1) as usize];
            if digit == Some(&expected_digit) {
                break;
            }
            acc += step;
        }
    }

    for i in acc.. {
        Executor::new(i, &state.program).run(&mut buf);
        if *buf == *state.program {
            return i;
        }
    }

    panic!("no solution found")
}
