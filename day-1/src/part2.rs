use std::collections::HashMap;
extern crate utils;

// solve attempts to solve part 2 of day 1 of AoC 2020.
// For each entry in the input, the number needed to sum
// to the required amount is calculated. This value is
// then checked against a HashMap storing the entries and
// their sums. If The required amount is found in the
// HashMap the required entries have been found. The
// result is then the product of these entries.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut entries: Vec<i32> = Vec::new();
    let mut added_entries: HashMap<i32, (i32, i32)> = HashMap::new();

    // Loop through each line of the input
    for line in lines.iter() {
        // Skip any blank lines
        if line.len() <= 0 {
            continue;
        }

        // Check to see if the line is a number
        let current_entry: i32 = line.parse::<i32>()?;

        // Calculate the partners value total
        let needed_entry: i32 = 2020 - current_entry;

        // Check if the partners sum has already
        // been seen.
        if added_entries.contains_key(&needed_entry) {
            // Found the partners, multiply them and return
            match added_entries.get(&needed_entry) {
                Some(partners) => return Ok(current_entry * partners.0 * partners.1),
                None => return Err(Box::new(utils::errors::SomethingIsWrongError::new("The map didn't contain the partners"))),
            }
        }

        // Compute all of the sums with all of the
        // entries in the input
        for entry in entries.iter() {
            added_entries.insert(current_entry+entry, (current_entry, *entry));
        }
        // Partners value not found, add this value to
        // the Vector of entries.
        entries.push(current_entry);
    }

    Err(Box::new(utils::errors::NoResultError::new("No entries matching requirements")))
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
        
        assert_eq!(solve(&input)?, 241861950);
        Ok(())
    }
}