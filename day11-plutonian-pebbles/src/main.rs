use std::collections::HashMap;

fn main() {
    let input = read_input();
    println!("{}", solve(input, true));
}

fn solve(input: Vec<u64>, _2: bool) -> usize {
    let mut counts = input.iter().fold(HashMap::new(), |mut acc, &stone| {
        *acc.entry(stone).or_insert(0) += 1;
        acc
    });
    let num_blinks = if _2 { 75 } else { 25 };
    for _ in 0..num_blinks {
        let mut new_stones = HashMap::new();
        for (&stone, &count) in &counts {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if (stone.to_string().len() % 2) == 0 {
                let digits_count = stone.to_string().len() as u32;
                let divisor = 10_u64.pow(digits_count / 2);
                let left = stone / divisor;
                let right = stone % divisor;
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                let multiplied = stone * 2024;
                *new_stones.entry(multiplied).or_insert(0) += count;
            }
        }
        counts = new_stones;
    }

    counts.values().sum()
}
fn read_input() -> Vec<u64> {
    include_str!("input.txt")
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        println!("{:?}", input);
        assert_ne!(input.len(), 0);
    }
    #[test]
    fn test_part_one() {
        let input = read_input();
        assert_eq!(solve(input, false), 55312);
    }
}
