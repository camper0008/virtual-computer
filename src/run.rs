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
            /*
            0 - fn noop()
            1 mov &0x0 &0x1 # moves whatever's at memory address 0x1 to memory address 0x1
            2 mov &0x0 0x1 # moves an int with value 1 into memory address 0
            3 add &0x0 &0x1 # adds src address to dest address
            4 sub &0x0 &0x1 # subs src address from dest address
            5 jmp &0x1 # moves stack pointer to address 0x1
            6 jnz &0x1 &0x2 # moves stack pointer to address 0x1 if address 0x2 is not zero
            7-255 - crash and burn exception
            */
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
                mem[dest] += mem[src];
                pc += 1;
            }
            4 => {
                pc += 1;
                let dest = mem[pc] as usize;
                pc += 1;
                let src = mem[pc] as usize;
                mem[dest] -= mem[src];
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
            invalid_instruction => {
                panic!("invalid instruction {invalid_instruction} at memory {pc}")
            }
        }
        render_memory(&mem);
    }
}
