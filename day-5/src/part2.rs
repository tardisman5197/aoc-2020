extern crate utils;
use super::shared;
use std::collections::HashSet;

// solve attempts to solve part 2 of day 5 of AoC 2020.
// Try and find my seat.
pub fn solve(lines: &Vec<String>) -> Result<u16, Box<dyn std::error::Error>> {
    let mut plane: HashSet<u16> = HashSet::new();

    // Fill the plane with other passengers.
    for line in lines.iter() {
        plane.insert(shared::get_seat_id(line)?);
    }

    for passenger in 0..(128 * 8) {
        // Check if the seat is full
        if plane.contains(&passenger) {
            continue;
        }

        // The plane is full and I am told that
        // there is somebody in the seat id above
        // and below me.
        if !plane.contains(&(passenger - 1)) || !plane.contains(&(passenger + 1)) {
            continue;
        }

        // This seat_id must be ours
        return Ok(passenger);
    }
    // We have checked all of the seats
    // and there were none.
    Err(Box::new(utils::errors::NoResultError::new("No Seat Found")))
}
