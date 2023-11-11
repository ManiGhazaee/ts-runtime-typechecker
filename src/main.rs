use crate::inter_parser::{all_entries_value_walk, parse_arrays, parse_generics, parse_interfaces};
use crate::lexer::tokenize;
use std::{env, error::Error, fs};

mod inter_parser;
mod lexer;
mod macros;
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).ok_or("file_path not found")?;
    let src = fs::read_to_string(file_path)?;
    let tokens = tokenize(src);
    println!("{:?}", tokens);

    let mut interfaces = parse_interfaces(tokens);
    println!("{:#?}", interfaces);
    for i in interfaces.iter_mut() {
        all_entries_value_walk(i, parse_generics);
    }
    for i in interfaces.iter_mut() {
        all_entries_value_walk(i, parse_arrays);
    }

    println!("------------------------------------");
    println!("{:#?}", interfaces);

    Ok(())
}
