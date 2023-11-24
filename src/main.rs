use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::cmd::input;
use crate::js::interfaces_to_js_string;
use crate::lexer::{tokenize, find_interfaces};
use crate::parsers::{
    for_each_value, merge_interfaces, parse_and, parse_arrays, parse_custom_types, parse_generics, parse_interfaces,
    parse_or, parse_parens, parse_tuples,
};
use std::fs;
use std::time::Instant;

mod cmd;
mod js;
mod lexer;
mod macros;
mod parsers;
mod tests;

fn main() {
    let inst = Instant::now();
    let (read_path, write_path, write_path_extension) = input();
    let src = fs::read_to_string(read_path).unwrap();
    let src = find_interfaces(src);
    let tokens = tokenize(src);
    let mut interfaces = parse_interfaces(tokens);
    let interfaces_clone = interfaces.clone();

    interfaces
        .par_iter_mut()
        .for_each(|i| parse_custom_types(i, &interfaces_clone));
    interfaces.par_iter_mut().for_each(|i| for_each_value(i, parse_generics));
    interfaces.par_iter_mut().for_each(|i| parse_tuples(i));
    interfaces.par_iter_mut().for_each(|i| for_each_value(i, parse_arrays));
    interfaces.par_iter_mut().for_each(|i| parse_parens(i));
    interfaces.par_iter_mut().for_each(|i| parse_and(i));
    interfaces.par_iter_mut().for_each(|i| parse_or(i));
    merge_interfaces(&mut interfaces);

    let string: String = interfaces_to_js_string(interfaces, write_path_extension);

    fs::write(write_path, string).unwrap();

    println!("Finished Successfully in {}ms", inst.elapsed().as_millis());
}
