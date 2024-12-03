use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day3.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(input)
        .map(|m| m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let regex = Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for m in regex.captures_iter(input) {
        match &m[0] {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    sum += m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap()
                }
            }
        }
    }

    sum
}
