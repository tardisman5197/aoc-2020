use std::collections::HashSet;
use std::convert::TryFrom;

// get_seats returns a set off all of the coordinates
//pf seats in the input.
fn get_seats(lines: &Vec<String>) -> Result<HashSet<(i64, i64)>, Box<dyn std::error::Error>> {
    let mut seats: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'L' {
                seats.insert((i64::try_from(x)?, i64::try_from(y)?));
            }
        }
    }
    Ok(seats)
}

// directly_adjacent_occupied returns the number of occupied
// tiles around the seat specified.
fn directly_adjacent_occupied(seat: &(i64, i64), occupied: &HashSet<(i64, i64)>) -> i64 {
    let mut occupied_adjacent: i64 = 0;
    for x in 0..3 {
        for y in 0..3 {
            if x != 1 || y != 1 {
                if occupied.contains(&(seat.0 + (x - 1), seat.1 + (y - 1))) {
                    occupied_adjacent += 1;
                }
            }
        }
    }
    return occupied_adjacent;
}

// adjacent_occupied returns the number of occupied
// seats around the seat specified.
fn adjacent_occupied(
    seat: &(i64, i64),
    occupied: &HashSet<(i64, i64)>,
    seats: &HashSet<(i64, i64)>,
    size: &(i64, i64),
) -> i64 {
    let mut occupied_adjacent: i64 = 0;
    // For each direction:
    //  0 = negative
    //  1 = equal
    //  2 = positive
    for dir_x in 0..3 {
        for dir_y in 0..3 {
            if dir_x != 1 || dir_y != 1 {
                let delta_x: i64 = dir_x - 1;
                let delta_y: i64 = dir_y - 1;

                let mut x: i64 = seat.0;
                let mut y: i64 = seat.1;

                while (x >= 0 && x < size.0) && (y >= 0 && y < size.1) {
                    x += delta_x;
                    y += delta_y;

                    if seats.contains(&(x, y)) {
                        if occupied.contains(&(x, y)) {
                            occupied_adjacent += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    return occupied_adjacent;
}

// update_seat returns true if the seat
// should be occupied in the next round.
fn update_seat(
    seat: &(i64, i64),
    occupied: &HashSet<(i64, i64)>,
    seats: &HashSet<(i64, i64)>,
    size: &(i64, i64),
    directly_adjacent: bool,
) -> bool {
    let ao: i64;
    if directly_adjacent {
        ao = directly_adjacent_occupied(seat, occupied);
    } else {
        ao = adjacent_occupied(seat, occupied, seats, size);
    }
    if occupied.contains(seat) {
        if directly_adjacent {
            if ao >= 4 {
                return false;
            }
        } else {
            if ao >= 5 {
                return false;
            }
        }
        return true;
    }
    if ao == 0 {
        return true;
    }
    return false;
}

// occupied_seats_in_steady_state returns the number
// of seats that are occupied when the map reaches a
// steady state.
pub fn occupied_seats_in_steady_state(
    lines: &Vec<String>,
    directly_adjacent: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    let y_size: i64 = i64::try_from(lines.len())?;
    let x_size: i64 = match lines.get(0) {
        Some(line) => i64::try_from(line.len())?,
        None => {
            return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                "Input Lines Empty",
            )))
        }
    };
    let size: (i64, i64) = (x_size, y_size);
    let seats: HashSet<(i64, i64)> = get_seats(lines)?;

    let mut previous_occupied: HashSet<(i64, i64)> = HashSet::new();

    loop {
        let mut occupied: HashSet<(i64, i64)> = HashSet::new();

        for seat in seats.iter() {
            if update_seat(&seat, &previous_occupied, &seats, &size, directly_adjacent) {
                occupied.insert(*seat);
            }
        }

        if previous_occupied == occupied {
            break;
        }

        previous_occupied = occupied;
    }

    Ok(previous_occupied.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_occupied() -> Result<(), Box<dyn std::error::Error>> {
        // #L.
        // .#.
        // ...
        let mut seats: HashSet<(i64, i64)> = HashSet::new();
        seats.insert((0, 1));
        seats.insert((0, 0));
        seats.insert((1, 1));
        let mut occupied: HashSet<(i64, i64)> = HashSet::new();
        occupied.insert((0, 0));
        assert_eq!(adjacent_occupied(&(1, 1), &seats, &occupied, &(3, 3)), 1);

        // ##.
        // .#.
        // ...
        let mut seats: HashSet<(i64, i64)> = HashSet::new();
        seats.insert((0, 1));
        seats.insert((0, 0));
        seats.insert((1, 1));
        let mut occupied: HashSet<(i64, i64)> = HashSet::new();
        occupied.insert((0, 0));
        occupied.insert((0, 1));
        assert_eq!(adjacent_occupied(&(1, 1), &seats, &occupied, &(3, 3)), 2);

        // #L..
        // .#.#
        // ....
        let mut seats: HashSet<(i64, i64)> = HashSet::new();
        seats.insert((0, 1));
        seats.insert((0, 0));
        seats.insert((1, 1));
        seats.insert((3, 1));
        let mut occupied: HashSet<(i64, i64)> = HashSet::new();
        occupied.insert((0, 0));
        occupied.insert((3, 1));
        assert_eq!(adjacent_occupied(&(1, 1), &seats, &occupied, &(4, 3)), 2);

        // #L..
        // .#.#
        // ....
        // ...#
        let mut seats: HashSet<(i64, i64)> = HashSet::new();
        seats.insert((0, 1));
        seats.insert((0, 0));
        seats.insert((1, 1));
        seats.insert((3, 1));
        seats.insert((3, 3));
        let mut occupied: HashSet<(i64, i64)> = HashSet::new();
        occupied.insert((0, 0));
        occupied.insert((3, 1));
        occupied.insert((3, 3));
        assert_eq!(adjacent_occupied(&(1, 1), &seats, &occupied, &(4, 4)), 3);

        Ok(())
    }
}
