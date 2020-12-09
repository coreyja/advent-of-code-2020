enum Command {
    Nop,
    Acc,
    Jump,
}

struct Instruction {
    cmd: Command,
    arg: i64,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|x| {
            let mut split_iter = x.split(" ");
            let cmd_str = split_iter.next().unwrap();

            let arg: i64 = split_iter.next().unwrap().parse().unwrap();

            let cmd = match cmd_str {
                "acc" => Command::Acc,
                "nop" => Command::Nop,
                "jmp" => Command::Jump,
                _ => panic!("Got an unknown command string"),
            };

            Instruction { cmd, arg }
        })
        .collect()
}

use std::convert::TryFrom;

pub fn run_until_loop(input: &str) -> i64 {
    let mut current_acc = 0;
    let mut current_line_number: i64 = 0;
    let mut visited_line_numbers = vec![];

    let instructions = parse_input(input);

    while !visited_line_numbers.contains(&current_line_number) {
        visited_line_numbers.push(current_line_number);

        let current_instruction = if let Ok(l) = usize::try_from(current_line_number) {
            instructions
                .get(l)
                .expect("Code jumped to a non existant place")
        } else {
            panic!("We jumped to something we can't index into. Likely something negative");
        };

        match current_instruction.cmd {
            Command::Acc => {
                current_acc = current_acc + current_instruction.arg;
                current_line_number = current_line_number + 1;
            }
            Command::Jump => current_line_number = current_line_number + current_instruction.arg,
            Command::Nop => current_line_number = current_line_number + 1,
        }
    }

    current_acc
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_a_sample_works() {
        assert_eq!(run_until_loop(include_str!("sample.input")), 5);
    }

    #[test]
    fn part_a_input_works() {
        assert_eq!(run_until_loop(include_str!("my.input")), 0);
    }
}
