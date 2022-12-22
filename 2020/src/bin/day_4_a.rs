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

    fn valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
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
        .filter(|passport| passport.valid())
        .collect();
    let invalid_passports: Vec<&Passport> = passports
        .iter()
        .filter(|passport| !passport.valid())
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
