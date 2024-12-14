use std::u32;

fn main() {
    let input = read_input();
    println!("{}", solve(input));
}
fn solve(machines: Vec<Vec<(u32, u32)>>) -> u32 {
    let mut total_cost = 0;
    for machine in machines {
        let cost = solve_for_machine(
            machine[0].0,
            machine[0].1,
            machine[1].0,
            machine[1].1,
            machine[2].0,
            machine[2].1,
        );
        if let Some(machine_cost) = cost {
            total_cost += machine_cost;
        }
    }
    total_cost
}
fn solve_for_machine(x_a: u32, y_a: u32, x_b: u32, y_b: u32, x_p: u32, y_p: u32) -> Option<u32> {
    let mut min = u32::MAX;
    for x in 0..101 {
        for y in 0..101 {
            if x_a * x + x_b * y == x_p && y_a * x + y_b * y == y_p {
                min = min.min(3 * x + y);
            }
        }
    }
    return if min == u32::MAX { None } else { Some(min) };
}
fn read_input() -> Vec<Vec<(u32, u32)>> {
    let input = include_str!("input.txt");
    let mut machines = Vec::new();
    for block in input.split("\n\n") {
        let mut machine = Vec::new();
        for line in block.lines() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts[0].starts_with("B") {
                if let Some((x_str, y_str)) = parts.get(2).zip(parts.get(3)) {
                    let x = x_str
                        .trim_start_matches(&['X', '+', ','][..])
                        .trim_end_matches(&[','][..])
                        .parse()
                        .unwrap();
                    let y = y_str.trim_start_matches(&['Y', '+'][..]).parse().unwrap();
                    machine.push((x, y));
                }
            } else if parts[0].starts_with("P") {
                if let Some((x_str, y_str)) = parts.get(1).zip(parts.get(2)) {
                    let x = x_str
                        .trim_start_matches(&['X', '=', ','][..])
                        .trim_end_matches(&[','][..])
                        .parse()
                        .unwrap();
                    let y = y_str.trim_start_matches(&['Y', '='][..]).parse().unwrap();
                    machine.push((x, y));
                }
            }
        }
        machines.push(machine);
    }
    machines
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
    #[test]
    fn test_solve_part_one() {
        let input = read_input();
        assert_eq!(solve(input), 480);
    }
}
