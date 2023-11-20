use crate::cmd::{get_extension, USAGE};
use crate::js::{function_dec, return_body};
use crate::lexer::tokenize;
use crate::parsers::{
    for_each_value, js_tokens_to_string, merge_interfaces, parse_and, parse_arrays, parse_custom_types, parse_generics,
    parse_interfaces, parse_or, parse_parens, parse_tuples, x, Key,
};
use std::time::Instant;
use std::{env, fs};

mod cmd;
mod js;
mod lexer;
mod macros;
mod parsers;
mod tests;

fn main() {
    let inst = Instant::now();
    let args: Vec<String> = env::args().collect();
    let (file_path, write_path) = if let (Some(f), Some(w)) = (args.get(1), args.get(2)) {
        (f, w)
    } else {
        eprintln!("{}", USAGE);
        return;
    };
    let write_path_extension = get_extension(write_path.clone());
    let src = fs::read_to_string(file_path).unwrap();
    let tokens = tokenize(src);
    println!("{:#?}", tokens);
    let mut interfaces = parse_interfaces(tokens);

    interfaces.iter_mut().for_each(|i| for_each_value(i, parse_generics));
    interfaces.iter_mut().for_each(|i| parse_tuples(i));
    interfaces.iter_mut().for_each(|i| for_each_value(i, parse_arrays));
    interfaces.iter_mut().for_each(|i| parse_parens(i));
    interfaces.iter_mut().for_each(|i| parse_and(i));
    interfaces.iter_mut().for_each(|i| parse_or(i));

    let interfaces_clone = interfaces.clone();
    interfaces
        .iter_mut()
        .for_each(|i| parse_custom_types(i, &interfaces_clone));

    merge_interfaces(&mut interfaces);
    println!("{:#?}", interfaces);

    let string: String = interfaces
        .into_iter()
        .map(|i| {
            let entries_len = i.value.len();
            let interface_name = if let Key::Name(name) = i.key {
                name
            } else {
                panic!("Name of interface not found");
            };
            let string = i
                .value
                .into_iter()
                .map(|j| {
                    let all = x(j, vec!["o".to_string()]);
                    js_tokens_to_string(all)
                })
                .collect::<Vec<String>>()
                .join("");
            let return_body = return_body(entries_len, string);
            format!("{}\n", function_dec(interface_name, return_body, write_path_extension))
        })
        .collect::<Vec<String>>()
        .join("\n");

    // println!("{}", string);

    fs::write(write_path, string).expect("Something went wrong with writing file");

    println!("Finished Successfully in {}ms", inst.elapsed().as_millis());
}
