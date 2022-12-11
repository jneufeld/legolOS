use std::collections::VecDeque;

/// The problem's input is well formatted. Every line contains one instruction.
pub fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    let mut instructions = VecDeque::new();

    for line in input.split('\n') {
        instructions.push_back(Instruction::from(line));
    }

    instructions
}

/// This machine has a myriad of options: add with one operand or do nothing.
///
/// NB this is implicitly coupled to the machine's implementation of scheduling.
/// A `Noop` takes a single CPU cycle to complete, but `Addx` takes two. Neither
/// is captured here.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        // The noop instruction is the simplest. Parse it first without bother.
        if s.starts_with("noop") {
            return Instruction::Noop;
        }

        // Add instructions always start with `addx` followed by a space
        // followed by the value (operand). Split at that index and ignore the
        // first portion.
        let (_addx, number) = s.split_at(5);

        let number = number
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("Can't parse isize from {}", number));

        Instruction::Addx(number)
    }
}