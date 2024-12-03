use regex::Regex;
fn main() {
    let input = read_input();
    println!("{}", solve(input.clone(), true)); //change to false for part_one of the puzzle.
}
fn solve(input: String, _2: bool) -> u32 {
    let mut sum: u32 = 0;
    let mut mul: bool = true;
    let pat = match _2 {
        true => r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)",
        false => r"mul\((\d{1,3}),(\d{1,3})\)",
    };
    let re = Regex::new(pat).unwrap();
    re.find_iter(&input).for_each(|mat| {
        let mat = mat.as_str();
        if _2 {
            if mat.starts_with("mul") {
                if mul {
                    sum += calc(mat);
                }
            } else {
                mul = !mat.starts_with("don't")
            }
        } else {
            sum += calc(mat);
        }
    });
    sum
}
fn calc(mat: &str) -> u32 {
    mat.replace("mul(", "")
        .replace(")", "")
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .fold(1, |a, b| a * b)
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
