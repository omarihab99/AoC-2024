use std::collections::HashMap;

fn main() {
    let (rules, updates) = read_input();
    println!("{}", solve(rules, updates));
}
fn solve(rules: HashMap<u32, Vec<u32>>, updates: Vec<Vec<u32>>) -> u32 {
    let mut middles: Vec<u32> = Vec::new();
    updates.iter().for_each(|update| {
        if is_valid_order(&rules, update) {
            middles.push(update[update.len() / 2]);
        }
    });
    middles.iter().sum()
}
fn is_valid_order(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if !rules.get(&update[i]).unwrap().contains(&update[j]) {
                return false;
            }
        }
    }
    true
}
fn read_input() -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let input = include_str!("input.txt");
    let mut sections = input.split("\n\n");
    let rules = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|s| s.trim().parse::<u32>().unwrap());
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            (key, value)
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert_with(Vec::new).push(value);
            acc
        });
    let updates = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    (rules, updates)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let (rules, updates) = read_input();
        assert_ne!(rules.len(), 0);
        assert_ne!(updates.len(), 0);
    }
}
