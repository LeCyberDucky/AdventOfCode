use anyhow::Result;
use std::fs::read_to_string;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new(input: &str) -> Passport {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        let input: Vec<String> = input
            .split([':', ' '].as_ref())
            .map(|s| s.to_string())
            .collect();

        if let Some(index) = input.iter().position(|element| *element == "byr") {
            if index + 1 < input.len() {
                passport.byr = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "iyr") {
            if index + 1 < input.len() {
                passport.iyr = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "eyr") {
            if index + 1 < input.len() {
                passport.eyr = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "hgt") {
            if index + 1 < input.len() {
                passport.hgt = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "hcl") {
            if index + 1 < input.len() {
                passport.hcl = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "ecl") {
            if index + 1 < input.len() {
                passport.ecl = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "pid") {
            if index + 1 < input.len() {
                passport.pid = Some(input[index + 1].clone());
            }
        }

        if let Some(index) = input.iter().position(|element| *element == "cid") {
            if index + 1 < input.len() {
                passport.cid = Some(input[index + 1].clone());
            }
        }

        passport
    }

    fn valid(&self) -> Result<bool> {
        let mut valid = self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();

        if let Some(byr) = &self.byr {
            valid &= byr.len() == 4;

            let byr: usize = byr.parse()?;

            valid &= byr >= 1920;
            valid &= byr <= 2002;
        }

        if let Some(iyr) = &self.iyr {
            valid &= iyr.len() == 4;

            let iyr: usize = iyr.parse()?;

            valid &= iyr >= 2010;
            valid &= iyr <= 2020;
        }

        if let Some(eyr) = &self.eyr {
            valid &= eyr.len() == 4;

            let eyr: usize = eyr.parse()?;

            valid &= eyr >= 2020;
            valid &= eyr <= 2030;
        }

        if let Some(hgt) = &self.hgt {
            if hgt.len() >= 4 {
                match &hgt[hgt.len() - 2..hgt.len()] {
                    "cm" => {
                        let num: usize = hgt[..hgt.len() - 2].parse()?;
                        valid &= num >= 150 && num <= 193;
                    }
                    "in" => {
                        let num: usize = hgt[..hgt.len() - 2].parse()?;
                        valid &= num >= 59 && num <= 76;
                    }
                    _ => return Ok(false),
                }
            } else {
                return Ok(false);
            }
        }

        if let Some(hcl) = &self.hcl {
            if hcl.len() != 7 {
                return Ok(false);
            }

            valid &= hcl.chars().nth(0) == Some('#');

            for character in hcl[1..7].chars() {
                valid &= match character {
                    '0'..='9' | 'a'..='f' => true,
                    _ => false,
                }
            }
        }

        if let Some(ecl) = &self.ecl {
            let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            valid &= colors.contains(&ecl.as_ref());
        }

        if let Some(pid) = &self.pid {
            valid &= pid.len() == 9;
            for character in pid.chars() {
                valid &= character.is_ascii_digit();
            }
        }

        Ok(valid)
    }
}

fn main() -> Result<()> {
    let path = r"Day 4.txt";

    let input = read_to_string(path)?;
    let input: Vec<String> = input
        .split("\r\n\r\n")
        .map(|s| s.to_string().replace("\r\n", " "))
        .collect();

    let passports: Vec<Passport> = input
        .iter()
        .map(|input_string| Passport::new(input_string))
        .collect();

    let valid_passports: Vec<&Passport> = passports
        .iter()
        .filter_map(|passport| match passport.valid() {
            Ok(true) => Some(passport),
            _ => None,
        })
        .collect();
    let invalid_passports: Vec<&Passport> = passports
        .iter()
        .filter_map(|passport| match passport.valid() {
            Ok(true) => None,
            _ => Some(passport),
        })
        .collect();

    let valid_passport_count = valid_passports.iter().count();
    let invalid_passport_count = invalid_passports.iter().count();

    println!("Number of passports: {}", passports.iter().count());
    println!("Number of invalid passports: {}", invalid_passport_count);
    println!("Number of valid passports: {}", valid_passport_count);

    // for passport in passports.iter() {
    //     if passport.valid() {
    //         print!("✔: ");
    //     } else {
    //         print!("❌: ");
    //     }

    //     println!("{:?}\n", passport);
    // }

    // println!("{:#?}", valid_passports);

    Ok(())
}
