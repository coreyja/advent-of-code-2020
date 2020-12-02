pub fn count_valid_passwords(input: &str) -> usize {
    input
        .trim()
        .split("\n")
        .filter(|&x| is_valid_password(x))
        .count()
}

fn is_valid_password(input: &str) -> bool {
    let mut i = input.split(":");
    let first_half = i.next().unwrap().trim();
    let mut first_half_i = first_half.split(" ");
    let range = first_half_i.next().unwrap();
    let mut range_i = range.split("-");
    let min: usize = range_i.next().unwrap().parse().expect("min");
    let max: usize = range_i.next().unwrap().parse().expect("max");

    let char_to_count = first_half_i.next().unwrap().chars().next().unwrap();

    let password = i.next().unwrap().trim();

    let count = password.chars().filter(|&x| x == char_to_count).count();

    count >= min && count <= max
}

#[cfg(test)]
mod tests {
    use crate::count_valid_passwords;

    #[test]
    fn it_works_for_the_sample_input() {
        assert_eq!(count_valid_passwords(include_str!("sample.input")), 2);
    }
}
