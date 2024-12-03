use regex::Regex;
fn main() {
    let input = read_input();
    println!("{}", part_one(input.clone()));
}
fn part_one(input: String) -> u32 {
    let mut sum: u32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.find_iter(&input).for_each(|mat| {
        sum += mat
            .as_str()
            .replace("mul(", "")
            .replace(")", "")
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .fold(1, |a, b| a * b);
    });
    sum
}
fn read_input() -> String {
    let input = include_str!("input.txt");
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = read_input();
        assert_ne!(input.len(), 0);
    }
}
