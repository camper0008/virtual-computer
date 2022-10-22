mod def;
mod parse;

fn main() {
    let mut mem: [u8; def::MEMORY_SIZE] = [0; def::MEMORY_SIZE];
    let instructions = parse::file("examples/test");
    parse::instructions_into_bytes(instructions)
        .into_iter()
        .enumerate()
        .for_each(|(i, value)| mem[i + def::STARTING_MEMORY] = value);

    let formatted_mem: Vec<u8> = mem.into_iter().skip(64).collect();
    println!("{:?}", formatted_mem);
}
