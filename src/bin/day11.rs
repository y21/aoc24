use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../../inputs/day11.txt");
    println!("Part 1: {}", solve(input, 25));
    println!("Part 2: {}", solve(input, 75));
}

fn blink(seen: &mut FxHashMap<(u64, u32), u64>, num: u64, step: u32, target_steps: u32) -> u64 {
    fn inner(seen: &mut FxHashMap<(u64, u32), u64>, num: u64, step: u32, target_steps: u32) -> u64 {
        if step == target_steps {
            return 1;
        }

        if num == 0 {
            return blink(seen, 1, step + 1, target_steps);
        }

        let digits = num.ilog10() + 1;
        if digits % 2 == 0 {
            let half = digits / 2;
            let left = num / 10u64.pow(half);
            let right = num % 10u64.pow(half);
            blink(seen, left, step + 1, target_steps) + blink(seen, right, step + 1, target_steps)
        } else {
            blink(seen, num * 2024, step + 1, target_steps)
        }
    }

    if let Some(v) = seen.get(&(num, step)) {
        *v
    } else {
        let v = inner(seen, num, step, target_steps);
        seen.insert((num, step), v);
        v
    }
}

fn solve(input: &str, blinks: u32) -> u64 {
    let seen = &mut FxHashMap::default();
    let mut count = 0;
    for num in input.split_ascii_whitespace() {
        let num = num.parse::<u64>().unwrap();
        count += blink(seen, num, 0, blinks);
    }
    count
}
