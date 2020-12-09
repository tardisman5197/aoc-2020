extern crate utils;
use std::collections::HashMap;
use std::collections::HashSet;

// solve attempts to solve part 1 of day 9 of AoC 2020.
// Find the first number, excluding the preamble, which
// can not be made by adding two numbers from the previous
// few numbers.
pub fn solve(lines: &Vec<String>, preamble: usize) -> Result<i64, Box<dyn std::error::Error>> {
    let mut sums: HashMap<usize, HashSet<i64>> = HashMap::new();
    let mut numbers: HashMap<usize, i64> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        // Parse the current line
        let x: i64 = line.parse::<i64>()?;

        // Only consider the numbers which are not
        // in the preamble
        if i >= preamble {
            // remove the sums and numbers which
            // are too far from the current number
            sums.remove(&(i - preamble));
            numbers.remove(&(i - preamble));

            // Check if the number is valid
            let mut valid: bool = false;
            for sum in sums.values() {
                if sum.contains(&x) {
                    valid = true;
                    break;
                }
            }
            // Part 1 asks for the first number
            // which is not valid
            if !valid {
                return Ok(x);
            }
        }
        // Calculate all of the sums with the previous numbers
        let mut current_sums: HashSet<i64> = HashSet::new();
        for y in numbers.values() {
            current_sums.insert(x + y);
        }
        // Add the number and sums
        numbers.insert(i, x);
        sums.insert(i, current_sums);
        continue;
    }

    Err(Box::new(utils::errors::NoResultError::new(
        "All Numbers are Valid",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input, 5)?, 127);
        Ok(())
    }
}
