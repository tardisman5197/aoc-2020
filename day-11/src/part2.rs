extern crate utils;
use super::shared;

// solve attempts to solve part 2 of day 11 of AoC 2020.
// Return the number of occupied seats in the steady state
// of the input.
pub fn solve(lines: &Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(shared::occupied_seats_in_steady_state(lines, false)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 26);
        Ok(())
    }
}
