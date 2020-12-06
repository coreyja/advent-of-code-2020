pub fn boarding_id_from_str(input: &str) -> u32 {
    binary_num_vec(input, vec!['F', 'L'], vec!['R', 'B'])
}

fn binary_num_vec(s: &str, low_chars: Vec<char>, high_chars: Vec<char>) -> u32 {
    let mut string = s.to_owned();

    low_chars
        .iter()
        .for_each(|&low_char| string = string.replace(low_char, "0"));
    high_chars
        .iter()
        .for_each(|&high_char| string = string.replace(high_char, "1"));

    u32::from_str_radix(&string, 2).unwrap()
}

pub fn find_max_boarding_pass(input: &str) -> u32 {
    input
        .trim()
        .split("\n")
        .map(|x| boarding_id_from_str(x))
        .max()
        .unwrap()
}

fn missing_ids(input: &str, max: u32) -> Vec<u32> {
    let found_ids: Vec<_> = input
        .trim()
        .split("\n")
        .map(|x| boarding_id_from_str(x))
        .collect();

    (0..max).filter(|x| !found_ids.contains(x)).collect()
}

pub fn find_my_id(input: &str) -> Option<u32> {
    let mut missing = missing_ids(input, 1023).into_iter();
    let mut max = missing.next().unwrap().clone();

    missing.find(|&x| {
        let b = max + 1 == x;
        max = max + 1;
        !b
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn binary_string_to_num_works() {
        assert_eq!(
            binary_num_vec("BFFFBBFRRR", vec!['F', 'L'], vec!['R', 'B']),
            567
        );
    }

    #[test]
    fn sample_passes_work() {
        assert_eq!(boarding_id_from_str("BFFFBBFRRR"), 567);
        assert_eq!(boarding_id_from_str("FFFBBBFRRR"), 119);
        assert_eq!(boarding_id_from_str("BBFFBBFRLL"), 820);
    }

    #[test]
    fn finds_the_max_in_the_sample() {
        assert_eq!(find_max_boarding_pass(include_str!("sample.input")), 820);
    }

    #[test]
    fn finds_the_max_in_my_input() {
        assert_eq!(find_max_boarding_pass(include_str!("my.input")), 835);
    }

    #[test]
    fn finds_missing_in_my_input() {
        assert_eq!(find_my_id(include_str!("my.input")), Some(649));
    }
}
