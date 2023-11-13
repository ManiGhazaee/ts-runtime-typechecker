use crate::lexer::{Oper, Punct, Token, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum EValue {
    Entry(Entry),
    Type(Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EKey {
    Name(String),
    GenericName(GenericName),
    Tuple,
    Or,
    And,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericName {
    Custom(String),
    Array,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    elems: Vec<EValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub key: EKey,
    pub value: Vec<EValue>,
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
            let end = i + 1;
            let start: usize;
            let mut j = i - 1;
            let mut par_count = 0;
            let mut args: Vec<EValue> = Vec::new();
            loop {
                match &value[j] {
                    EValue::Type(Type::Punct(Punct::RPar)) => par_count += 1,
                    EValue::Type(Type::Punct(Punct::LPar)) => {
                        par_count -= 1;
                        if par_count == 0 && args.len() > 0 {
                            start = j;
                            args.reverse();
                            value.splice(
                                start..=end,
                                [EValue::Entry(Entry {
                                    key: EKey::GenericName(GenericName::Array),
                                    value: args,
                                })],
                            );
                            break;
                        } else if par_count == 0 && args.len() == 0 {
                            panic!("unexpected EValue before []");
                        }
                    }
                    t => {
                        args.push(t.clone());
                        if par_count == 0 && args.len() > 0 {
                            start = j;
                            args.reverse();
                            value.splice(
                                start..=end,
                                [EValue::Entry(Entry {
                                    key: EKey::GenericName(GenericName::Array),
                                    value: args,
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
                                [EValue::Entry(Entry {
                                    key: EKey::GenericName(generic_name),
                                    value: args.clone(),
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

#[allow(dead_code)]
#[allow(unused)]
pub fn parse_tuples(value: &mut Vec<EValue>) -> () {
    let mut i = 0;
    while i < value.len() - 1 {
        match value[i] {
            EValue::Type(Type::Punct(Punct::LBrack)) => {
                if let EValue::Type(Type::Punct(Punct::RBrack)) = value[i + 1] {
                    continue;
                }
                let start = i - 1;
                let end: usize;
                let mut count = 1;
                let mut j = i + 1;
                let mut elems: Vec<EValue> = Vec::new();
                while j < value.len() {
                    match value[j] {
                        EValue::Type(Type::Punct(Punct::LBrack)) => count += 1,
                        EValue::Type(Type::Punct(Punct::RBrack)) => {
                            count -= 1;
                            if count == 0 && elems.len() > 0 {
                                end = j;

                                break;
                            } else if count == 0 && elems.len() == 0 {
                                panic!("why?");
                            }
                        }
                        _ => (),
                    }
                    j += 1;
                }
            }
            _ => (),
        }
        i += 1;
    }
}

type Addr = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum JSTypes {
    String,
    Number,
    Function,
    Boolean,
    Object,
    Undefined,
    Symbol,
    BigInt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JSValue {
    True,
    False,
    Undefined,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TCToken {
    Typeof(Addr, JSTypes),
    StringLit(Addr, String),
    IsArray(Addr),
    IsEqTo(Addr, JSValue),
    IsEqEqTo(Addr, JSValue),
    IsNotEqTo(Addr, JSValue),
    IsNotEqEqTo(Addr, JSValue),
    Array(Addr, JSTypes),
    And,
    Or,
}
pub fn to_tctokens(value: Vec<EValue>, addr: Vec<String>, result: &mut Vec<TCToken>) -> () {
    let mut i = 0;
    while i < value.len() {
        match &value[i] {
            EValue::Entry(e) => {
                result.push(TCToken::IsNotEqTo(addr.clone(), JSValue::Null));
                result.push(TCToken::Typeof(addr.clone(), JSTypes::Object));

                match &e.key {
                    EKey::GenericName(n) => match n.clone() {
                        // GenericName::Custom(_) => todo!(),
                        GenericName::Array => result.push(TCToken::IsArray(addr.clone())),
                        _ => (),
                    },
                    _ => (),
                }
            }
            EValue::Type(t) => match t {
                // Type::Custom(_) => todo!(),
                Type::Oper(o) => match o {
                    Oper::And => result.push(TCToken::And),
                    Oper::Or => result.push(TCToken::Or),
                },
                Type::StringLit(s) => result.push(TCToken::StringLit(addr.clone(), s.clone())),
                Type::True => result.push(TCToken::IsEqEqTo(addr.clone(), JSValue::True)),
                Type::False => result.push(TCToken::IsEqEqTo(addr.clone(), JSValue::False)),
                Type::String => result.push(TCToken::Typeof(addr.clone(), JSTypes::String)),
                Type::Number => result.push(TCToken::Typeof(addr.clone(), JSTypes::Number)),
                Type::Object => result.push(TCToken::Typeof(addr.clone(), JSTypes::Object)),
                Type::Boolean => result.push(TCToken::Typeof(addr.clone(), JSTypes::Boolean)),
                Type::Null => result.push(TCToken::IsEqEqTo(addr.clone(), JSValue::Null)),
                Type::Undefined => {
                    result.push(TCToken::Typeof(addr.clone(), JSTypes::Undefined));
                    result.push(TCToken::IsEqEqTo(addr.clone(), JSValue::Undefined))
                }
                Type::Any => (),
                _ => (),
            },
        }
        i += 1;
    }
}

pub fn value_walk_with_addr(
    entry: Entry,
    f: fn(value: Vec<EValue>, addr: Vec<String>, result: &mut Vec<TCToken>),
    mut addr: Vec<String>,
    result: &mut Vec<TCToken>,
) {
    (f)(entry.value.clone(), addr.clone(), result);
    for j in entry.value {
        if let EValue::Entry(e) = j {
            if let EKey::Name(name) = &e.key {
                addr.push(name.clone());
            }
            value_walk_with_addr(e.clone(), f, addr.clone(), result);
        }
    }
}

pub fn value_walk(entry: &mut Entry, f: fn(value: &mut Vec<EValue>)) {
    (f)(&mut entry.value);
    for j in &mut entry.value {
        if let EValue::Entry(e) = j {
            value_walk(e, f);
        }
    }
}

pub fn parse_and(entry: &mut Entry) {
    let mut i = 0;
    while i < entry.value.len() {
        match &mut entry.value[i] {
            EValue::Entry(e) => {
                parse_and(e);
                i += 1;
            }
            EValue::Type(Type::Oper(Oper::And)) => {
                if i == 0 || i == entry.value.len() - 1 {
                    entry.value.remove(i);
                } else {
                    entry.value.splice(
                        (i - 1)..=(i + 1),
                        [EValue::Entry(Entry {
                            key: EKey::And,
                            value: vec![entry.value[i - 1].clone(), entry.value[i + 1].clone()],
                        })],
                    );
                    i -= 1;
                }
            }
            _ => i += 1,
        }
    }
}



pub fn parse_or(entry: &mut Entry) {
    let mut i = 0;
    while i < entry.value.len() {
        match &mut entry.value[i] {
            EValue::Entry(e) => {
                parse_or(e);
                i += 1;
            }
            EValue::Type(Type::Oper(Oper::Or)) => {
                if i == 0 || i == entry.value.len() - 1 {
                    entry.value.remove(i);
                } else {
                    entry.value.splice(
                        (i - 1)..=(i + 1),
                        [EValue::Entry(Entry {
                            key: EKey::Or,
                            value: vec![entry.value[i - 1].clone(), entry.value[i + 1].clone()],
                        })],
                    );
                    i -= 1;
                }
            }
            _ => i += 1,
        }
    }
}
