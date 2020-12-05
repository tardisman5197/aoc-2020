// get_seat finds the row and column for the boarding pass provided.
// This is done by performing a binary search on the row and column,
// where the directions are specified by the characters in the
// boarding pass.
pub fn get_seat(boarding_pass: &str) -> Result<(u8, u8), Box<dyn std::error::Error>> {
    if boarding_pass.len() != 10 {
        return Err(Box::new(utils::errors::SomethingIsWrongError::new(
            "Invalid Boarding Pass",
        )));
    }

    // The size of the plane is 128 * 8
    let mut row_size: u8 = 128;
    let mut row: u8 = 0;

    let mut column_size: u8 = 8;
    let mut column: u8 = 0;

    for character in boarding_pass.chars() {
        match character {
            'F' => {
                // front half of the plane (lower)
                row_size /= 2;
            }
            'B' => {
                // back half of the plane (higher)
                row_size /= 2;
                row += row_size;
            }
            'L' => {
                // left side of the plane (lower)
                column_size /= 2;
            }
            'R' => {
                // right side of the plane (upper)
                column_size /= 2;
                column += column_size;
            }
            _ => {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Invalid Boarding Pass",
                )))
            }
        }
    }
    Ok((row, column))
}

// get_seat_id takes the boarding pass and calculates
// the seat id based on the specified row and column.
pub fn get_seat_id(seat: (u8, u8)) -> i64 {
    // The seat_id is calculated by doing
    //  row*8 + column
    (i64::from(seat.0) * 8) + i64::from(seat.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat_id() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(get_seat_id(get_seat("FBFBBFFRLR")?), 357);
        assert_eq!(get_seat_id(get_seat("BFFFBBFRRR")?), 567);
        assert_eq!(get_seat_id(get_seat("FFFBBBFRRR")?), 119);
        assert_eq!(get_seat_id(get_seat("BBFFBBFRLL")?), 820);
        Ok(())
    }
}
