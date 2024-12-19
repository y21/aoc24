use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../../inputs/day19.txt");
    let (part1, part2) = solve(input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    (patterns.split(", ").collect(), designs.lines().collect())
}

fn count_patterns<'a>(
    seen: &mut FxHashMap<&'a str, u64>,
    patterns: &[&str],
    design: &'a str,
) -> u64 {
    if let Some(&p) = seen.get(design) {
        return p;
    }

    if design.is_empty() {
        return 1;
    }

    let count: u64 = patterns
        .iter()
        .filter_map(|pat| design.strip_prefix(pat))
        .map(|design| count_patterns(seen, patterns, design))
        .sum();

    seen.insert(design, count);

    count
}

fn solve(input: &str) -> (u64, u64) {
    let (patterns, designs) = parse(input);

    designs
        .par_iter()
        .map(|design| count_patterns(&mut FxHashMap::default(), &patterns, design))
        .fold(
            || (0, 0),
            |(part1, part2), c| (part1 + if c > 0 { 1 } else { 0 }, part2 + c),
        )
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}
