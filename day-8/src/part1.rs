extern crate utils;
use super::shared;
use std::collections::HashMap;

// solve attempts to solve part 1 of day 8 of AoC 2020.
// Identify the value of the accumulator when a loop
// occurs in the input instruction set.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: HashMap<usize, shared::Op> = shared::parse_instructions(lines)?;
    // The input should not complete successfully and will return (value, false).
    let result: (i64, bool) = shared::run_instructions(&instructions)?;
    // Return the i64 regardless of the bool.
    Ok(result.0)
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
