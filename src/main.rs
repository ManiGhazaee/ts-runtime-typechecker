use lexer::tokenize;
use std::{env, error::Error, fs};

mod lexer;
mod macros;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1);
    println!("{args:?}");
    let src = fs::read_to_string(file_path.ok_or("file_path not found")?)?;
    let tokens = tokenize(src);
    println!("{:?}", tokens);
    Ok(())
}
