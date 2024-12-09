use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input();
    let disk_map = input.trim();
    let blocks = parse_disk_map(&disk_map);
    let compacted_blocks = compact_files_two(blocks); // compact_files(blocks) for part one
    let checksum = calculate_checksum(&compacted_blocks);
    println!("Checksum: {}", checksum);
}

fn parse_disk_map(disk_map: &str) -> Vec<usize> {
    let mut blocks = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    for c in disk_map.chars() {
        let count = c.to_digit(10).unwrap() as usize;
        if is_file {
            blocks.extend(std::iter::repeat(file_id).take(count));
            file_id += 1;
        } else {
            blocks.extend(std::iter::repeat(usize::MAX).take(count));
        }
        is_file = !is_file;
    }
    blocks
}
fn compact_files_two(mut blocks: Vec<usize>) -> Vec<usize> {
    let file_ids = blocks.iter().filter(|&&block| block != usize::MAX).fold(
        HashMap::new(),
        |mut acc, &block| {
            *acc.entry(block).or_insert(0) += 1;
            acc
        },
    );
    let mut ids = file_ids.keys().collect::<Vec<_>>();
    ids.sort();
    for file_id in ids.iter().rev() {
        let file_blocks: Vec<usize> = blocks
            .iter()
            .enumerate()
            .filter_map(|(i, &block)| if block == **file_id { Some(i) } else { None })
            .collect();
        let file_length = file_ids.get(&file_id).unwrap();
        if *file_length == 0 {
            continue;
        }
        let mut leftmost_span = None;
        let mut free_count = 0;
        let mut span_start = 0;

        for (i, &block) in blocks.iter().enumerate() {
            if block == usize::MAX {
                if free_count == 0 {
                    span_start = i;
                }
                free_count += 1;

                if free_count == *file_length {
                    leftmost_span = Some(span_start);
                    break;
                }
            } else {
                free_count = 0;
            }
        }

        if let Some(span_start) = leftmost_span {
            if span_start < file_blocks[0] {
                for &index in &file_blocks {
                    blocks[index] = usize::MAX;
                }

                for i in 0..*file_length {
                    blocks[span_start + i] = **file_id;
                }
            }
        }
    }

    blocks
}
fn compact_files(mut blocks: Vec<usize>) -> Vec<usize> {
    let mut i = blocks.len() - 1;
    for j in 0..blocks.len() {
        if blocks[j] == usize::MAX {
            while i > j && blocks[i] == usize::MAX {
                i -= 1;
            }
            if i > j {
                blocks[j] = blocks[i];
                blocks[i] = usize::MAX;
                i -= 1;
            }
        }
    }
    blocks
}
fn calculate_checksum(blocks: &[usize]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &id)| {
            if id != usize::MAX {
                Some((id as u64) * (i as u64))
            } else {
                None
            }
        })
        .sum()
}

fn read_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        println!("{:?}", input);
        assert_ne!(input.len(), 0);
    }
    #[test]
    fn test_solve_part_one() {
        let input = read_input();
        let blocks = parse_disk_map(&input);
        let compacted_blocks = compact_files(blocks);
        let checksum = calculate_checksum(&compacted_blocks);
        assert_eq!(checksum, 1928);
    }
    #[test]
    fn test_solve_part_two() {
        let input = read_input();
        let blocks = parse_disk_map(&input);
        let compacted_blocks = compact_files_two(blocks);
        let checksum = calculate_checksum(&compacted_blocks);
        assert_eq!(checksum, 2858);
    }
}
