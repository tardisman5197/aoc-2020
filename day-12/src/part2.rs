extern crate utils;

// solve attempts to solve part 2 of day x of AoC 2020.
pub fn solve(_lines: &Vec<String>) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 0.0);
        Ok(())
    }
}
