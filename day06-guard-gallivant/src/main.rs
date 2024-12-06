use std::collections::HashSet;

fn main() {
    let input = read_input();
    println!("{}", solve(input));
}
#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn step(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
}
fn solve(input: Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let (start_pos, _) = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|x| *x == '^')
                .map(|j| ((i as isize, j as isize), 1))
        })
        .unwrap();
    let mut curr_pos = start_pos;
    let mut curr_direction = Direction::Up;
    loop {
        if !visited.contains(&curr_pos) {
            visited.insert(curr_pos);
            count += 1;
        }
        if curr_pos.0 == 0
            || curr_pos.1 == 0
            || curr_pos.0 == (input.len() - 1) as isize
            || curr_pos.1 == (input[0].len() - 1) as isize
        {
            break;
        }
        let mut next_pos = curr_direction.step(curr_pos);
        while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            curr_direction = curr_direction.next();
            next_pos = curr_direction.step(curr_pos);
        }
        curr_pos = next_pos;
    }
    count
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
        assert_ne!(input.len(), 0);
        assert_ne!(input[0].len(), 0);
    }
}
