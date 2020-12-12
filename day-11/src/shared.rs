use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

// get_seats returns a set off all of the coordinates
// seats in the input.
fn get_seats(
    lines: &Vec<String>,
    size: &(i64, i64),
    directly_adjacent: bool,
) -> Result<HashMap<(i64, i64), Vec<(i64, i64)>>, Box<dyn std::error::Error>> {
    let mut seat_set: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'L' {
                seat_set.insert((i64::try_from(x)?, i64::try_from(y)?));
            }
        }
    }
    let mut seats: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for seat in seat_set.iter() {
        if directly_adjacent {
            seats.insert(*seat, get_adjacent_neighbors(seat, &seat_set));
        } else {
            seats.insert(*seat, get_neighbors(seat, &seat_set, size));
        }
    }

    Ok(seats)
}

// get_adjacent_neighbors generates a Vec of the adjacent
// seats to the seat specified.
fn get_adjacent_neighbors(seat: &(i64, i64), seats: &HashSet<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut neighbors: Vec<(i64, i64)> = Vec::new();
    // For each direction:
    //  0 = negative
    //  1 = equal
    //  2 = positive
    for i in 0..3 {
        for j in 0..3 {
            if i != 1 || j != 1 {
                let position: (i64, i64) = (seat.0 + (i - 1), seat.1 + (j - 1));

                if seats.contains(&position) {
                    neighbors.push(position)
                }
            }
        }
    }

    neighbors
}

// get_neighbors returns a Vec if the closest seats
// in every direction to the seat specified.
fn get_neighbors(
    seat: &(i64, i64),
    seats: &HashSet<(i64, i64)>,
    size: &(i64, i64),
) -> Vec<(i64, i64)> {
    let mut neighbors: Vec<(i64, i64)> = Vec::new();
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
                        neighbors.push((x, y));
                        break;
                    }
                }
            }
        }
    }

    neighbors
}

// get_occupied_neighbors returns the number of
// occupied seats neighboring the specified seat.
fn get_occupied_neighbors(
    seat: &(i64, i64),
    occupied: &HashSet<(i64, i64)>,
    seats: &HashMap<(i64, i64), Vec<(i64, i64)>>,
) -> i64 {
    let mut occupied_neighbors: i64 = 0;

    let neighbors: &Vec<(i64, i64)> = match seats.get(seat) {
        Some(neighbors) => neighbors,
        None => return 0,
    };

    for neighbor in neighbors.iter() {
        if occupied.contains(neighbor) {
            occupied_neighbors += 1;
        }
    }

    occupied_neighbors
}

// update_seat returns true if the seat
// should be occupied in the next round.
fn update_seat(
    seat: &(i64, i64),
    occupied: &HashSet<(i64, i64)>,
    seats: &HashMap<(i64, i64), Vec<(i64, i64)>>,
    directly_adjacent: bool,
) -> bool {
    let occupied_neighbors = get_occupied_neighbors(seat, occupied, seats);
    if occupied.contains(seat) {
        if directly_adjacent {
            if occupied_neighbors >= 4 {
                return false;
            }
        } else {
            if occupied_neighbors >= 5 {
                return false;
            }
        }
        return true;
    }
    if occupied_neighbors == 0 {
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
    let seats: HashMap<(i64, i64), Vec<(i64, i64)>> = get_seats(lines, &size, directly_adjacent)?;

    let mut previous_occupied: HashSet<(i64, i64)> = HashSet::new();

    loop {
        let mut occupied: HashSet<(i64, i64)> = HashSet::new();

        for seat in seats.keys() {
            if update_seat(&seat, &previous_occupied, &seats, directly_adjacent) {
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
