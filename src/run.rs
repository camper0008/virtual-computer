use crate::def;

fn render_memory(memory: &[u16; def::MEMORY_SIZE]) {
    print!("{esc}[1;1H", esc = 27 as char);
    (0..def::SCREEN_HEIGHT)
        .map(|y| {
            (0..def::SCREEN_WIDTH).map(move |x| {
                (memory[def::SCREEN_OFFSET + x + def::SCREEN_WIDTH * y] % 255) as u8 as char
            })
        })
        .for_each(|col| {
            col.for_each(|out| print!("{out}"));
            println!();
        });
}

fn check_and_rerender(addr: u16, mem: &[u16; def::MEMORY_SIZE]) {
    if (2048..2048 + (80 * 24)).contains(&addr) {
        render_memory(mem);
    }
}

fn run_binary_instruction(
    mem: &mut [u16; def::MEMORY_SIZE],
    pc: &mut usize,
    operation: fn(dest: u16, src: u16) -> u16,
) {
    *pc += 1;
    let dest = mem[*pc] as usize;
    *pc += 1;
    let src = mem[*pc] as usize;
    mem[dest] = operation(mem[dest], mem[src]);
    *pc += 1;
    check_and_rerender(dest as u16, &mem);
}

pub fn run(mut mem: [u16; def::MEMORY_SIZE]) {
    let mut pc = def::INITIAL_OFFSET;
    while pc < def::MEMORY_SIZE {
        match mem[pc] {
            0 => {
                pc += 1;
            }
            1 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[dest] = mem[src];
                pc += 1;
                check_and_rerender(dest as u16, &mem);
            }
            2 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let int = mem[pc];
                mem[dest] = int;
                pc += 1;
                check_and_rerender(dest as u16, &mem);
            }
            3 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest.overflowing_add(src).0),
            4 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest.overflowing_sub(src).0),
            9 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest.overflowing_mul(src).0),
            10 => {
                run_binary_instruction(&mut mem, &mut pc, |dest, src| dest.overflowing_div(src).0)
            }
            11 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest % src),
            12 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest & src),
            13 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest | src),
            14 => run_binary_instruction(&mut mem, &mut pc, |dest, src| dest ^ src),
            15 => run_binary_instruction(&mut mem, &mut pc, |dest, src| {
                dest.overflowing_shl(src.into()).0
            }),
            16 => run_binary_instruction(&mut mem, &mut pc, |dest, src| {
                dest.overflowing_shr(src.into()).0
            }),
            17 => run_binary_instruction(
                &mut mem,
                &mut pc,
                |dest, src| if dest == src { 1 } else { 0 },
            ),
            18 => run_binary_instruction(
                &mut mem,
                &mut pc,
                |dest, src| if dest < src { 1 } else { 0 },
            ),
            5 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc = dest;
            }
            6 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let cond = mem[mem[pc] as usize];
                if cond == 0 {
                    pc += 1;
                } else {
                    pc = dest;
                }
            }
            7 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[dest] = mem[mem[src] as usize];
                pc += 1;
                check_and_rerender(dest as u16, &mem);
            }
            8 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[mem[dest] as usize] = mem[src];
                pc += 1;
                check_and_rerender(mem[dest], &mem);
            }
            invalid_instruction => {
                panic!("invalid instruction {invalid_instruction} at memory {pc}")
            }
        }
    }
}
