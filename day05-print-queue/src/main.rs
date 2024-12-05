use std::collections::HashMap;
fn main() {
    let (rules, updates) = read_input();
    let res = solve(rules, updates, true);
    println!("{} ", res);
}
fn solve(rules: HashMap<u32, Vec<u32>>, updates: Vec<Vec<u32>>, _2: bool) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            let sorted = topological_sort(update, &rules);
            let condition = if _2 {
                &sorted != update
            } else {
                &sorted == update
            };
            if !sorted.is_empty() && condition {
                Some(sorted[sorted.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
fn topological_sort(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut in_degree: HashMap<u32, usize> = update.iter().map(|&u| (u, 0)).collect();
    for (u, neighbors) in rules {
        if update.contains(u) {
            for v in neighbors {
                if update.contains(v) {
                    *in_degree.entry(*v).or_default() += 1;
                }
            }
        }
    }
    let mut queue: Vec<u32> = in_degree
        .iter()
        .filter_map(|(&k, &v)| if v == 0 { Some(k) } else { None })
        .collect();
    let mut ans = Vec::new();
    while let Some(u) = queue.pop() {
        ans.push(u);
        if let Some(neighbors) = rules.get(&u) {
            for &v in neighbors {
                if update.contains(&v) {
                    if let Some(degree) = in_degree.get_mut(&v) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(v);
                        }
                    }
                }
            }
        }
    }
    if ans.len() == update.len() {
        ans
    } else {
        Vec::new()
    }
}
fn read_input() -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let input = include_str!("input.txt");
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules = rules_str.lines().fold(HashMap::new(), |mut acc, line| {
        let mut parts = line.split('|').map(|s| s.parse::<u32>().unwrap());
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        acc.entry(key).or_insert_with(Vec::new).push(value);
        acc
    });

    let updates = updates_str
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<u32>().unwrap()).collect())
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
