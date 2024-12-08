use std::{
    collections::{HashMap, HashSet},
    isize,
};

fn main() {
    let input = read_input();
    println!("{}", solve(input.clone(), true)); //change to false for part_one of the puzzle.
}
fn solve(input: Vec<Vec<char>>, _2: bool) -> u32 {
    let rows = input.len();
    let cols = input[0].len();
    let frequencies = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &c)| {
                if c != '.' {
                    Some((c, (i as isize, j as isize)))
                } else {
                    None
                }
            })
        })
        .fold(HashMap::new(), |mut acc, (freq, pos)| {
            acc.entry(freq).or_insert(Vec::new()).push(pos);
            acc
        });
    frequencies
        .values()
        .filter(|positions| positions.len() > 1)
        .flat_map(|positions| {
            positions.iter().enumerate().flat_map(move |(i, &pos1)| {
                positions.iter().skip(i + 1).map(move |&pos2| (pos1, pos2))
            })
        })
        .fold(HashSet::new(), |mut antinodes, (pos1, pos2)| {
            if _2 {
                part_two(pos1, pos2, rows, cols).for_each(|antinode| {
                    antinodes.insert(antinode);
                })
            } else {
                part_one(pos1, pos2, rows, cols).for_each(|antinode| {
                    antinodes.insert(antinode);
                });
            }
            antinodes
        })
        .len() as u32
}
fn part_one(
    pos1: (isize, isize),
    pos2: (isize, isize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (isize, isize)> {
    let dx = pos2.0 - pos1.0;
    let dy = pos2.1 - pos1.1;
    let antinode1 = (pos1.0 - dx, pos1.1 - dy);
    let antinode2 = (pos2.0 + dx, pos2.1 + dy);
    vec![antinode1, antinode2]
        .into_iter()
        .filter(move |&(x, y)| x >= 0 && x < rows as isize && y >= 0 && y < cols as isize)
}
fn part_two(
    pos1: (isize, isize),
    pos2: (isize, isize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (isize, isize)> {
    let dx = pos1.0 - pos2.0;
    let dy = pos1.1 - pos2.1;
    let line_points = |mut pos: (isize, isize), step: (isize, isize)| {
        std::iter::from_fn(move || {
            if pos.0 >= 0 && pos.0 < rows as isize && pos.1 >= 0 && pos.1 < cols as isize {
                let curr = pos;
                pos = (pos.0 + step.0, pos.1 + step.1);
                Some(curr)
            } else {
                None
            }
        })
    };
    line_points(pos1, (-dx, -dy)).chain(line_points(pos2, (dx, dy)))
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
    fn test_solve_part_one() {
        let input = read_input();
        assert_eq!(solve(input, false), 14);
    }
    #[test]
    fn test_solve_part_two() {
        let input = read_input();
        assert_eq!(solve(input, true), 34);
    }
}
