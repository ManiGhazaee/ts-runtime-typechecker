use crate::lexer::{Punct, Token, Type};

#[derive(Debug, Clone)]
pub enum EValue {
    Entry(Entry),
    Type(Type),
    Generic(Generic),
    Tuple(Tuple),
}

#[derive(Debug, Clone)]
pub enum EKey {
    Name(String),
    None,
}

#[derive(Debug, Clone)]
pub enum GenericName {
    Custom(String),
    Array,
}

#[derive(Debug, Clone)]
pub struct Generic {
    name: GenericName,
    args: Vec<EValue>,
}

#[derive(Debug, Clone)]
pub struct Tuple {
    elems: Vec<EValue>,
}

#[derive(Debug, Clone)]
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

pub fn parse_interfaces(mut tokens: Vec<Token>) -> Vec<Entry> {
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
            Token::Type(Type::Punct(Punct::LBrace)) => {
                if let Token::EndOfEntry = tokens[i + 1] {
                    tokens.remove(i + 1);
                    i -= 1;
                }
            }
            Token::Type(Type::Punct(Punct::RBrace)) => {
                tokens.insert(i, Token::Type(Type::Punct(Punct::RBrace)));
                i += 1;
            }
            _ => (),
        }
        i += 1;
    }

    tokens = tokens
        .into_iter()
        .map(|i| {
            if let Token::Type(Type::Punct(Punct::RBrace)) = i {
                Token::EndOfEntry
            } else {
                i
            }
        })
        .collect();

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
                    panic!("stack length > 1");
                }
            }
            Token::Key(str) => {
                stack.push(Entry {
                    key: EKey::Name(str.to_string()),
                    value: Vec::new(),
                });
            }
            Token::Type(Type::Punct(Punct::LBrace)) => {
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
        panic!("stack length != 1");
    }

    interfaces
}

pub fn parse_arrays(value: &mut Vec<EValue>) -> () {
    let mut i = 0;
    while i < value.len() - 1 {
        if let (
            &EValue::Type(Type::Punct(Punct::LBrack)),
            &EValue::Type(Type::Punct(Punct::RBrack)),
        ) = (&value[i], &value[i + 1])
        {
            println!("matched: {}", i);
            let end = i + 1;
            let start: usize;
            let mut j = i - 1;
            let mut par_count = 0;
            let mut args: Vec<EValue> = Vec::new();
            loop {
                match &value[j] {
                    EValue::Type(Type::Punct(Punct::RPar)) => par_count += 1,
                    EValue::Type(Type::Punct(Punct::LPar)) => par_count -= 1,
                    t => {
                        args.push(t.clone());
                        if par_count == 0 && args.len() > 0 {
                            start = j;
                            value.splice(
                                start..=end,
                                [EValue::Generic(Generic {
                                    name: GenericName::Array,
                                    args: args.clone(),
                                })],
                            );
                            break;
                        } else if par_count == 0 && args.len() == 0 {
                            panic!("unexpected EValue before []");
                        }
                    }
                }
                j -= 1;
            }
        }
        i += 1;
    }
}

pub fn parse_generics(value: &mut Vec<EValue>) -> () {
    let mut i = 1;
    while i < value.len() {
        if let &EValue::Type(Type::Punct(Punct::Less)) = &value[i] {
            let generic_name: GenericName = match &value[i - 1] {
                EValue::Type(Type::Custom(str)) => match str.clone().as_str() {
                    "Array" => GenericName::Array,
                    _ => GenericName::Custom(str.clone()),
                },
                _ => panic!("unexpected generic name"),
            };
            let start = i - 1;
            let end: usize;
            let mut j = i + 1;
            let mut args: Vec<EValue> = Vec::new();
            let mut count = 1;
            while j < value.len() {
                match &value[j] {
                    EValue::Type(Type::Punct(Punct::Less)) => count += 1,
                    EValue::Type(Type::Punct(Punct::Greater)) => {
                        count -= 1;
                        if count == 0 && args.len() > 0 {
                            end = j;
                            value.splice(
                                start..=end,
                                [EValue::Generic(Generic {
                                    name: generic_name,
                                    args: args.clone(),
                                })],
                            );
                            break;
                        } else if count == 0 && args.len() == 0 {
                            panic!("why?");
                        }
                    }
                    t => {
                        args.push(t.clone());
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
}

pub fn all_entries_value_walk(entry: &mut Entry, f: fn(value: &mut Vec<EValue>)) {
    (f)(&mut entry.value);
    for j in &mut entry.value {
        if let EValue::Entry(e) = j {
            all_entries_value_walk(e, f);
        }
    }
}
