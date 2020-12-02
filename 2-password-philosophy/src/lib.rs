pub fn count_valid_passwords_part_a(input: &str) -> usize {
    input
        .trim()
        .split("\n")
        .filter(|&x| is_valid_password_part_a(x))
        .count()
}

pub fn count_valid_passwords_part_b(input: &str) -> usize {
    input
        .trim()
        .split("\n")
        .filter(|&x| is_valid_password_part_b(x))
        .count()
}

struct PasswordLine {
    num_1: usize,
    num_2: usize,
    c: char,
    password: String,
}

fn parse_line(input: &str) -> PasswordLine {
    let mut i = input.split(":");
    let first_half = i.next().unwrap().trim();
    let mut first_half_i = first_half.split(" ");
    let range = first_half_i.next().unwrap();
    let mut range_i = range.split("-");
    let min: usize = range_i.next().unwrap().parse().expect("min");
    let max: usize = range_i.next().unwrap().parse().expect("max");

    let char_to_count = first_half_i.next().unwrap().chars().next().unwrap();

    let password = i.next().unwrap().trim();

    PasswordLine {
        num_1: min,
        num_2: max,
        c: char_to_count,
        password: password.to_owned(),
    }
}

fn is_valid_password_part_a(input: &str) -> bool {
    let parsed = parse_line(input);

    let count = parsed.password.chars().filter(|&x| x == parsed.c).count();

    count >= parsed.num_1 && count <= parsed.num_2
}

fn is_valid_password_part_b(input: &str) -> bool {
    let parsed = parse_line(input);

    let pass_chars: Vec<_> = parsed.password.chars().collect();

    (pass_chars[parsed.num_1 - 1] == parsed.c) ^ (pass_chars[parsed.num_2 - 1] == parsed.c)
}

#[cfg(test)]
mod tests {
    use crate::{count_valid_passwords_part_a, count_valid_passwords_part_b};

    #[test]
    fn it_works_for_the_sample_input() {
        assert_eq!(
            count_valid_passwords_part_a(include_str!("sample.input")),
            2
        );
    }

    #[test]
    fn it_works_for_my_input() {
        assert_eq!(count_valid_passwords_part_a(include_str!("my.input")), 477);
    }

    #[test]
    fn part_b_it_works_for_the_sample_input() {
        assert_eq!(
            count_valid_passwords_part_b(include_str!("sample.input")),
            1
        );
    }

    #[test]
    fn part_b_it_works_for_my_input() {
        assert_eq!(count_valid_passwords_part_b(include_str!("my.input")), 686);
    }
}
