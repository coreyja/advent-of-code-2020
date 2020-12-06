type Passport<'a> = std::collections::HashMap<&'a str, &'a str>;

pub fn count_valid_passports(input: &str, required_keys: &Vec<&str>) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|x| parse_passport(x))
        .filter(|x| valid_password(x, required_keys))
        .count()
}

fn valid_password(passport: &Passport, required_keys: &Vec<&str>) -> bool {
    required_keys.iter().all(|key| passport.contains_key(key))
}

fn parse_passport(input: &str) -> Passport {
    let mut passport = Passport::new();

    input.split_whitespace().for_each(|x| {
        let mut iter = x.split(":");

        let key = iter.next().expect("key to exist");
        let value = iter.next().expect("value to exist");

        passport.insert(key, value);
    });

    passport
}

#[cfg(test)]
mod tests {
    use crate::count_valid_passports;

    #[test]
    fn part_a_works_for_sample_input() {
        assert_eq!(
            count_valid_passports(
                include_str!("sample.input"),
                &vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            ),
            2
        );
    }

    #[test]
    fn part_a_works_for_my_input() {
        assert_eq!(
            count_valid_passports(
                include_str!("my.input"),
                &vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            ),
            222
        );
    }
}
