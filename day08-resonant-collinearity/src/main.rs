use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input();
    println!("{}", solve(input.clone()));
}
fn solve(input: Vec<Vec<char>>) -> u32 {
    let rows = input.len();
    let cols = input[0].len();
    let mut count = 0;
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let mut frequencies: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i][j];
            if c != '.' {
                frequencies
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((i as isize, j as isize));
            }
        }
    }
    for (_, positions) in frequencies {
        if positions.len() > 1 {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let antinode1 = (x1 - dx, y1 - dy);
                    let antinode2 = (x2 + dx, y2 + dy);
                    if antinode1.0 >= 0
                        && antinode1.0 < rows as isize
                        && antinode1.1 >= 0
                        && antinode1.1 < cols as isize
                        && antinodes.insert(antinode1)
                    {
                        count += 1;
                    }
                    if antinode2.0 >= 0
                        && antinode2.0 < rows as isize
                        && antinode2.1 >= 0
                        && antinode2.1 < cols as isize
                        && antinodes.insert(antinode2)
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
fn read_input() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");
    input.lines().map(|line| line.chars().collect()).collect()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        assert_ne!(input.len(), 0);
    }
    #[test]
    fn test_solve() {
        let input = read_input();
        assert_eq!(solve(input), 14);
    }
}
