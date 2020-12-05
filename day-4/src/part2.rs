extern crate utils;
use regex::Regex;
use std::fmt;

// Passport contains all of the possible
// values a passport can hold.
#[derive(Debug)]
struct Passport {
    // byr (Birth Year)
    birth_year: i64,
    // iyr (Issue Year)
    issue_year: i64,
    // eyr (Expiration Year)
    expiration_year: i64,
    // hgt (Height)
    height: String,
    // hcl (Hair Color)
    hair_color: String,
    // ecl (Eye Color)
    eye_color: String,
    // pid (Passport ID)
    passport_id: String,
    // cid (Country ID)
    country_id: String,
}

impl Passport {
    fn new() -> Passport {
        return Passport {
            birth_year: -1,
            issue_year: -1,
            expiration_year: -1,
            height: "".to_string(),
            hair_color: "".to_string(),
            eye_color: "".to_string(),
            passport_id: "".to_string(),
            country_id: "".to_string(),
        };
    }

    // valid checks if the Passport is
    // allowed. A Passport is valid if every
    // field is present, with the exception of
    // the country_id.
    fn valid(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // hgt (Height) - a number followed by either cm or in:
        //  If cm, the number must be at least 150 and at most 193.
        //  If in, the number must be at least 59 and at most 76.
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.
        let height_reg: Regex =
            Regex::new(r"^((59|6[0-9]|7[0-6]){1}in|1([5-8][0-9]|9[0-3]){1}cm)$").unwrap();
        let hair_color_reg: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let eye_color_reg: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth){1}$").unwrap();
        let passport_id_reg: Regex = Regex::new(r"^[0-9]{9}$").unwrap();

        return self.birth_year >= 1920
            && self.birth_year <= 2002
            && self.issue_year >= 2010
            && self.issue_year <= 2020
            && self.expiration_year >= 2020
            && self.expiration_year <= 2030
            && height_reg.is_match(&self.height)
            && hair_color_reg.is_match(&self.hair_color)
            && eye_color_reg.is_match(&self.eye_color)
            && passport_id_reg.is_match(&self.passport_id);
        // The country_id is optional
    }
}

// Implement fmt::Display, this needs to be done
// to allow for the Passport to be displayed.
impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Passport:\n\tBirth Year: {}\n\tIssue Year: {}\n\tExpiration Year: {}\n\tHeight: {}\n\tHair Color: {}\n\tEye Color: {}\n\tPassport ID: {}\n\tCountry ID: {}",
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id,
            self.country_id,
        )
    }
}

// parse_passports takes in input lines and
// creates a Vector of Passport structs. The
// list of structs may contain invalid Passports.
fn parse_passports(lines: &Vec<String>) -> Result<Vec<Passport>, Box<dyn std::error::Error>> {
    let mut passports: Vec<Passport> = Vec::new();

    let mut current_passport: Passport = Passport::new();
    for line in lines.iter() {
        // Blank line so move onto to the next
        // passport.
        if line == "" {
            passports.push(current_passport);
            current_passport = Passport::new();
            continue;
        }

        // Go through each field in the passport.
        let fields: Vec<String> = line.trim().split_whitespace().map(String::from).collect();
        for field in fields.iter() {
            let values: Vec<String> = field.trim().split(":").map(String::from).collect();
            if values.len() != 2 {
                continue;
            }

            // Attempt to read in each field into the
            // current passport.
            match values[0].as_str() {
                "byr" => current_passport.birth_year = values[1].parse::<i64>()?,
                "iyr" => current_passport.issue_year = values[1].parse::<i64>()?,
                "eyr" => current_passport.expiration_year = values[1].parse::<i64>()?,
                "hgt" => current_passport.height = values[1].clone(),
                "hcl" => current_passport.hair_color = values[1].clone(),
                "ecl" => current_passport.eye_color = values[1].clone(),
                "pid" => current_passport.passport_id = values[1].clone(),
                "cid" => current_passport.country_id = values[1].clone(),
                _ => continue,
            }
        }
    }

    // Push the last passport
    passports.push(current_passport);

    Ok(passports)
}

// solve attempts to solve part 2 of day 4 of AoC 2020.
// After creating a list of Passports from the input, each
// one is checked to see if it is valid, a tally of how many
// are is kept and returned.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut valid_passport_count: i32 = 0;

    let passports: Vec<Passport> = parse_passports(lines)?;

    for passport in passports.iter() {
        if passport.valid() {
            valid_passport_count = valid_passport_count + 1;
        }
    }

    Ok(valid_passport_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: String = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            .to_string();

        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 2);
        Ok(())
    }
}
