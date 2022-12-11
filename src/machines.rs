use std::collections::VecDeque;

use crate::instructions::Instruction;

/// A virtual machine executes a sequence of `Instruction`s (i.e. a program). It
/// maintains the value of a single register. Since some instructions take
/// longer to execute, it separates the program instructions from those
/// in-flight.
#[derive(Debug)]
pub struct VirtualMachine {
    /// The program is a sequence of instructions that will be executed
    /// sequentially.
    program: VecDeque<Instruction>,

    /// An in-flight instruction is currently executing
    in_flight: Option<Instruction>,

    /// The single register used in this VM. It is initially `1`.
    register: isize,

    /// Stores how many cycles this VM has executed. It is initially `0` and
    /// increases by one every time the CPU cycles (i.e. `cycle()` is called).
    ticks: usize,
}

impl VirtualMachine {
    pub fn new(program: VecDeque<Instruction>) -> Self {
        let in_flight = None;

        // Start at tick one then increment after completing a cycle.
        //
        // TODO This problem begs for property-directed testing!
        let ticks = 1;

        VirtualMachine {
            program,
            in_flight,
            ticks,
            register: 1, // Initially `1` by specification
        }
    }

    /// Returns `false` when the program has finished executing (i.e. all
    /// instructions) have completed.
    pub fn is_executing(&self) -> bool {
        !self.program.is_empty() || !self.in_flight.is_none()
    }

    /// Return the value currently stored in the register. When instructions
    /// that modify this value (e.g. `Addx`) execute, the value is only updated
    /// after the instruction completes, at the end of the CPU cycle.
    pub fn read_register(&self) -> isize {
        self.register
    }

    /// Returns the number of cycles performed by the CPU
    pub fn get_ticks(&self) -> usize {
        self.ticks
    }

    /// Cycles the CPU by executing the next instruction. This will increase
    /// the cycle counter and possibly the register (depending on the
    /// instruction).
    ///
    /// NB the cycle counter (i.e. `ticks`) is incremented only after the cycle
    /// is complete.
    pub fn cycle(&mut self) {
        if self.in_flight.is_none() {
            self.schedule();
        } else {
            self.execute();
        }

        self.ticks += 1;
    }

    /// An instruction is currently executing. In this architecture, that means
    /// an `addx` instruction was scheduled on the previous cycle. Since `addx`
    /// takes two cycles it can be completed on this cycle.
    ///
    /// If a `Noop` was scheduled, ignore it. The VM sets a `Noop` instruction
    /// as in-flight when starting so `Addx` doesn't execute too fast.
    ///
    /// This pattern would require refactoring if more instructions with varying
    /// execution lengths are added.
    fn execute(&mut self) {
        let instruction = self.in_flight.unwrap();

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(number) => self.register += number,
        }

        self.in_flight = None;
    }

    /// No instructions are currently executing. Pull the next one from the
    /// program and execute or schedule it depending on the type.
    fn schedule(&mut self) {
        let instruction = self.program.pop_front();
        let instruction = instruction.unwrap();

        // `Noop` instructions take a single cycle to execute and have no side
        // effects. Adding takes two cycles, so the instruction is scheduled to
        // complete on the next cycle.
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(_) => self.in_flight = Some(instruction),
        }
    }
}
