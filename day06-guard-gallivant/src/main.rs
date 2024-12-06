mod utils;
use std::collections::HashSet;
use utils::Direction;
fn main() {
    let input = read_input();
    println!("{}", solve(input, true)); //false for part one.
}

fn solve(input: Vec<Vec<char>>, _2: bool) -> u32 {
    let mut count_obstruction_positions: u32 = 0;
    let start_pos = find_start_pos(&input);
    let mut path = Vec::new();
    let count = process_guard(&input, start_pos, &mut path, false).0;
    if !_2 {
        count
    } else {
        for pos in path {
            if pos == start_pos {
                continue;
            }
            let mut modified_grid = input.clone();
            modified_grid[pos.0 as usize][pos.1 as usize] = '#';
            if process_guard(&modified_grid, start_pos, &mut Vec::new(), true).1 {
                count_obstruction_positions += 1
            }
        }
        count_obstruction_positions
    }
}
fn process_guard(
    input: &Vec<Vec<char>>,
    start_pos: (isize, isize),
    path: &mut Vec<(isize, isize)>,
    check_trap: bool, // to check whether you will solve part 01/get path or use to check if guard is trapped.
) -> (u32, bool) {
    let mut curr_pos = start_pos;
    let mut visited: HashSet<(isize, isize, Direction)> = HashSet::new();
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut curr_direction = Direction::Up;
    loop {
        let state = (curr_pos.0, curr_pos.1, curr_direction);
        if check_trap && visited.contains(&state) {
            return (0, true);
        }
        if !visited_positions.contains(&(curr_pos)) {
            path.push(curr_pos);
            visited_positions.insert(curr_pos);
        }
        visited.insert(state);
        if curr_pos.0 == 0
            || curr_pos.1 == 0
            || curr_pos.0 == (input.len() - 1) as isize
            || curr_pos.1 == (input[0].len() - 1) as isize
        {
            return (visited_positions.len() as u32, false);
        }
        let mut next_pos = curr_direction.step(curr_pos);
        while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            curr_direction = curr_direction.next();
            next_pos = curr_direction.step(curr_pos);
        }
        curr_pos = next_pos;
    }
}
fn find_start_pos(input: &Vec<Vec<char>>) -> (isize, isize) {
    input
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|x| *x == '^')
                .map(|j| (i as isize, j as isize))
        })
        .unwrap()
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
