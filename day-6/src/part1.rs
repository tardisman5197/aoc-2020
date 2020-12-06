extern crate utils;

// solve attempts to solve part 1 of day 5 of AoC 2020.
// Go through each boarding pass and calculate the
// seat id, then keep track of the highest seat id
// seen.
pub fn solve(lines: &Vec<String>) -> Result<u16, Box<dyn std::error::Error>> {
    Err(Box::new(utils::errors::SomethingIsWrongError::new(
        "Not Implemented",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 11);
        Ok(())
    }
}
