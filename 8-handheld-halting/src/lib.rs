use std::convert::TryFrom;

#[derive(Clone)]
pub enum Command {
    Nop,
    Acc,
    Jump,
}

#[derive(Clone)]
pub struct Instruction {
    cmd: Command,
    arg: i64,
}

impl Instruction {
    fn flip(&mut self) {
        self.cmd = match &self.cmd {
            Command::Nop => Command::Jump,
            Command::Jump => Command::Nop,
            x => x.clone(),
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum ExitStatus {
    LoopDetected(i64),
    Exited(i64),
}

fn compile(input: &str) -> Vec<Instruction> {
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

pub fn run(instructions: &Vec<Instruction>) -> ExitStatus {
    let mut current_acc = 0;
    let mut current_line_number: i64 = 0;
    let mut visited_line_numbers = vec![];

    while !visited_line_numbers.contains(&current_line_number) {
        visited_line_numbers.push(current_line_number);

        let current_instruction = if let Ok(l) = usize::try_from(current_line_number) {
            if l == instructions.len() {
                // We have reached the end of the program and are exiting gracefully
                return ExitStatus::Exited(current_acc);
            }

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

    ExitStatus::LoopDetected(current_acc)
}

pub fn find_flip(input: &str) -> Option<ExitStatus> {
    let instructions = compile(input);

    let maybe_flip_index = (0..instructions.len())
        .map(|i| {
            let mut new_instructions = instructions.to_owned();
            new_instructions[i].flip();
            (i, new_instructions)
        })
        .find(|(_, instructions)| match run(instructions) {
            ExitStatus::Exited(_) => true,
            _ => false,
        });

    let mut last_flip = instructions.to_owned();
    last_flip[maybe_flip_index?.0].flip();

    Some(run(&last_flip))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_a_sample_works() {
        assert_eq!(
            run(&compile(include_str!("sample.input"))),
            ExitStatus::LoopDetected(5)
        );
    }

    #[test]
    fn part_a_input_works() {
        assert_eq!(
            run(&compile(include_str!("my.input"))),
            ExitStatus::LoopDetected(1584)
        );
    }

    #[test]
    fn part_b_sample_works() {
        assert_eq!(
            find_flip(include_str!("sample.input")),
            Some(ExitStatus::Exited(8))
        );
    }

    #[test]
    fn part_b_input_works() {
        assert_eq!(
            find_flip(include_str!("my.input")),
            Some(ExitStatus::Exited(920))
        );
    }
}
