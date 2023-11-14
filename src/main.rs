use crate::inter_parser::{
    parse_and, parse_arrays, parse_generics, parse_interfaces, parse_or, value_walk, x, js_tokens_to_string,
};
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
    let mut interfaces = parse_interfaces(tokens);
    for i in interfaces.iter_mut() {
        value_walk(i, parse_generics);
    }
    for i in interfaces.iter_mut() {
        value_walk(i, parse_arrays);
    }
    for i in interfaces.iter_mut() {
        parse_and(i);
    }
    for i in interfaces.iter_mut() {
        parse_or(i);
    }
    println!("{:#?}", interfaces);
    for i in interfaces.into_iter() {
        for j in i.value {
            let all = x(j, vec!["obj".to_string()]);
            let string = js_tokens_to_string(all);
            println!("{}", string);
        }
    }

    // println!("{:#?}", interfaces);

    Ok(())
}
