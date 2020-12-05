extern crate utils;

// get_seat_id takes the boarding pass and calculates
// the seat id based on the resulting row and column.
// A boarding pass is made up of characters that perform
// a binary search on the column or row of the seat.get_seat_id(boarding_pass: &str)
fn get_seat_id(boarding_pass: &str) -> Result<i64, Box<dyn std::error::Error>> {
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

    // The seat_id is calculated by doing
    //  row*8 + column
    Ok((i64::from(row) * 8) + i64::from(column))
}

// solve attempts to solve part 1 of day 5 of AoC 2020.
// Go through each boarding pass and calculate the
// seat id, then keep track of the highest seat id
// seen.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut highest_seat_id: i64 = 0;

    for line in lines.iter() {
        let current_seat_id = get_seat_id(line)?;
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

    #[test]
    fn test_get_seat_id() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(get_seat_id("FBFBBFFRLR")?, 357);
        assert_eq!(get_seat_id("BFFFBBFRRR")?, 567);
        assert_eq!(get_seat_id("FFFBBBFRRR")?, 119);
        assert_eq!(get_seat_id("BBFFBBFRLL")?, 820);
        Ok(())
    }
}
