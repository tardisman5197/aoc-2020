use std::collections::HashSet;
extern crate utils;

// solve attempts to solve part 1 of day 1 of AoC 2020.
// A set of all the values are created as each line in
// the input file is read. If the partner value is found
// for the current entry in the entries set then the
// result is the product of the pair of values.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut entries: HashSet<i32> = HashSet::new();

    // Loop through each line of the input
    for line in lines.iter() {
        // Skip any blank lines
        if line.len() <= 0 {
            continue;
        }

        // Check to see if the line is a number
        let current_entry: i32 = line.parse::<i32>()?;

        // Calculate the partner value
        let needed_entry: i32 = 2020 - current_entry;

        // Check if the partner value has already
        // been seen.
        if entries.contains(&needed_entry) {
            // Found the pair, multiply them and return
            return Ok(current_entry * needed_entry);
        }

        // Partner value not found, add this value to
        // the set.
        entries.insert(current_entry);
    }

    Err(Box::new(utils::errors::NoResultError::new(
        "No entries matching requirements",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<String> = vec![
            "1721".to_string(),
            "979".to_string(),
            "366".to_string(),
            "299".to_string(),
            "675".to_string(),
            "1456".to_string(),
        ];

        assert_eq!(solve(&input)?, 514579);
        Ok(())
    }
}
