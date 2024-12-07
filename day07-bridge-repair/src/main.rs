fn main() {
    let input = read_input();
    println!("{}", solve(input));
}

fn solve(input: Vec<Vec<u128>>) -> u128 {
    input
        .iter()
        .filter_map(|line| {
            if line.len() < 2 {
                return None;
            }
            let target = line[0];
            let numbers = &line[1..];
            if backtrack(&numbers.to_vec(), numbers[0], 1, target) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn backtrack(input: &Vec<u128>, first_operand: u128, index: usize, result: u128) -> bool {
    if index == input.len() {
        return first_operand == result;
    }
    backtrack(input, first_operand + input[index], index + 1, result)
        || backtrack(input, first_operand * input[index], index + 1, result)
}

fn read_input() -> Vec<Vec<u128>> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            let line = line.replace(":", "");
            line.split_whitespace()
                .filter_map(|n| n.parse::<u128>().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = read_input();
        println!("{:?}", input);
        assert_ne!(input.len(), 0);
    }
}
