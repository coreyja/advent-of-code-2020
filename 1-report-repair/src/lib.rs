pub fn find_pair(input: &str) -> i32 {
    find_next(parse_input(input), 2020, 2).unwrap()
}

pub fn find_triplet(input: &str) -> i32 {
    find_next(parse_input(input), 2020, 3).unwrap()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn find_next(input: Vec<i32>, target: i32, count: i32) -> Option<i32> {
    // This is out recursion base case. Once we are down to count == 1 we are only look for a
    // single number 'set' so its now an equality check with the target
    // With count == 1 the product is also just equal to the number (as if we multiplied by 1)
    if count == 1 {
        return input.into_iter().find(|&x| x == target);
    }

    input
        .clone()
        .into_iter()
        .filter_map(|x| {
            // We map over the input and create a sub-problem for recursion
            // Our new target is made by subtracting the current from the previous target. This is
            // the inverse of adding up to a target, and allows us to pass less things down the
            // recursion
            // We also reduce the count (the number of recursions possible)
            let next = find_next(input.clone(), target - x, count - 1);

            // If we found a result we need to return the ans to the subproblem * the current value
            if let Some(ans) = next {
                Some(x * ans)
            } else {
                None
            }
        })
        .next()
}

#[cfg(test)]
mod tests {
    use crate::{find_pair, find_triplet};

    #[test]
    fn it_works_for_the_sample_input() {
        assert_eq!(find_pair(include_str!("sample.input")), 514579);
    }

    #[test]
    fn it_works_for_the_given_input() {
        assert_eq!(find_pair(include_str!("my.input")), 776064);
    }

    #[test]
    fn it_works_for_the_part_b_sample_input() {
        assert_eq!(find_triplet(include_str!("sample.input")), 241861950);
    }

    #[test]
    fn it_works_for_the_part_b_given_input() {
        assert_eq!(find_triplet(include_str!("my.input")), 6964490);
    }
}
