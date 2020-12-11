extern crate utils;
use super::shared;

// solve attempts to solve part 1 of day 11 of AoC 2020.
// Find all of the seats in the input, then for every round,
// check if each seat needs to be occupied or not. When the
// state of the seats does not change between rounds returned
// the number of occupied seats.
pub fn solve(lines: &Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(shared::occupied_seats_in_steady_state(lines, true)?)
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

        assert_eq!(solve(&input)?, 37);
        Ok(())
    }
}
