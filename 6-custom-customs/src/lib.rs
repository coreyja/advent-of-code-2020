use std::collections::HashSet;

fn count_number_of_any_yes_questions(group: &str) -> usize {
    let mut yes_questions = HashSet::new();

    let all_answers = group.to_owned().replace('\n', "");
    all_answers.chars().for_each(|l| {
        println!("");
        yes_questions.insert(l);
    });

    yes_questions.len()
}

pub fn sum_any_yes_questions(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|x| count_number_of_any_yes_questions(x))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_groups_work_part_a() {
        assert_eq!(count_number_of_any_yes_questions(""), 0);
        assert_eq!(count_number_of_any_yes_questions("abc"), 3);
        assert_eq!(
            count_number_of_any_yes_questions(
                "ab
ac"
            ),
            3
        );
    }

    #[test]
    fn part_a_works_for_sample_input() {
        assert_eq!(sum_any_yes_questions(include_str!("sample.input")), 11)
    }

    #[test]
    fn part_a_works_for_my_input() {
        assert_eq!(sum_any_yes_questions(include_str!("my.input")), 6735)
    }
}
