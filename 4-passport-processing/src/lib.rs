use eyre::{eyre, Result};
use regex::Regex;
use std::str::FromStr;

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
    pub fn valid(&self) -> bool {
        self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }

    pub fn has_all_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn byr_valid(&self) -> bool {
        if let Some(v) = &self.byr {
            let parsed: Result<u32, _> = v.parse();

            parsed.ok().filter(|&x| x >= 1920 && x <= 2002).is_some()
        } else {
            false
        }
    }

    fn iyr_valid(&self) -> bool {
        if let Some(v) = &self.iyr {
            let parsed: Result<u32, _> = v.parse();

            parsed.ok().filter(|&x| x >= 2010 && x <= 2020).is_some()
        } else {
            false
        }
    }

    fn eyr_valid(&self) -> bool {
        if let Some(v) = &self.eyr {
            let parsed: Result<u32, _> = v.parse();

            parsed.ok().filter(|&x| x >= 2020 && x <= 2030).is_some()
        } else {
            false
        }
    }

    fn errorable_in_hgt_valid(&self) -> Result<bool> {
        let hgt = self.hgt.as_ref().ok_or_else(|| eyre!("No HGT"))?;
        let in_regex = Regex::new(r"^(\d*)in$").expect("regex is valid");

        let in_match = in_regex
            .captures(hgt)
            .ok_or_else(|| eyre!("No match"))?
            .get(1)
            .ok_or_else(|| eyre!("No match"))?
            .as_str();

        let inches: u32 = in_match.parse()?;

        Ok(inches >= 59 && inches <= 76)
    }

    fn errorable_cm_hgt_valid(&self) -> Result<bool> {
        let hgt = self.hgt.as_ref().ok_or_else(|| eyre!("No HGT"))?;
        let regex = Regex::new(r"^(\d*)cm$").expect("regex is valid");

        let m = regex
            .captures(hgt)
            .ok_or_else(|| eyre!("No match"))?
            .get(1)
            .ok_or_else(|| eyre!("No match"))?
            .as_str();

        let val: u32 = m.parse()?;

        Ok(val >= 150 && val <= 193)
    }

    fn hgt_valid(&self) -> bool {
        self.errorable_in_hgt_valid().ok().filter(|&x| x).is_some()
            || self.errorable_cm_hgt_valid().ok().filter(|&x| x).is_some()
    }

    fn hcl_valid(&self) -> bool {
        self.hcl
            .as_ref()
            .filter(|x| {
                let regex = Regex::new(r"^#[\da-z]{6}").expect("regex is valid");

                regex.is_match(x)
            })
            .is_some()
    }

    fn ecl_valid(&self) -> bool {
        let valid_ecls: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        self.ecl
            .as_ref()
            .filter(|&ecl| valid_ecls.contains(&ecl.as_ref()))
            .is_some()
    }

    fn pid_valid(&self) -> bool {
        self.pid
            .as_ref()
            .filter(|x| {
                let regex = Regex::new(r"^\d{9}$").expect("regex is valid");

                regex.is_match(x)
            })
            .is_some()
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn from_map(hash: &std::collections::HashMap<&str, &str>, key: &str) -> Option<String> {
            hash.get(key).map(|&x| x.to_owned())
        }

        let mut hash = std::collections::HashMap::new();

        input.split_whitespace().for_each(|x| {
            let mut iter = x.split(":");

            let key = iter.next().expect("key to exist");
            let value = iter.next().expect("value to exist");

            hash.insert(key, value);
        });

        Ok(Self {
            byr: from_map(&hash, "byr"),
            iyr: from_map(&hash, "iyr"),
            eyr: from_map(&hash, "eyr"),
            hgt: from_map(&hash, "hgt"),
            hcl: from_map(&hash, "hcl"),
            ecl: from_map(&hash, "ecl"),
            pid: from_map(&hash, "pid"),
            cid: from_map(&hash, "cid"),
        })
    }
}

pub fn count_passports_with_all_fields(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|x| Passport::from_str(x).unwrap())
        .filter(|x| x.has_all_required_fields())
        .count()
}

pub fn count_valid_passports(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|x| Passport::from_str(x).unwrap())
        .filter(|x| x.valid())
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{count_passports_with_all_fields, count_valid_passports};

    #[test]
    fn part_a_works_for_sample_input() {
        assert_eq!(
            count_passports_with_all_fields(include_str!("sample.input"),),
            2
        );
    }

    #[test]
    fn part_a_works_for_my_input() {
        assert_eq!(
            count_passports_with_all_fields(include_str!("my.input"),),
            222
        );
    }

    #[test]
    fn part_b_works_for_my_input() {
        assert_eq!(count_valid_passports(include_str!("my.input"),), 140);
    }
}
