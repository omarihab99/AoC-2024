use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let input = read_input();
    println!("{}", solve(input));
}
fn solve(mut input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut val = 0;
            let mut visited = HashSet::new();
            if input[i][j] == 0 {
                recurse(i, j, &mut input, &mut val, &mut visited);
                sum += val;
            }
        }
    }
    sum
}
fn recurse(
    i: usize,
    j: usize,
    input: &mut Vec<Vec<u32>>,
    val: &mut u32,
    visited: &mut HashSet<(usize, usize)>,
) {
    if !visited.contains(&(i, j)) && input[i][j] == 9 {
        *val += 1;
        visited.insert((i, j));
        return;
    }
    if i > 0 && input[i - 1][j] == input[i][j] + 1 {
        recurse(i - 1, j, input, val, visited);
    }
    if i < input.len() - 1 && input[i + 1][j] == input[i][j] + 1 {
        recurse(i + 1, j, input, val, visited);
    }
    if j > 0 && input[i][j - 1] == input[i][j] + 1 {
        recurse(i, j - 1, input, val, visited);
    }
    if j < input[0].len() - 1 && input[i][j + 1] == input[i][j] + 1 {
        recurse(i, j + 1, input, val, visited);
    }
}
fn read_input() -> Vec<Vec<u32>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
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
        assert_eq!(solve(input), 36);
    }
}
