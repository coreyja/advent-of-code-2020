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
        true
    }

    fn iyr_valid(&self) -> bool {
        true
    }

    fn eyr_valid(&self) -> bool {
        true
    }

    fn hgt_valid(&self) -> bool {
        true
    }

    fn hcl_valid(&self) -> bool {
        true
    }

    fn ecl_valid(&self) -> bool {
        true
    }

    fn pid_valid(&self) -> bool {
        true
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

#[cfg(test)]
mod tests {
    use crate::count_passports_with_all_fields;

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
}
