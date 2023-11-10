use inter_parser::parse_interfaces;
use lexer::tokenize;
use std::{env, error::Error, fs};

mod inter_parser;
mod lexer;
mod macros;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).ok_or("file_path not found")?;
    let src = fs::read_to_string(file_path)?;
    let tokens = tokenize(src);
    println!("{:?}", tokens);

    let interfaces = parse_interfaces(tokens);
    println!("{:#?}", interfaces);

    Ok(())
}
