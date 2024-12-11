fn main() {
    let mut input = read_input();
    println!("{}", solve(input));
}

fn solve(mut input: Vec<u64>) -> usize {
    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in input.iter() {
            if *stone == 0 {
                new_stones.push(1);
            } else if ((*stone as f64).log10() as u64 + 1) & 1 == 0 {
                let digits_count = ((*stone as f64).log10() as u64 + 1) as u32;
                let divisor = 10_u64.pow(digits_count / 2 );
                new_stones.push(*stone / divisor);
                new_stones.push(*stone % divisor);
            } else {
                new_stones.push(*stone * 2024);
            }
        }
        input = new_stones;
    }
    input.len()
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
        assert_eq!(solve(input), 55312);
    }
}
