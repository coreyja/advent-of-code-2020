pub fn count_trees(input: &str, route: (usize, usize)) -> u64 {
    let lines: Vec<_> = input.trim().split("\n").collect();
    let row_length = lines.get(0).unwrap().chars().count();
    let column_length = lines.len();

    let mut curr = (0, 0);
    let mut count = 0;

    while curr.1 < column_length {
        let chars: Vec<_> = lines.get(curr.1).unwrap().chars().collect();
        if *chars.get(curr.0).unwrap() == '#' {
            count = count + 1;
        }

        curr = (curr.0 + route.0, curr.1 + route.1);
        curr = (curr.0 % row_length, curr.1);
    }

    count
}

pub fn check_multiple_paths(input: &str, routes: Vec<(usize, usize)>) -> u64 {
    routes
        .into_iter()
        .map(|x| count_trees(input, x))
        .fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use crate::{check_multiple_paths, count_trees};

    #[test]
    fn it_works_for_the_sample_input() {
        assert_eq!(count_trees(include_str!("sample.input"), (3, 1)), 7);
    }

    #[test]
    fn it_works_for_my_input() {
        assert_eq!(count_trees(include_str!("my.input"), (3, 1)), 234);
    }

    #[test]
    fn part_b_works_for_the_sample_input() {
        assert_eq!(
            check_multiple_paths(
                include_str!("sample.input"),
                vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            ),
            336
        );
    }

    #[test]
    fn part_b_works_for_my_input() {
        assert_eq!(
            check_multiple_paths(
                include_str!("my.input"),
                vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            ),
            5813773056
        );
    }
}
