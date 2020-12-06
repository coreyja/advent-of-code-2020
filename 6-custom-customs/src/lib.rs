use std::collections::HashSet;

fn count_number_of_yes_questions(group: &str) -> usize {
    let mut yes_questions = HashSet::new();

    let all_answers = group.to_owned().replace('\n', "");
    all_answers.chars().for_each(|l| {
        println!("");
        yes_questions.insert(l);
    });

    yes_questions.len()
}

pub fn sum_question(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|x| count_number_of_yes_questions(x))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_groups_work() {
        assert_eq!(count_number_of_yes_questions(""), 0);
        assert_eq!(count_number_of_yes_questions("abc"), 3);
        assert_eq!(
            count_number_of_yes_questions(
                "ab
ac"
            ),
            3
        );
    }

    #[test]
    fn part_a_works_for_sample_input() {
        assert_eq!(sum_question(include_str!("sample.input")), 11)
    }
}
