fn main() {
    let input = read_input();
    println!("{}", solve(input.clone(), true)); // change to false for part one.
}
fn solve(reports: Vec<Vec<u32>>, _2: bool) -> usize {
    reports
        .iter()
        .filter(|report| {
            is_report_safe(report)
                || (_2
                    && (0..report.len()).any(|i| {
                        let new_report = report
                            .iter()
                            .enumerate()
                            .filter(|&(idx, _)| idx != i)
                            .map(|(_, &val)| val)
                            .collect();
                        is_report_safe(&new_report)
                    }))
        })
        .count()
}
fn is_report_safe(report: &Vec<u32>) -> bool {
    let is_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = report.windows(2).all(|w| w[0] > w[1]);
    if !(is_increasing || is_decreasing) {
        return false;
    }
    report.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        diff >= 1 && diff <= 3
    })
}

fn read_input() -> Vec<Vec<u32>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        println!("{:?}", input);
        assert_ne!(input.len(), 0);
    }
}
