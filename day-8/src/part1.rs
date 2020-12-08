extern crate utils;
use std::collections::HashMap;

// solve attempts to solve part 1 of day 6 of AoC 2020.
// Calculate the sum of the unique values for each set
// of declaration forms.
pub fn solve(lines: &Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Err(Box::new(utils::errors::SomethingIsWrongError::new(
        "Not Implemented",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 5);
        Ok(())
    }
}
