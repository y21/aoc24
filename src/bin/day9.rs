#![feature(let_chains)]
use std::iter;

use slot::DenseSlot;
use slot::SparseSlot;

fn main() {
    let input = include_str!("../../inputs/day9.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

mod slot {
    #[derive(Copy, Clone, Debug)]
    pub struct DenseSlot(u16);

    impl DenseSlot {
        pub const FREE: Self = Self(u16::MAX);

        pub fn new(v: u16) -> Self {
            assert!(v < u16::MAX);
            Self(v)
        }

        pub fn get(self) -> Option<u16> {
            (self.0 < u16::MAX).then_some(self.0)
        }
    }

    #[derive(Copy, Clone)]
    pub enum SparseSlot {
        Occupied { id: u16, count: u8 },
        Free { count: u8 },
    }
}

fn part1(input: &str) -> u64 {
    let mut disk = Vec::with_capacity(input.len());
    let mut free_indices = Vec::new();

    for (idx, byte) in input.bytes().enumerate() {
        let is_block = idx % 2 == 0;
        let block_id = u16::try_from(idx / 2).expect("id overflow");
        let count = usize::from(byte & 0xf);

        if is_block {
            disk.extend(iter::repeat_n(DenseSlot::new(block_id), count));
        } else {
            free_indices.extend((disk.len()..).take(count));
            disk.extend(iter::repeat_n(DenseSlot::FREE, count));
        }
    }

    let mut free = free_indices.drain(..);
    for pos in (0..disk.len()).rev() {
        if disk[pos].get().is_some() {
            let Some(free) = free.next() else { break };
            if free > pos {
                break;
            }

            disk.swap(pos, free);
        }
    }

    let checksum = disk
        .iter()
        .copied()
        .map_while(DenseSlot::get)
        .enumerate()
        .map(|(pos, id)| pos * usize::from(id))
        .sum::<usize>();

    checksum.try_into().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut disk = Vec::with_capacity(input.len());

    for (idx, byte) in input.bytes().enumerate() {
        let is_block = idx % 2 == 0;
        let block_id = u16::try_from(idx / 2).expect("id overflow");
        let count = byte & 0xf;

        if count == 0 {
            continue;
        }

        if is_block {
            disk.push(SparseSlot::Occupied {
                id: block_id,
                count,
            });
        } else {
            disk.push(SparseSlot::Free { count });
        }
    }

    let mut pos = disk.len() - 1;
    loop {
        if let SparseSlot::Occupied { id, count } = disk[pos]
            && let Some((free_pos, free)) = disk.iter_mut().enumerate().find_map(|(i, s)| match s {
                SparseSlot::Free { count: c } if *c >= count => Some((i, c)),
                _ => None,
            })
            && free_pos < pos
        {
            *free -= count;
            disk[pos] = SparseSlot::Free { count };
            disk.insert(free_pos, SparseSlot::Occupied { id, count });
        } else if pos == 0 {
            break;
        } else {
            pos -= 1;
        }
    }

    let mut checksum = 0;
    let mut norm_pos = 0;
    for slot in disk {
        match slot {
            SparseSlot::Occupied { id, count } => {
                for _ in 0..count {
                    checksum += u64::from(id) * norm_pos;
                    norm_pos += 1;
                }
            }
            SparseSlot::Free { count } => {
                norm_pos += u64::from(count);
            }
        }
    }

    checksum
}
