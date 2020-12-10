// get_seat_id finds the row and column for the boarding pass provided.
// This is done by performing a binary search on the row and column,
// where the directions are specified by the characters in the
// boarding pass.
pub fn get_seat_id(boarding_pass: &str) -> Result<u16, Box<dyn std::error::Error>> {
    if boarding_pass.len() != 10 {
        return Err(Box::new(utils::errors::SomethingIsWrongError::new(
            "Invalid Boarding Pass",
        )));
    }
  
    let mut seat_id: u16 = 0;

    for (i, character) in boarding_pass.char_indices() {
        match character {
            'B' | 'R' => seat_id |= 1 << (9 - i),
            'F' | 'L' => (),
            _ => {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Invalid Boarding Pass",
                )))
            }
        }
    }
    Ok(seat_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat_id() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(get_seat_id("FBFBBFFRLR")?, 357);
        assert_eq!(get_seat_id("BFFFBBFRRR")?, 567);
        assert_eq!(get_seat_id("FFFBBBFRRR")?, 119);
        assert_eq!(get_seat_id("BBFFBBFRLL")?, 820);
        Ok(())
    }
}
