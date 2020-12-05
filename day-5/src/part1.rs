extern crate utils;

// get_seat_id takes the boarding pass and calculates
// the seat id based on the resulting row and column.
fn get_seat_id(boarding_pass: &String) -> Result<i64, Box<dyn std::error::Error>> {
    Err(Box::new(utils::errors::SomethingIsWrongError::new("Not Implemented")))
}

// solve attempts to solve part 1 of day 5 of AoC 2020.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    Err(Box::new(utils::errors::SomethingIsWrongError::new("Not Implemented")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: String = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL".to_string();
        
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 820);
        Ok(())
    }

    #[test]
    fn test_get_seat_id() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(get_seat_id(&"FBFBBFFRLR".to_string()), 357);
        assert_eq!(get_seat_id(&"BFFFBBFRRR".to_string()), 567);
        assert_eq!(get_seat_id(&"FFFBBBFRRR".to_string()), 119);
        assert_eq!(get_seat_id(&"BBFFBBFRLL".to_string()), 820);
        Ok(())
    }
}