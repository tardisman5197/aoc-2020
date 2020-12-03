extern crate utils;

// solve attempts to solve Day 2 Part 1 of AoC.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    Err(Box::new(utils::errors::SomethingIsWrongError::new("Not Implemented")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<String> = vec![];
        
        assert_eq!(solve(&input)?, 0);
        Ok(())
    }
}