mod cmd;
mod def;
mod parse;
mod run;

fn main() {
    let filename = cmd::filename();
    let mut mem: [u16; def::MEMORY_SIZE] = [0; def::MEMORY_SIZE];
    let instructions = parse::file(&filename);
    parse::instructions_into_bytes(instructions)
        .into_iter()
        .enumerate()
        .for_each(|(i, value)| mem[i + def::INITIAL_OFFSET] = value);

    run::run(mem);
}
