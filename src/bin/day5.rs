#![feature(let_chains)]
use itertools::Itertools;
use typed_arena::Arena;

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse<'a>(input: &str, arena: &'a Arena<u32>) -> (Vec<(u32, u32)>, Vec<&'a [u32]>) {
    let mut lines_iter = input.split("\n\n");

    let orderings = lines_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (a, b) = line.split('|').collect_tuple().unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let updates = lines_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| &*arena.alloc_extend(line.split(',').map(|n| n.parse::<u32>().unwrap())))
        .collect::<Vec<_>>();

    (orderings, updates)
}

fn is_update_valid(update: &[u32], orderings: &[(u32, u32)]) -> bool {
    for &(first, second) in orderings {
        let mut pos1 = None;
        let mut pos2 = None;

        for (i, &num) in update.iter().enumerate() {
            if num == first && pos1.is_none() {
                pos1 = Some(i);
            } else if num == second && pos2.is_none() {
                pos2 = Some(i);
            }
        }

        if pos1.zip(pos2).is_some_and(|(a, b)| a > b) {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> u32 {
    let arena = Arena::new();
    let (orderings, updates) = parse(input, &arena);

    updates
        .into_iter()
        .filter(|update| is_update_valid(update, &orderings))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let arena = Arena::new();
    let (orderings, updates) = parse(input, &arena);

    let mut sum = 0;
    for update in updates {
        if !is_update_valid(update, &orderings) {
            let mut update_copy = update.to_vec();

            let mut needs_reorder = true;
            while needs_reorder {
                needs_reorder = false;

                for &num in update {
                    if let Some(pos) = update_copy.iter().position(|&c| c == num) {
                        // lowest index we could shift the element to while still being valid
                        let mut min_index = None::<usize>;
                        // highest index we could shift the element to while still being valid
                        let mut max_index = None::<usize>;

                        for &(first, second) in &orderings {
                            if first == num {
                                if let Some(other_pos) =
                                    update_copy.iter().position(|&c| c == second)
                                {
                                    if max_index.is_none_or(|i| i >= other_pos) {
                                        max_index = Some(other_pos);
                                    }
                                }
                            } else if second == num {
                                if let Some(other_pos) =
                                    update_copy.iter().position(|&c| c == first)
                                {
                                    if min_index.is_none_or(|i| i <= other_pos) {
                                        min_index = Some(other_pos);
                                    }
                                }
                            }
                        }

                        if let Some(min) = min_index
                            && min > pos
                        {
                            if max_index.is_some_and(|max| max < min) {
                                needs_reorder = true;
                            } else {
                                update_copy.remove(pos);
                                update_copy.insert(min, num);
                            }
                        } else if let Some(max) = max_index
                            && max < pos
                        {
                            if min_index.is_some_and(|min| min > max) {
                                needs_reorder = true;
                            } else {
                                update_copy.remove(pos);
                                update_copy.insert(max, num);
                            }
                        }
                    }
                }
            }

            let middle_number = update_copy[update_copy.len() / 2];
            sum += middle_number;
        }
    }

    sum
}
