pub fn boarding_id_from_str(input: &str) -> u32 {
    let (row_string, column_string) = input.split_at(7);

    let row = binary_num(&row_string, 'F', 'B');
    let column = binary_num(&column_string, 'L', 'R');

    row * 8 + column
}

pub fn binary_num(s: &str, low_char: char, high_char: char) -> u32 {
    let mut string = s.to_owned();

    string = string.replace(low_char, "0");
    string = string.replace(high_char, "1");

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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn binary_string_to_num_works() {
        assert_eq!(binary_num("BFFFBBF", 'F', 'B'), 70);
        assert_eq!(binary_num("RRR", 'L', 'R'), 7);
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
}
