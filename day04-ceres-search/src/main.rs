fn main() {
    let input = read_input();
    // print!("{}", part_one(input.clone()));
    println!("{}", part_two(input.clone()));
}
fn part_one(input: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    let word = "XMAS";
    let rows = input.len();
    let cols = input[0].len();
    let dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    (0..rows).into_iter().for_each(|row| {
        (0..cols).into_iter().for_each(|col| {
            dirs.iter().for_each(|(dx, dy)| {
                if is_word_found(&input, word, row as isize, col as isize, dx, dy) {
                    count += 1
                }
            });
        })
    });
    count
}
fn is_word_found(
    grid: &Vec<String>,
    word: &str,
    row: isize,
    col: isize,
    dx: &i32,
    dy: &i32,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    (0..4).into_iter().all(|i| {
        let x = row + *dx as isize * i as isize;
        let y = col + *dy as isize * i as isize;
        if x < 0 || y < 0 || x >= rows || y >= cols {
            return false;
        }
        grid[x as usize].chars().nth(y as usize).unwrap() == word.chars().nth(i as usize).unwrap()
    })
}
fn part_two(input: Vec<String>) -> u32 {
    let rows = input.len();
    let cols = input[0].len();
    let mut count: u32 = 0;
    (0..rows).into_iter().for_each(|row| {
        (0..cols).into_iter().for_each(|col| {
            if input[row].chars().nth(col) == Some('A') {
                count += count_xmas(&input, row, col);
            }
        });
    });
    count
}
fn count_xmas(grid: &Vec<String>, x_a: usize, y_a: usize) -> u32 {
    let directions = [
        (-1, -1, -1, 1),
        (-1, 1, 1, 1),
        (1, 1, 1, -1),
        (1, -1, -1, -1),
    ];
    let mut count: u32 = 0;
    for (dx, dy, ddx, ddy) in directions {
        if is_mas(grid, x_a, y_a, dx, dy) && is_mas(grid, x_a, y_a, ddx, ddy) {
            count += 1;
        }
    }
    count
}
fn is_mas(grid: &Vec<String>, x_a: usize, y_a: usize, dx: i32, dy: i32) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let m_pos = (x_a as isize + dx as isize, y_a as isize + dy as isize);
    let s_pos = (x_a as isize - dx as isize, y_a as isize - dy as isize);
    [m_pos, s_pos].iter().enumerate().all(|(i, &(x, y))| {
        if x < 0 || y < 0 || x >= rows || y >= cols {
            return false;
        }
        let expected_char = match i {
            0 => 'M',
            1 => 'S',
            _ => unreachable!(),
        };
        grid[x as usize].chars().nth(y as usize).unwrap() == expected_char
    })
}
fn read_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        assert_ne!(input.len(), 0);
    }
}
