use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type ContainedBag = (usize, String);
type ContainsMap = HashMap<String, Vec<ContainedBag>>;
type ContainedByMap = HashMap<String, Vec<String>>;

pub fn count_outer_bags(input: &str, bag: &str) -> usize {
    let map = reverse_map(parse_input(input));

    let mut visited_bags: HashSet<String> = HashSet::new();
    let mut todo = vec![bag.to_owned()];

    while todo.len() > 0 {
        let curr = todo
            .pop()
            .expect("We check the length before we get in here");

        if let Some(v) = map.get(&curr).cloned() {
            v.iter().for_each(|x| {
                todo.push(x.to_owned());
            });
        }

        visited_bags.insert(curr.to_owned());
    }

    // We don't want to include 'bag' but its here so we subtract one
    visited_bags.len() - 1
}

pub fn count_inner_bags(input: &str, bag: &str) -> usize {
    let map = parse_input(input);

    let mut visited_bags = vec![];
    let mut todo = vec![bag.to_owned()];

    while todo.len() > 0 {
        let curr = todo
            .pop()
            .expect("We check the length before we get in here");

        if let Some(v) = map.get(&curr).cloned() {
            v.iter().for_each(|(count, name)| {
                std::iter::repeat(name)
                    .take(*count)
                    .for_each(|x| todo.push(x.to_owned()));
            });
        }

        visited_bags.push(curr.to_owned());
    }

    // We don't want to include 'bag' but its here so we subtract one
    visited_bags.len() - 1
}

fn parse_input(input: &str) -> ContainsMap {
    let mut map = HashMap::new();

    input
        .trim()
        .lines()
        .for_each(|line| build_map_for_line(&mut map, line));

    map
}

fn reverse_map(old_map: ContainsMap) -> ContainedByMap {
    let mut new_map: ContainedByMap = HashMap::new();

    old_map.iter().for_each(|(key, contained_bags)| {
        contained_bags.iter().for_each(|(_, name)| {
            if let Some(current_vec) = new_map.get_mut(name) {
                current_vec.push(key.to_owned());
            } else {
                new_map.insert(name.to_owned(), vec![key.to_owned()]);
            }
        });
    });

    new_map
}

fn build_map_for_line(map: &mut ContainsMap, line: &str) {
    if line.contains("contain no other bags") {
        return;
    }

    let mut x = line.split("contain");
    let original = x
        .next()
        .expect("contain first half")
        .split("bags")
        .next()
        .unwrap()
        .trim();
    let second_half = x.next().expect("contain second half").replace(".", "");

    let contained_bags = second_half.split(",").map(str::trim);

    let regex = Regex::new(r"(?P<num>\d+) (?P<name>.*) bags?")
        .expect("Regex literal for contained bags is valid");

    let transformed_bags = contained_bags
        .map(|contained_bag_line| {
            let caps = regex.captures(contained_bag_line).unwrap();
            let bag_name = caps.name("name").unwrap().as_str().to_owned();
            let bag_count = caps.name("num").unwrap().as_str().parse().unwrap();

            (bag_count, bag_name)
        })
        .collect();

    map.insert(original.to_owned(), transformed_bags);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_a_sample_works() {
        assert_eq!(
            count_outer_bags(include_str!("sample.input"), "shiny gold"),
            4
        );
    }

    #[test]
    fn part_a_my_input_works() {
        assert_eq!(
            count_outer_bags(include_str!("my.input"), "shiny gold"),
            197
        );
    }

    #[test]
    fn part_b_new_sample_works() {
        assert_eq!(
            count_inner_bags(include_str!("new_sample.input"), "shiny gold"),
            126
        );
    }
}
