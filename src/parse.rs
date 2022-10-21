use crate::def;
use crate::parse::Instruction::*;

type Addr = u8;
type Int = u8;

enum Instruction {
    Noop,
    MovA(Addr, Addr),
    MovB(Addr, Int),
    Add(Addr, Addr),
    Sub(Addr, Addr),
    Jmp(Addr),
    Jnz(Addr, Addr),
}

impl Instruction {
    fn add_to_memory(
        self,
        mut tokens: [u8; def::BOOTLOADING_SIZE],
        mut pointer: usize,
    ) -> ([u8; def::BOOTLOADING_SIZE], usize) {
        match self {
            Noop => {
                pointer += 1;
            }
            MovA(dest, src) => {
                tokens[pointer] = 1;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = src;
                pointer += 1;
            }
            MovB(dest, src) => {
                tokens[pointer] = 2;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = src;
                pointer += 1;
            }
            Add(dest, src) => {
                tokens[pointer] = 3;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = src;
                pointer += 1;
            }
            Sub(dest, src) => {
                tokens[pointer] = 4;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = src;
                pointer += 1;
            }
            Jmp(dest) => {
                tokens[pointer] = 5;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;
            }
            Jnz(dest, cond) => {
                tokens[pointer] = 6;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = cond;
                pointer += 1;
            }
        };
        (tokens, pointer)
    }
}

struct Parser {
    filename: String,
    instructions: Vec<Instruction>,
}

impl Parser {
    fn new(filename: String) -> Self {
        return Parser {
            filename,
            instructions: Vec::new(),
        };
    }
    fn into_bytes(self) -> [u8; def::BOOTLOADING_SIZE] {
        self.instructions
            .into_iter()
            .fold(
                ([0; def::BOOTLOADING_SIZE], 0usize),
                |(acc_mem, acc_point), instruction| instruction.add_to_memory(acc_mem, acc_point),
            )
            .0
    }
}
