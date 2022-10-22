use std::path::Path;
use std::{env, process};

pub fn filename() -> String {
    let filename_arg = env::args().find(|arg| arg.starts_with("-f"));
    if filename_arg.is_none() {
        println!("use: -f=<filename>");
        println!("example: -f=examples/hello");
        process::exit(0);
    }
    let filename = filename_arg
        .unwrap()
        .strip_prefix("-f=")
        .unwrap()
        .to_string();

    if !Path::new(&filename).exists() {
        println!("file '{filename}' not found");
        process::exit(2);
    }

    filename
}
