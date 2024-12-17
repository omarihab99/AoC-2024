use std::collections::HashSet;

fn main() {
    let (map, instructions) = read_input();
    println!("{}", solve(map, instructions));
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new(movement: char) -> Self {
        match movement {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!(),
        }
    }
    fn step(&self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

fn solve(mut map: Vec<Vec<char>>, instructions: Vec<Vec<char>>) -> i32 {
    let mut box_positions = HashSet::new();
    let mut robot_pos = (0, 0);
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| {
            if *cell == '@' {
                robot_pos = (i as isize, j as isize);
            } else if *cell == 'O' {
                box_positions.insert((i as isize, j as isize));
            }
        })
    });

    for instruction in instructions {
        for movement in instruction {
            let direction = Direction::new(movement);
            let (dx, dy) = direction.step();
            let next_pos = (robot_pos.0 + dx, robot_pos.1 + dy);
            let mut cur_pos = next_pos;
            let mut boxes_to_move = Vec::new();
            while box_positions.contains(&cur_pos) {
                boxes_to_move.push(cur_pos);
                cur_pos = (cur_pos.0 + dx, cur_pos.1 + dy);
            }
            if cur_pos.0 >= 0
                && cur_pos.0 < map.len() as isize
                && cur_pos.1 >= 0
                && cur_pos.1 < map[0].len() as isize
                && map[cur_pos.0 as usize][cur_pos.1 as usize] == '#'
            {
                continue;
            }
            if cur_pos.0 >= 0
                && cur_pos.0 < map.len() as isize
                && cur_pos.1 >= 0
                && cur_pos.1 < map[0].len() as isize
                && map[cur_pos.0 as usize][cur_pos.1 as usize] != '#'
            {
                for box_to_move in boxes_to_move.iter().rev() {
                    let next_box_pos = (box_to_move.0 + dx, box_to_move.1 + dy);
                    box_positions.remove(box_to_move);
                    box_positions.insert(next_box_pos);
                    map[box_to_move.0 as usize][box_to_move.1 as usize] = '.';
                    map[next_box_pos.0 as usize][next_box_pos.1 as usize] = 'O';
                }
                map[robot_pos.0 as usize][robot_pos.1 as usize] = '.';
                map[next_pos.0 as usize][next_pos.1 as usize] = '@';
                robot_pos = next_pos;
            }
        }
    }
    box_positions.iter().fold(0, |mut sum, &pos| {
        sum += 100 * (pos.0 as i32) + (pos.1 as i32);
        sum
    })
}
fn read_input() -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let input = include_str!("input.txt");
    let mut input_split = input.split("\n\n");
    let to_char_grid = |input: &str| -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    };
    let map = to_char_grid(input_split.next().unwrap());
    let instructions = to_char_grid(input_split.next().unwrap());
    (map, instructions)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_read_input() {
        let (map, instructions) = read_input();
        println!("{:?}", map);
        println!("{:?}", instructions);
        assert_ne!(map.len(), 0);
        assert_ne!(instructions.len(), 0);
    }
    #[test]
    fn test_part_one() {
        let (map, instructions) = read_input();
        assert_eq!(solve(map, instructions), 2028);
    }
}
