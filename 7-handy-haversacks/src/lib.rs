use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn count_outer_bags(input: &str, bag: &str) -> usize {
    let map = parse_input(input);

    let mut visited_bags: HashSet<String> = HashSet::new();
    let mut todo = vec![bag.to_owned()];

    while todo.len() > 0 {
        let curr = todo
            .pop()
            .expect("We check the length before we get in here");

        if let Some(v) = map.get(&curr).cloned() {
            v.iter().for_each(|x| {
                todo.push(x.to_owned());
                visited_bags.insert(x.to_owned());
            });
        }

        visited_bags.insert(curr.to_owned());
    }

    // We don't want to include 'bag' but its here so we subtract one
    visited_bags.len() - 1
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    input
        .trim()
        .lines()
        .for_each(|line| process_line(&mut map, line));

    map
}

fn process_line(map: &mut HashMap<String, Vec<String>>, line: &str) {
    // Nothing to do in this case so just return
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

    let regex =
        Regex::new(r"\d+ (?P<name>.*) bags?").expect("Regex literal for contained bags is valid");

    contained_bags.for_each(|contained_bag_line| {
        let caps = regex.captures(contained_bag_line).unwrap();
        let bag_name = caps.name("name").unwrap().as_str().to_owned();

        if let Some(current_vec) = map.get_mut(&bag_name) {
            current_vec.push(original.to_owned());
        } else {
            map.insert(bag_name, vec![original.to_owned()]);
        }
    })
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
}
