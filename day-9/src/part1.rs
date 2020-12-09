extern crate utils;

// solve attempts to solve part 1 of day 9 of AoC 2020.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    Err(Box::new(utils::errors::SomethingIsWrongError::new(
        "Not Implemented",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 127);
        Ok(())
    }
}
