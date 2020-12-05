extern crate utils;
use super::shared;

// solve attempts to solve part 1 of day 5 of AoC 2020.
// Go through each boarding pass and calculate the
// seat id, then keep track of the highest seat id
// seen.
pub fn solve(lines: &Vec<String>) -> Result<u16, Box<dyn std::error::Error>> {
    let mut highest_seat_id: u16 = 0;

    for line in lines.iter() {
        let current_seat_id: u16 = shared::get_seat_id(line)?;
        if current_seat_id > highest_seat_id {
            highest_seat_id = current_seat_id
        }
    }

    Ok(highest_seat_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 820);
        Ok(())
    }
}
