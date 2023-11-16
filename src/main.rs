use crate::lexer::tokenize;
use crate::parsers::{
    js_tokens_to_string, parse_and, parse_arrays, parse_generics, parse_interfaces, parse_or, parse_parens, value_walk,
    x,
};
use std::string;
use std::{env, error::Error, fs};

mod lexer;
mod macros;
mod parsers;
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).ok_or("file_path not found")?;
    let src = fs::read_to_string(file_path)?;
    let tokens = tokenize(src);
    let mut interfaces = parse_interfaces(tokens);
    println!("{:#?}", interfaces);
    interfaces.iter_mut().for_each(|i| value_walk(i, parse_generics));
    interfaces.iter_mut().for_each(|i| value_walk(i, parse_arrays));
    interfaces.iter_mut().for_each(|i| parse_parens(i));
    interfaces.iter_mut().for_each(|i| parse_and(i));
    interfaces.iter_mut().for_each(|i| parse_or(i));
    println!("{:#?}", interfaces);

    let strings: Vec<String> = interfaces.into_iter().map(|i| {
        i.value.into_iter().map(|j| {
            let all = x(j, vec!["obj".to_string()]);
            let string = js_tokens_to_string(all);
            string
        }).collect::<Vec<String>>().join("")
    }).collect();
    println!("{:#?}", strings);

    Ok(())
}
