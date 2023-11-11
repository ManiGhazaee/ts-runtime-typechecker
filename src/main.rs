use inter_parser::{all_entries_value_walk, parse_arrays, parse_interfaces};
use lexer::tokenize;
use std::{env, error::Error, fs};

use crate::inter_parser::parse_generics;

mod inter_parser;
mod lexer;
mod macros;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).ok_or("file_path not found")?;
    let src = fs::read_to_string(file_path)?;
    let tokens = tokenize(src);
    println!("{:?}", tokens);

    let mut interfaces = parse_interfaces(tokens);
    println!("{:#?}", interfaces);

    for i in interfaces.iter_mut() {
        all_entries_value_walk(i, parse_arrays);
    }

    for i in interfaces.iter_mut() {
        all_entries_value_walk(i, parse_generics);
    }

    println!("------------------------------------");
    println!("{:#?}", interfaces);

    Ok(())
}
