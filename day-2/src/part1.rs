extern crate utils;

// Entry is all the information that
// can be retrieved from an input line.
struct Entry {
    lower_limit: usize,
    upper_limit: usize,
    character: String,
    password: String
}

// parse_line takes a line from Day 2's input and attempts
// to parse the line into an Entry struct.
fn parse_line(line: String) -> Result<Entry, Box<dyn std::error::Error>> {
    // Split the line by white space,
    // this should result in:
    //  lower_limit-upper_limit
    //  character:
    //  password
    let mut fields: Vec<String> = line.split_whitespace().map(String::from).collect();
    if fields.len() != 3 {
        return Err(Box::new(utils::errors::SomethingIsWrongError::new("Invalid Entry")))
    };

    // Parse the lower and upper limits
    let limits: Vec<usize> = fields[0].split("-").flat_map(|limit| limit.parse::<usize>()).collect();
    if limits.len() != 2 {
        return Err(Box::new(utils::errors::SomethingIsWrongError::new("Invalid Entry, limits are wrong")))
    };

    // Take the ':' off of the character
    let character_field_length = fields[1].len() - 1;
    fields[1].truncate(character_field_length);

    // Create the Entry to return
    Ok(Entry{
        lower_limit: limits[0], 
        upper_limit: limits[1],
        character: fields[1].clone(),
        password: fields[2].clone(),
    })
}

// solve attempts to solve Day 2 Part 1 of AoC.
// Parse each line of the index, then get the number
// of occurrences of the character specified in the password.
// Check this number against the limits, then record
// if it passed the criteria.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut valid_password_count: i32 = 0;

    for line in lines.iter() {
        let entry: Entry = parse_line(line.to_string())?;
        let occurrences = entry.password.matches(&entry.character).count();
        if occurrences >= entry.lower_limit && occurrences <= entry.upper_limit {
            valid_password_count= valid_password_count+1;
        };
    };

    Ok(valid_password_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<String> = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        
        assert_eq!(solve(&input)?, 2);
        Ok(())
    }
}