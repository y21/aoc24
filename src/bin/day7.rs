use typed_arena::Arena;

fn main() {
    let input = include_str!("../../inputs/day7.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let arena = Arena::new();

    let mut sum = 0;

    for line in input.lines() {
        let (target, nums) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = &*arena.alloc_extend(nums.split(' ').map(|n| n.parse::<u64>().unwrap()));

        let mut dfs = vec![(nums[0], &nums[1..])];

        while let Some((left, rest)) = dfs.pop() {
            if left == target {
                sum += target;
                break;
            }
            if let [cur, ref rest @ ..] = *rest {
                dfs.push((left + cur, rest));
                dfs.push((left * cur, rest));
            }
        }
    }

    sum
}

fn combine_digits(mut left: u64, mut right: u64) -> u64 {
    let right_digit_count = right.ilog10() + 1;
    left *= 10u64.pow(right_digit_count);

    for i in (0..right_digit_count).rev() {
        let pow = 10u64.pow(i);
        let digit = right / pow;
        left += digit * pow;
        right %= pow;
    }

    left
}

fn part2(input: &str) -> u64 {
    let arena = Arena::new();

    let mut sum = 0;

    for line in input.lines() {
        let (target, nums) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = &*arena.alloc_extend(nums.split(' ').map(|n| n.parse::<u64>().unwrap()));

        let mut dfs = vec![(nums[0], &nums[1..])];

        while let Some((left, rest)) = dfs.pop() {
            if left == target {
                sum += target;
                break;
            }
            if let [cur, ref rest @ ..] = *rest {
                dfs.push((left + cur, rest));
                dfs.push((left * cur, rest));
                dfs.push((combine_digits(left, cur), rest));
            }
        }
    }

    sum
}

#[test]
fn combine_digits_test() {
    assert_eq!(combine_digits(123, 4567), 1234567);
    assert_eq!(combine_digits(12345, 4567), 123454567);
}
