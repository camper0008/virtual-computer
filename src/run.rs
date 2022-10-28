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
            }
            2 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let int = mem[pc];
                mem[dest] = int;
                pc += 1;
            }
            3 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[dest] = mem[dest].overflowing_add(mem[src]).0;
                pc += 1;
            }
            4 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[dest] = mem[dest].overflowing_sub(mem[src]).0;
                pc += 1;
            }
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
            }
            8 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[mem[dest] as usize] = mem[src];
                pc += 1;
            }
            invalid_instruction => {
                panic!("invalid instruction {invalid_instruction} at memory {pc}")
            }
        }
        render_memory(&mem);
    }
}
