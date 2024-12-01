use std::collections::HashMap;

fn main() {
    let (first_group, second_group) = read_input();
    // let dis = part_one(first_group.clone(), second_group.clone());
    let similarity_score = part_two(&first_group, &second_group);
    println!("{}", similarity_score);
    // println!("{}", dis);
}
#[allow(dead_code)]
fn part_two(first_group: &Vec<u32>, second_group: &Vec<u32>) -> u32 {
    let second_group_counts = second_group.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    let mut similarity_score: u32 = 0;
    first_group.iter().for_each(|num| {
        if let Some(count) = second_group_counts.get(num) {
            similarity_score += count * num;
        }
    });
    similarity_score
}
#[allow(dead_code)]
fn part_one(mut first_group: Vec<u32>, mut second_group: Vec<u32>) -> u32 {
    first_group.sort();
    second_group.sort();
    let mut dis = 0;
    for i in 0..first_group.len() {
        dis += first_group[i].abs_diff(second_group[i]);
    }
    dis
}
fn read_input() -> (Vec<u32>, Vec<u32>) {
    let input = include_str!("input.txt");
    let mut first_group = Vec::new();
    let mut second_group = Vec::new();
    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok());
        if let Some(first) = nums.next() {
            first_group.push(first);
        }
        if let Some(second) = nums.next() {
            second_group.push(second);
        }
    }
    (first_group, second_group)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let (first_group, second_group) = read_input();
        assert_ne!(first_group.len(), 0);
        assert_ne!(second_group.len(), 0);
    }
}
