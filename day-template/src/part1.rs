extern crate utils;

// solve attempts to solve part 1 of day x of AoC 2020.
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
        let file: &str = "";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 0);
        Ok(())
    }
}
