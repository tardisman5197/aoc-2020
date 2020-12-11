use std::collections::HashSet;
use std::convert::TryFrom;

// get_seats returns a set off all of the coordinates
//pf seats in the input.
pub fn get_seats(lines: &Vec<String>) -> Result<HashSet<(i64, i64)>, Box<dyn std::error::Error>> {
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

// adjacent_occupied returns the number of occupied
// seats around the seat specified.
fn adjacent_occupied(seat: &(i64, i64), occupied: &HashSet<(i64, i64)>) -> i64 {
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

// update_seat returns true if the seat
// should be occupied in the next round.
pub fn update_seat(seat: &(i64, i64), occupied: &HashSet<(i64, i64)>) -> bool {
    let ao = adjacent_occupied(&seat, occupied);
    if occupied.contains(seat) {
        if ao >= 4 {
            return false;
        }
        return true;
    }
    if ao == 0 {
        return true;
    }
    return false;
}
