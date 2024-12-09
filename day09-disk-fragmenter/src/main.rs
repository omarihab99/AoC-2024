fn main() {
    let input = read_input();
    let disk_map = input.trim();
    let blocks = parse_disk_map(&disk_map);
    let compacted_blocks = compact_files(blocks);
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
            blocks.extend(std::iter::repeat(usize::MAX).take(count)); // Represent free space as usize::MAX
        }
        is_file = !is_file;
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
   
}
