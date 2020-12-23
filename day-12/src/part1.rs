extern crate utils;

// solve attempts to solve part 1 of day 12 of AoC 2020.
// Navigate using the directions given in the input.
pub fn solve(lines: &Vec<String>) -> Result<f64, Box<dyn std::error::Error>> {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut direction: f64 = 90.0; // Face East

    for line in lines.iter() {
        if line.len() < 2 {
            continue;
        }
        let mut value: String = line.to_owned();
        let amount: f64 = value.split_off(1).parse::<f64>()?;
        match line.chars().nth(0) {
            Some(op) => match op {
                'N' => y += amount,
                'S' => y -= amount,
                'E' => x += amount,
                'W' => x -= amount,
                'L' => {
                    direction = (direction - amount) % 360.0;
                    if direction < 0.0 {
                        direction += 360.0;
                    }
                }
                'R' => direction = (direction + amount) % 360.0,
                'F' => {
                    let (s, c): (f64, f64) = direction.to_radians().sin_cos();
                    x += amount * s;
                    y += amount * c;
                }
                _ => (),
            },
            None => (),
        }
    }

    Ok(x.abs() + y.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "F10
N3
F7
R90
F11";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 25.0);
        Ok(())
    }
}
