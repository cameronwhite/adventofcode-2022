#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
fn solve_part1(entries: &Vec<Vec<i32>>) -> i32 {
    entries.iter().map(|v| v.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(entries: &Vec<Vec<i32>>) -> i32 {
    let mut totals: Vec<i32> = entries.iter().map(|v| v.iter().sum()).collect();
    totals.sort();
    totals.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        assert_eq!(solve_part1(&parse(input)), 24000);
        assert_eq!(solve_part2(&parse(input)), 45000);
    }
}
