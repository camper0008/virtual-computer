use crate::def;
use crate::parse::Instruction as Inst;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Addr = u16;
type Int = u16;
type Value = u16;

pub enum Instruction {
    Noop,
    MovA(Addr, Addr),
    MovI(Addr, Int),
    And(Addr, Addr),
    Or(Addr, Addr),
    Xor(Addr, Addr),
    Add(Addr, Addr),
    Sub(Addr, Addr),
    Mul(Addr, Addr),
    Div(Addr, Addr),
    Mod(Addr, Addr),
    Shl(Addr, Addr),
    Shr(Addr, Addr),
    Cmp(Addr, Addr),
    Lt(Addr, Addr),
    JmpA(Addr),
    JnzA(Addr, Addr),
    JmpI(Int),
    JnzI(Int, Addr),
    Load(Addr, Addr),
    Store(Addr, Addr),
}

impl Instruction {
    fn add_to_memory(
        self,
        mut tokens: [u16; def::BOOTLOADING_SIZE],
        mut pointer: usize,
    ) -> ([u16; def::BOOTLOADING_SIZE], usize) {
        let mut put_into = |words: Vec<u16>| {
            for word in words {
                tokens[pointer] = word;
                pointer += 1;
            }
        };
        match self {
           Inst::Noop => {
               pointer += 1;
           }
           Inst::MovA(dest, src) => put_into(vec![1, dest, src]),
           Inst::MovI(dest, src) => put_into(vec![2, dest, src]),
           Inst::Add(dest, src) => put_into(vec![3, dest, src]),
           Inst::Sub(dest, src) => put_into(vec![4, dest, src]),
           Inst::Mul(dest, src) => put_into(vec![9, dest, src]),
           Inst::Div(dest, src) => put_into(vec![10, dest, src]),
           Inst::Mod(dest, src) => put_into(vec![11, dest, src]),
           Inst::And(dest, src) => put_into(vec![12, dest, src]),
           Inst::Or(dest, src) => put_into(vec![13, dest, src]),
           Inst::Xor(dest, src) => put_into(vec![14, dest, src]),
           Inst::Shl(dest, src) => put_into(vec![15, dest, src]),
           Inst::Shr(dest, src) => put_into(vec![16, dest, src]),
           Inst::Cmp(dest, src) => put_into(vec![17, dest, src]),
           Inst::Lt(dest, src) => put_into(vec![18, dest, src]),
           Inst::JmpI(dest) => put_into(vec![5, dest]),
           Inst::JnzI(dest, cond) => put_into(vec![6, dest, cond]),
           Inst::JmpA(dest) => put_into(vec![19, dest]),
           Inst::JnzA(dest, cond) => put_into(vec![20, dest, cond]),
           Inst::Load(dest, src) => put_into(vec![7, dest, src]),
           Inst::Store(dest, src) => put_into(vec![8, dest, src]),
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
        invalid_addr if !invalid_addr.starts_with('&') => {
            panic!("address must be prefixed with &");
        }
        hex if hex.starts_with("&0x") => {
            let without_prefix = hex.trim_start_matches("&0x");
            u16::from_str_radix(without_prefix, 16).expect("invalid hex address")
        }
        dec => {
            let without_prefix = dec.trim_start_matches('&');
            without_prefix.parse::<u16>().expect("invalid dec address")
        }
    }
}

fn parse_int(value: &str) -> Int {
    match value {
        invalid_value if invalid_value.starts_with('&') => {
            panic!("int must not be prefixed with &")
        }
        hex if hex.starts_with("0x") => {
            let without_prefix = hex.trim_start_matches("0x");
            u16::from_str_radix(without_prefix, 16).expect("invalid hex value")
        }
        dec => dec.parse::<u16>().expect("invalid dec value"),
    }
}

fn parse_maybe_address(maybe: &str) -> (Value, bool) {
    if maybe.starts_with('&') || maybe.starts_with('r') {
        (parse_address(maybe), true)
    } else {
        (parse_int(maybe), false)
    }
}

fn unwrap_with_error<T>(res: Option<T>, error_msg: &'static str, line_number: usize) -> T {
    match res {
        Some(value) => value,
        None => {
            panic!("Error occurred on line {line_number}: '{error_msg}'");
        }
    }
}

fn parse_binary_instruction(
    dest_next: Option<&str>,
    src_next: Option<&str>,
    line_number: usize,
    maker: fn(dest: Addr, src: Addr) -> Instruction,
) -> Instruction {
    let dest = parse_address(unwrap_with_error(
        dest_next,
        "missing argument 1",
        line_number,
    ));
    let src = parse_address(unwrap_with_error(
        src_next,
        "missing argument 2",
        line_number,
    ));
    maker(dest, src)
}

pub fn file(filename: &str) -> Vec<Instruction> {
    let file = File::open(filename).expect("unable to open file");
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
        .enumerate()
        .map(|(line_number, line)| {
            let unwrapped_line = line.unwrap();
            let mut words_iter = unwrapped_line
                .split_whitespace()
                .filter(|word| !word.is_empty())
                .map_while(|word| if word.contains(';') { None } else { Some(word) });
            let instruction =
                unwrap_with_error(words_iter.next(), "invalid instruction", line_number);
            match instruction {
                "noop" => Instruction::Noop,
                "mov" => {
                    let dest = parse_address(unwrap_with_error(
                        words_iter.next(),
                        "missing argument 1 for mov",
                        line_number,
                    ));
                    let (src, is_address) = parse_maybe_address(unwrap_with_error(
                        words_iter.next(),
                        "missing argument 2 for mov",
                        line_number,
                    ));
                    if is_address {
                        Instruction::MovA(dest, src)
                    } else {
                        Instruction::MovI(dest, src)
                    }
                }
                "add" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Add)
                }
                "sub" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Sub)
                }
                "mul" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Mul)
                }
                "div" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Div)
                }
                "mod" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Mod)
                }
                "and" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::And)
                }
                "or" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Or)
                }
                "xor" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Xor)
                }
                "shl" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Shl)
                }
                "shr" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Shr)
                }
                "cmp" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Cmp)
                }
                "lt" => {
                    parse_binary_instruction(words_iter.next(), words_iter.next(), line_number, Inst::Lt)
                }
                "jmp" => {
                    let (dest, is_address) = parse_maybe_address(unwrap_with_error(
                        words_iter.next(),
                        "missing argument 1",
                        line_number,
                    ));

                    if is_address {
                        Inst::JmpA(dest)
                    } else {
                        Inst::JmpI(dest)
                    }
                }
                "jnz" => {
                    let (dest, is_address) = parse_maybe_address(unwrap_with_error(
                        words_iter.next(),
                        "missing argument 1",
                        line_number,
                    ));

                    let cond = parse_address(unwrap_with_error(
                        words_iter.next(),
                        "missing argument 2",
                        line_number,
                    ));

                    if is_address {
                        Inst::JnzA(dest, cond)
                    } else {
                        Inst::JnzI(dest, cond)
                    }
                }
                "load" => parse_binary_instruction(
                    words_iter.next(),
                    words_iter.next(),
                    line_number,
                    Inst::Load,
                ),
                "store" => parse_binary_instruction(
                    words_iter.next(),
                    words_iter.next(),
                    line_number,
                    Inst::Store,
                ),
                invalid_instruction => panic!("unrecognized instruction {invalid_instruction}"),
            }
        })
        .collect()
}
