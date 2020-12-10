extern crate utils;

// solve attempts to solve part 2 of day 9 of AoC 2020.
// Find a contiguous set of a numbers which sum to
// the invalid number. This function then returns
// the sum of the smallest and largest number in the
// set.
pub fn solve(lines: &Vec<String>, invalid_number: i64) -> Result<i64, Box<dyn std::error::Error>> {
    // Convert to numbers
    let numbers: Vec<i64> = lines.iter().flat_map(|x| x.parse::<i64>()).collect();

    // For each number try to find a
    // contiguous set which add to the
    // invalid number
    for i in 0..numbers.len() {
        let mut current_sum: i64 = numbers[i];
        let mut smallest: i64 = numbers[i];
        let mut largest: i64 = numbers[i];
        // Add each of the numbers after
        // the current index
        for j in i + 1..numbers.len() {
            current_sum += numbers[j];
            // Stop if the current sum is more
            // than the invalid number
            if current_sum > invalid_number {
                break;
            }
            // Track the smallest number
            if numbers[j] < smallest {
                smallest = numbers[j];
            }
            // Track the largest number
            if numbers[j] > largest {
                largest = numbers[j];
            }
            // Check if the number is the one
            // we are looking for.
            if current_sum == invalid_number {
                return Ok(smallest + largest);
            }
        }
    }

    Err(Box::new(utils::errors::NoResultError::new(
        "No Contiguous Set",
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

        assert_eq!(solve(&input, 127)?, 62);
        Ok(())
    }
}
