use crate::def;
use crate::parse::Instruction::{Add, Jmp, Jnz, Load, MovA, MovB, Noop, Store, Sub};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Addr = u16;
type Int = u16;
type Value = u16;

pub enum Instruction {
    Noop,
    MovA(Addr, Addr),
    MovB(Addr, Int),
    Add(Addr, Addr),
    Sub(Addr, Addr),
    Jmp(Addr),
    Jnz(Addr, Addr),
    Load(Addr, Addr),
    Store(Addr, Addr),
}

impl Instruction {
    fn add_to_memory(
        self,
        mut tokens: [u16; def::BOOTLOADING_SIZE],
        mut pointer: usize,
    ) -> ([u16; def::BOOTLOADING_SIZE], usize) {
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
            Load(dest, src) => {
                tokens[pointer] = 7;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = src;
                pointer += 1;
            }
            Store(dest, src) => {
                tokens[pointer] = 8;
                pointer += 1;

                tokens[pointer] = dest;
                pointer += 1;

                tokens[pointer] = src;
                pointer += 1;
            }
        };
        (tokens, pointer)
    }
}

pub fn instructions_into_bytes(instructions: Vec<Instruction>) -> [u16; def::BOOTLOADING_SIZE] {
    instructions
        .into_iter()
        .fold(
            ([0; def::BOOTLOADING_SIZE], 0usize),
            |(acc_mem, acc_point), instruction| instruction.add_to_memory(acc_mem, acc_point),
        )
        .0
}

fn parse_address(addr: &str) -> Addr {
    match addr {
        "ra" => 0x10u16,
        "r1" => 0x11u16,
        "r2" => 0x12u16,
        "r3" => 0x13u16,
        "r4" => 0x14u16,
        "r5" => 0x15u16,
        "r6" => 0x16u16,
        "r7" => 0x17u16,
        "r8" => 0x18u16,
        _ => {
            let without_prefix = addr.trim_start_matches("&0x");
            u16::from_str_radix(without_prefix, 16).expect("invalid hex address")
        }
    }
}

fn parse_int(value: &str) -> Int {
    let without_prefix = value.trim_start_matches("0x");
    u16::from_str_radix(without_prefix, 16).expect("invalid hex value")
}

fn parse_maybe_address(maybe: &str) -> (Value, bool) {
    if maybe.starts_with('&') {
        (parse_address(maybe), true)
    } else {
        (parse_int(maybe), false)
    }
}

pub fn file(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter(|line| {
            let unwrapped_line = line.as_ref().unwrap();
            if unwrapped_line.is_empty() {
                false
            } else {
                unwrapped_line
                    .split_whitespace()
                    .take_while(|word| !word.contains(';'))
                    .count()
                    > 0
            }
        })
        .map(|line| {
            let unwrapped_line = line.unwrap();
            let mut words_iter = unwrapped_line
                .split_whitespace()
                .filter(|word| !word.is_empty())
                .map_while(|word| if word.contains(';') { None } else { Some(word) });
            let instruction = words_iter.next().unwrap();
            match instruction {
                "noop" => Instruction::Noop,
                "mov" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for mov"));
                    let (src, is_address) =
                        parse_maybe_address(words_iter.next().expect("missing argument 2 for mov"));
                    if is_address {
                        Instruction::MovA(dest, src)
                    } else {
                        Instruction::MovB(dest, src)
                    }
                }
                "add" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for add"));
                    let src = parse_address(words_iter.next().expect("missing argument 2 for add"));
                    Add(dest, src)
                }
                "sub" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for sub"));
                    let src = parse_address(words_iter.next().expect("missing argument 2 for sub"));
                    Sub(dest, src)
                }
                "jmp" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for jmp"));
                    Jmp(dest)
                }
                "jnz" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for jnz"));
                    let cond =
                        parse_address(words_iter.next().expect("missing argument 2 for jnz"));
                    Jnz(dest, cond)
                }
                "load" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for load"));
                    let src =
                        parse_address(words_iter.next().expect("missing argument 2 for load"));
                    Load(dest, src)
                }
                "store" => {
                    let dest =
                        parse_address(words_iter.next().expect("missing argument 1 for load"));
                    let src =
                        parse_address(words_iter.next().expect("missing argument 2 for load"));
                    Store(dest, src)
                }
                invalid_instruction => panic!("unrecognized instruction {invalid_instruction}"),
            }
        })
        .collect()
}
