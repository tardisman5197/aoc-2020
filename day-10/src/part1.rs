extern crate utils;

// solve attempts to solve part 1 of day 10 of AoC 2020.
// Tally the number of 1 and 3 gaps between all of the
// adapters. Then this function returns the multiple
// product of the two tallies.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut adapters: Vec<i64> = lines.iter().flat_map(|x| x.parse::<i64>()).collect();
    // Sort the adapters
    adapters.sort();
    // Init the needed values
    let mut one_diff = 0;
    let mut three_diff = 0;
    let mut previous_adapter: i64 = 0;
    for adapter in adapters.iter() {
        let current_diff: i64 = adapter - previous_adapter;
        match current_diff {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => (),
        }
        previous_adapter = adapter.to_owned();
    }
    // +1 as the final device is 3 jolts above the
    // laster adapter.
    Ok(one_diff * (three_diff + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 220);
        Ok(())
    }
}
