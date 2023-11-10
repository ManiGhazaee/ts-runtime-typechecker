use core::panic;
use lexer::{tokenize, Token, Type};
use std::{env, error::Error, fs};

mod lexer;
mod macros;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1);
    let src = fs::read_to_string(file_path.ok_or("file_path not found")?)?;
    let mut tokens = tokenize(src);

    let mut i = 0;
    while i < tokens.len() - 1 {
        if let Token::Id(str) | Token::String(str) = &tokens[i] {
            if let Token::Colon = tokens[i + 1] {
                tokens[i] = Token::Key(str.to_string());
                tokens.insert(i, Token::EndOfEntry);
                i += 1;
            }
        }
        i += 1;
    }

    tokens = tokens
        .into_iter()
        .filter(|i| {
            if let Token::Undefined(_) = i {
                false
            } else {
                true
            }
        })
        .collect();

    let mut i: usize = 0;
    while i < tokens.len() - 1 {
        match tokens[i] {
            Token::LBrace => {
                if let Token::EndOfEntry = tokens[i + 1] {
                    tokens.remove(i + 1);
                    i -= 1;
                }
            }
            Token::RBrace => {
                tokens.insert(i, Token::RBrace);
                i += 1;
            }
            _ => (),
        }
        i += 1;
    }

    tokens = tokens
        .into_iter()
        .map(|i| {
            if let Token::RBrace = i {
                Token::EndOfEntry
            } else {
                i
            }
        })
        .collect();

    println!("{:?}", tokens);

    let mut i: usize = 0;
    let mut stack: Vec<Entry> = Vec::new();
    let mut interfaces: Vec<Entry> = Vec::new();
    while i < tokens.len() {
        match &tokens[i] {
            Token::Interface => {
                if stack.len() == 0 {
                    let entry_name = match &tokens[i + 1] {
                        Token::Id(str) => str.to_string(),
                        _ => panic!("Interface name not found"),
                    };
                    let entry = Entry {
                        key: EKey::Name(entry_name),
                        value: Vec::new(),
                    };
                    stack.push(entry);
                    i += 2;
                } else if stack.len() == 1 {
                    interfaces.push(stack[0].clone());
                    stack.pop();
                    let entry_name = match &tokens[i + 1] {
                        Token::Id(str) => str.to_string(),
                        _ => panic!("Interface name not found"),
                    };
                    let entry = Entry {
                        key: EKey::Name(entry_name),
                        value: Vec::new(),
                    };
                    stack.push(entry);
                    i += 2;
                } else {
                    panic!("WHY??");
                }
            }
            Token::Key(str) => {
                stack.push(Entry {
                    key: EKey::Name(str.to_string()),
                    value: Vec::new(),
                });
            }
            Token::LBrace => {
                let entry = Entry {
                    key: EKey::None,
                    value: Vec::new(),
                };
                stack.push(entry);
            }
            Token::EndOfEntry => {
                stack_handle_remove(&mut stack);
            }
            Token::Type(_type) => add_type_value_to_last(&mut stack, _type),
            Token::String(str) => {
                add_type_value_to_last(&mut stack, &Type::StringLit(str.to_string()))
            }
            Token::Id(id) => add_type_value_to_last(&mut stack, &Type::Custom(id.to_string())),
            _ => (),
        }
        i += 1;
    }

    if stack.len() == 1 {
        interfaces.push(stack[0].clone());
        stack.pop();
    } else {
        panic!("why theres more than one elem in stack right now?");
    }

    println!("{:#?}", interfaces);

    Ok(())
}

#[derive(Debug, Clone)]
enum EValue {
    Entry(Entry),
    Type(Type),
}

#[derive(Debug, Clone)]
enum EKey {
    Name(String),
    None,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Entry {
    key: EKey,
    value: Vec<EValue>,
}

fn stack_handle_remove(stack: &mut Vec<Entry>) -> () {
    let stack_len = stack.len();
    if stack_len < 2 {
        return;
    }
    let last = stack[stack_len - 1].clone();
    stack[stack_len - 2].value.push(EValue::Entry(last));
    stack.pop();
}

fn add_type_value_to_last(stack: &mut Vec<Entry>, _type: &Type) -> () {
    let stack_len = stack.len();
    stack[stack_len - 1].value.push(EValue::Type(_type.clone()));
}
