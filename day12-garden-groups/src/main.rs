use std::collections::HashSet;

fn main() {
    let input = read_input();
    println!("{}", solve(input));
}
fn solve(input: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    let rows = input.len();
    let cols = input[0].len();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited = HashSet::new();
    let is_valid = |r: usize, c: usize| r < rows && c < cols;
    let explore_region = |r: usize, c: usize, visited: &mut HashSet<(usize, usize)>| {
        let region_type = input[r][c];
        let mut region_visited = HashSet::new();
        let mut area = 0;
        let mut perimeter = 0;
        let mut queue = Vec::new();
        queue.push((r, c));
        region_visited.insert((r, c));
        while !queue.is_empty() {
            let (r, c) = queue.pop().unwrap();
            visited.insert((r, c));
            area += 1;
            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if !is_valid(nr as usize, nc as usize)
                    || input[nr as usize][nc as usize] != region_type
                {
                    perimeter += 1;
                    continue;
                }
                if !region_visited.contains(&(nr as usize, nc as usize)) {
                    region_visited.insert((nr as usize, nc as usize));
                    queue.push((nr as usize, nc as usize));
                }
            }
        }
        return area * perimeter;
    };
    for i in 0..rows {
        for j in 0..cols {
            if !visited.contains(&(i, j)) {
                sum += explore_region(i, j, &mut visited);
            }
        }
    }
    sum
}
fn read_input() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");
    input.lines().map(|line| line.chars().collect()).collect()
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
        assert_eq!(solve(input), 772);
    }
}
