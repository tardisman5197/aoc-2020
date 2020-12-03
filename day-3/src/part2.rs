extern crate utils;


// check_slope takes a map and the slope.
// The slope is made of the delta_right and delta_down.
// This function returns the number pf trees on the slope.
pub fn check_slope(lines: &Vec<String>, slope: &(usize, usize)) -> Result<i64, Box<dyn std::error::Error>> {
    let mut tree_count = 0;

    // Set the delta values
    let delta_right = slope.0;
    let delta_down = slope.1;
    let mut current_down = 0;

    for (i, line) in lines.iter().enumerate() {
        // Only check the necessary lines
        if i%delta_down != 0 {
            continue
        }

        // Calculate current position in line:
        //  (current_down*delta_right)%line_length
        let current_right = match line.chars().nth((current_down*delta_right)%line.len()) {
            Some(current_right) => current_right,
            None => return Err(Box::new(utils::errors::SomethingIsWrongError::new("Could not get position"))),
        };

        // Check if it is a tree, if so count it
        if current_right == '#' {
            tree_count = tree_count + 1;
        };

        current_down = current_down + 1;
    };

    Ok(tree_count)
}

// solve attempts to solve part 2 of day 3 of AoC 2020.
// This function checks the number of trees on all of the
// specified slopes and returns the number of trees on
// each slope multiplied.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let slopes: Vec<(usize, usize)> = vec![
        (1, 1), 
        (3, 1), 
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut total_tree_count: i64 = 1;

    for slope in slopes.iter() {
        total_tree_count = total_tree_count * check_slope(lines, slope)?
    }

    Ok(total_tree_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<String> = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];
        
        assert_eq!(solve(&input)?, 336);
        Ok(())
    }
}