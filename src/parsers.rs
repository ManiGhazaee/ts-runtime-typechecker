use core::panic;
use std::{clone, vec};

use crate::lexer::{Oper, Punct, Token, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Entry(Entry),
    Type(Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Name(String),
    Generic(Generic),
    Paren,
    Or,
    And,
    None,
    Tuple,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Generic {
    Custom(String),
    Array,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub key: Key,
    pub value: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JSType {
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
pub enum JSToken {
    String(String),
    Id(String),
    Addr(Addr),
    Number(String),
    ArrayIsArray(Addr),
    ObjectKeysLength(Addr),
    AddrLength(Addr),
    JSType(JSType),
    EqEq,
    EqEqEq,
    NotEq,
    NotEqEq,
    In,
    And,
    Or,
    LPar,
    RPar,
    Typeof,
    True,
    False,
    Undefined,
    Null,
    None,
}

type Addr = Vec<String>;

fn stack_handle_remove(stack: &mut Vec<Entry>) -> () {
    let stack_len = stack.len();
    if stack_len < 2 {
        return;
    }
    let last = stack[stack_len - 1].clone();
    stack[stack_len - 2].value.push(Value::Entry(last));
    stack.pop();
}

fn add_type_value_to_last(stack: &mut Vec<Entry>, _type: &Type) -> () {
    let stack_len = stack.len();
    stack[stack_len - 1].value.push(Value::Type(_type.clone()));
}

pub fn parse_interfaces(mut tokens: Vec<Token>) -> Vec<Entry> {
    let mut i = 0;
    while i < tokens.len() - 1 {
        if let Token::Colon = tokens[i + 1] {
            let mut has_matched = true;
            match &tokens[i] {
                Token::Id(str) | Token::String(str) => tokens[i] = Token::Key(str.to_string()),
                Token::Type(t) => match t {
                    Type::True => tokens[i] = Token::Key("true".to_string()),
                    Type::False => tokens[i] = Token::Key("false".to_string()),
                    Type::String => tokens[i] = Token::Key("string".to_string()),
                    Type::Number => tokens[i] = Token::Key("number".to_string()),
                    Type::Object => tokens[i] = Token::Key("object".to_string()),
                    Type::Boolean => tokens[i] = Token::Key("boolean".to_string()),
                    Type::Null => tokens[i] = Token::Key("null".to_string()),
                    Type::Undefined => tokens[i] = Token::Key("undefined".to_string()),
                    Type::Symbol => tokens[i] = Token::Key("symbol".to_string()),
                    Type::BigInt => tokens[i] = Token::Key("bigint".to_string()),
                    Type::Any => tokens[i] = Token::Key("any".to_string()),
                    Type::Function => tokens[i] = Token::Key("Function".to_string()),
                    _ => has_matched = false,
                },
                Token::Interface => tokens[i] = Token::Key("interface".to_string()),
                Token::Export => tokens[i] = Token::Key("export".to_string()),
                _ => has_matched = false,
            }
            if has_matched {
                tokens.insert(i, Token::EOE);
                i += 1;
            }
        }
        i += 1;
    }

    let mut i = 1;
    while i < tokens.len() {
        match tokens[i] {
            Token::Number(n) => {
                if let Token::Dash = tokens[i - 1] {
                    tokens.splice(
                        (i - 1)..=i,
                        [Token::Type(Type::NumberLit("-".to_string() + n.to_string().as_str()))],
                    );
                    i -= 1;
                } else {
                    tokens[i] = Token::Type(Type::NumberLit(n.to_string()));
                }
            }
            _ => (),
        }
        i += 1;
    }

    tokens = tokens
        .into_iter()
        .filter(|i| if let Token::Undefined(_) = i { false } else { true })
        .collect();

    let mut i: usize = 0;
    while i < tokens.len() - 1 {
        match tokens[i] {
            Token::Type(Type::Punct(Punct::LBrace)) => {
                if let Token::EOE = tokens[i + 1] {
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
                Token::EOE
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
                        key: Key::Name(entry_name),
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
                        key: Key::Name(entry_name),
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
                    key: Key::Name(str.to_string()),
                    value: Vec::new(),
                });
            }
            Token::Type(Type::Punct(Punct::LBrace)) => {
                let entry = Entry {
                    key: Key::None,
                    value: Vec::new(),
                };
                stack.push(entry);
            }
            Token::EOE => {
                stack_handle_remove(&mut stack);
            }
            Token::Type(_type) => add_type_value_to_last(&mut stack, _type),
            Token::String(str) => add_type_value_to_last(&mut stack, &Type::StringLit(str.to_string())),
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

pub fn parse_arrays(value: &mut Vec<Value>) -> () {
    if value.len() == 0 {
        return;
    }
    let mut i = value.len() - 1;
    while i > 0 {
        if let (&Value::Type(Type::Punct(Punct::LBrack)), &Value::Type(Type::Punct(Punct::RBrack))) =
            (&value[i - 1], &value[i])
        {
            let end = i;
            let start: usize;
            let mut j = i - 2;
            let mut par_count = 0;
            let mut args: Vec<Value> = Vec::new();
            loop {
                match &value[j] {
                    Value::Type(Type::Punct(Punct::RBrack)) => {
                        args.push(Value::Type(Type::Punct(Punct::RBrack)));
                        j -= 1;
                        continue;
                    }
                    Value::Type(Type::Punct(Punct::LBrack)) => {
                        args.push(Value::Type(Type::Punct(Punct::LBrack)));
                        j -= 1;
                        continue;
                    }
                    Value::Type(Type::Punct(Punct::RPar)) => par_count += 1,
                    Value::Type(Type::Punct(Punct::LPar)) => {
                        par_count -= 1;
                        if par_count == 0 && args.len() > 0 {
                            start = j;
                            args.reverse();
                            value.splice(
                                start..=end,
                                [Value::Entry(Entry {
                                    key: Key::Generic(Generic::Array),
                                    value: args,
                                })],
                            );
                            if let Value::Entry(e) = &mut value[start] {
                                parse_arrays(&mut e.value);
                            }
                            if start == 0 {
                                i = start;
                            } else {
                                i = start - 1;
                            }
                            break;
                        } else if par_count == 0 && args.len() == 0 {
                            panic!("unexpected Value before []");
                        }
                    }
                    t => {
                        args.push(t.clone());
                        if par_count == 0 && args.len() > 0 {
                            start = j;
                            args.reverse();
                            value.splice(
                                start..=end,
                                [Value::Entry(Entry {
                                    key: Key::Generic(Generic::Array),
                                    value: args,
                                })],
                            );
                            if let Value::Entry(e) = &mut value[start] {
                                parse_arrays(&mut e.value);
                            }
                            if start == 0 {
                                i = start;
                            } else {
                                i = start - 1;
                            }
                            break;
                        } else if par_count == 0 && args.len() == 0 {
                            panic!("unexpected Value before []");
                        }
                    }
                }
                j -= 1;
            }
        } else {
            i -= 1;
        }
    }
}

pub fn parse_generics(value: &mut Vec<Value>) -> () {
    let mut i = 1;
    while i < value.len() {
        if let &Value::Type(Type::Punct(Punct::LAngleB)) = &value[i] {
            let generic_name: Generic = match &value[i - 1] {
                Value::Type(Type::Custom(str)) => match str.clone().as_str() {
                    "Array" => Generic::Array,
                    _ => Generic::Custom(str.clone()),
                },
                _ => panic!("Unexpected generic name"),
            };
            let start = i - 1;
            let end: usize;
            let mut j = i + 1;
            let mut args: Vec<Value> = Vec::new();
            let mut count = 1;
            while j < value.len() {
                match &value[j] {
                    Value::Type(Type::Punct(Punct::LAngleB)) => {
                        args.push(Value::Type(Type::Punct(Punct::LAngleB)));
                        count += 1;
                    }
                    Value::Type(Type::Punct(Punct::RAngleB)) => {
                        count -= 1;
                        if count == 0 && args.len() > 0 {
                            end = j;
                            value.splice(
                                start..=end,
                                [Value::Entry(Entry {
                                    key: Key::Generic(generic_name),
                                    value: args.clone(),
                                })],
                            );
                            if let Value::Entry(e) = &mut value[start] {
                                parse_generics(&mut e.value);
                            }
                            break;
                        } else if count == 0 && args.len() == 0 {
                            panic!("why?");
                        }
                        args.push(Value::Type(Type::Punct(Punct::RAngleB)));
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

pub fn for_each_value(entry: &mut Entry, f: fn(value: &mut Vec<Value>)) {
    (f)(&mut entry.value);
    for j in &mut entry.value {
        if let Value::Entry(e) = j {
            for_each_value(e, f);
        }
    }
}

pub fn parse_parens(entry: &mut Entry) {
    let mut i = 0;
    while i < entry.value.len() {
        match &mut entry.value[i] {
            Value::Entry(e) => {
                parse_parens(e);
            }
            Value::Type(t) => match t {
                Type::Punct(Punct::LPar) => {
                    let start = i;
                    let end: usize;
                    let mut j = i + 1;
                    let mut par_count = 1;
                    let mut value: Vec<Value> = Vec::new();
                    while j < entry.value.len() {
                        match &mut entry.value[j] {
                            Value::Type(Type::Punct(Punct::LPar)) => {
                                par_count += 1;
                                value.push(Value::Type(Type::Punct(Punct::LPar)));
                            }
                            Value::Type(Type::Punct(Punct::RPar)) => {
                                par_count -= 1;
                                if par_count == 0 {
                                    end = j;
                                    entry
                                        .value
                                        .splice(start..=end, [Value::Entry(Entry { key: Key::Paren, value })]);
                                    if let Value::Entry(e) = &mut entry.value[i] {
                                        parse_parens(e);
                                    }
                                    break;
                                } else {
                                    value.push(Value::Type(Type::Punct(Punct::RPar)));
                                }
                            }
                            t => {
                                value.push(t.clone());
                            }
                        }
                        j += 1;
                    }
                }
                _ => (),
            },
        }
        i += 1;
    }
}

pub fn parse_tuples(entry: &mut Entry) {
    let mut i = 0;
    while i < entry.value.len() {
        match &mut entry.value[i] {
            Value::Entry(e) => {
                parse_tuples(e);
            }
            Value::Type(t) => match t {
                Type::Punct(Punct::LBrack) => {
                    if let Value::Type(Type::Punct(Punct::RBrack)) = entry.value[i + 1] {
                        i += 1;
                        continue;
                    }
                    let start = i;
                    let end: usize;
                    let mut j = i + 1;
                    let mut brack_count = 1;
                    let mut value: Vec<Value> = Vec::new();
                    while j < entry.value.len() {
                        match &mut entry.value[j] {
                            Value::Type(Type::Punct(Punct::LBrack)) => {
                                brack_count += 1;
                                value.push(Value::Type(Type::Punct(Punct::LBrack)));
                            }
                            Value::Type(Type::Punct(Punct::RBrack)) => {
                                brack_count -= 1;
                                if brack_count == 0 {
                                    end = j;
                                    entry
                                        .value
                                        .splice(start..=end, [Value::Entry(Entry { key: Key::Tuple, value })]);
                                    if let Value::Entry(e) = &mut entry.value[i] {
                                        parse_tuples(e);
                                    }
                                    break;
                                } else {
                                    value.push(Value::Type(Type::Punct(Punct::RBrack)));
                                }
                            }
                            t => {
                                value.push(t.clone());
                            }
                        }
                        j += 1;
                    }
                }
                _ => (),
            },
        }
        i += 1;
    }
}

pub fn parse_and(entry: &mut Entry) {
    let mut i = 0;
    while i < entry.value.len() {
        match &mut entry.value[i] {
            Value::Entry(e) => {
                parse_and(e);
                i += 1;
            }
            Value::Type(Type::Oper(Oper::And)) => {
                if i == 0 || i == entry.value.len() - 1 {
                    entry.value.remove(i);
                } else {
                    entry.value.splice(
                        (i - 1)..=(i + 1),
                        [Value::Entry(Entry {
                            key: Key::And,
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
            Value::Entry(e) => {
                parse_or(e);
                i += 1;
            }
            Value::Type(Type::Oper(Oper::Or)) => {
                if i == 0 || i == entry.value.len() - 1 {
                    entry.value.remove(i);
                } else {
                    entry.value.splice(
                        (i - 1)..=(i + 1),
                        [Value::Entry(Entry {
                            key: Key::Or,
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

pub fn x(value: Value, addr: Vec<String>) -> Vec<JSToken> {
    match value {
        Value::Entry(e) => match e.key {
            Key::Name(n) => {
                let new_addr = [addr.clone(), vec![n.clone()]].concat();
                let token_vec: Vec<JSToken> = e
                    .value
                    .iter()
                    .map(|val| x(val.clone(), new_addr.clone()))
                    .into_iter()
                    .flatten()
                    .collect();
                let res = [
                    vec![JSToken::And],
                    vec![JSToken::String(n), JSToken::In, JSToken::Addr(addr), JSToken::And],
                    token_vec.clone(),
                ]
                .concat();
                return res;
            }
            Key::Generic(g) => match g {
                Generic::Custom(_) => vec![JSToken::None],
                Generic::Array => {
                    let new_addr = [addr.clone(), vec!["0".to_string()]].concat();
                    let token_vec: Vec<JSToken> = e
                        .value
                        .iter()
                        .map(|val| x(val.clone(), new_addr.clone()))
                        .into_iter()
                        .flatten()
                        .collect();
                    let res = [
                        vec![JSToken::LPar, JSToken::ArrayIsArray(addr.clone()), JSToken::And],
                        token_vec,
                        vec![JSToken::RPar],
                    ]
                    .concat();
                    return res;
                }
            },
            Key::Or => {
                let l = x(e.value[0].clone(), addr.clone());
                let r = x(e.value[1].clone(), addr.clone());
                let res = [vec![JSToken::LPar], l, vec![JSToken::Or], r, vec![JSToken::RPar]].concat();
                return res;
            }
            Key::And => {
                let l = x(e.value[0].clone(), addr.clone());
                let r = x(e.value[1].clone(), addr.clone());
                let res = [vec![JSToken::LPar], l, vec![JSToken::And], r, vec![JSToken::RPar]].concat();
                return res;
            }
            Key::None => {
                let entries_len = e.value.len();
                let token_vec: Vec<JSToken> = e
                    .value
                    .iter()
                    .map(|val| x(val.clone(), addr.clone()))
                    .into_iter()
                    .flatten()
                    .collect();
                let res = [
                    typeof_token_vec(addr.clone(), JSType::Object),
                    vec![JSToken::And],
                    loose_not_eq(JSToken::Addr(addr.clone()), JSToken::Null),
                    vec![JSToken::And],
                    strict_eq(
                        JSToken::ObjectKeysLength(addr.clone()),
                        JSToken::Number(entries_len.to_string()),
                    ),
                    token_vec,
                ]
                .concat();
                return res;
            }
            Key::Paren => {
                let token_vec = e
                    .value
                    .iter()
                    .map(|val| x(val.clone(), addr.clone()))
                    .into_iter()
                    .flatten()
                    .collect();

                // let res = [vec![JSToken::LPar], token_vec, vec![JSToken::RPar]].concat();
                // return res;
                return token_vec;
            }
            Key::Tuple => {
                let mut token_vec: Vec<JSToken> = e
                    .value
                    .iter()
                    .enumerate()
                    .map(|i| {
                        vec![
                            x(i.1.clone(), [addr.clone(), vec![i.0.to_string()]].concat()),
                            vec![JSToken::And],
                        ]
                    })
                    .flatten()
                    .flatten()
                    .collect();

                token_vec.pop(); // remove last And

                let res = [
                    vec![JSToken::LPar, JSToken::ArrayIsArray(addr.clone()), JSToken::And],
                    strict_eq(
                        JSToken::AddrLength(addr.clone()),
                        JSToken::Number(e.value.len().to_string()),
                    ),
                    vec![JSToken::And],
                    token_vec,
                    vec![JSToken::RPar],
                ]
                .concat();
                return res;
            }
        },
        Value::Type(Type::Number) => return typeof_token_vec(addr, JSType::Number),
        Value::Type(Type::String) => return typeof_token_vec(addr, JSType::String),
        Value::Type(Type::Object) => return typeof_token_vec(addr, JSType::Object),
        Value::Type(Type::Boolean) => return typeof_token_vec(addr, JSType::Boolean),
        Value::Type(Type::Undefined) => return typeof_token_vec(addr, JSType::Undefined),
        Value::Type(Type::Function) => return typeof_token_vec(addr, JSType::Function),
        Value::Type(Type::Symbol) => return typeof_token_vec(addr, JSType::Symbol),
        Value::Type(Type::BigInt) => return typeof_token_vec(addr, JSType::BigInt),
        Value::Type(Type::False) => return strict_eq(JSToken::Addr(addr), JSToken::False),
        Value::Type(Type::True) => return strict_eq(JSToken::Addr(addr), JSToken::True),
        Value::Type(Type::Null) => return strict_eq(JSToken::Addr(addr), JSToken::Null),
        Value::Type(Type::StringLit(str)) => return strict_eq(JSToken::Addr(addr), JSToken::String(str)),
        Value::Type(Type::NumberLit(str)) => return strict_eq(JSToken::Addr(addr), JSToken::Number(str)),
        Value::Type(Type::Custom(_)) => typeof_token_vec(addr, JSType::Object),
        _ => vec![JSToken::None],
    }
}

fn typeof_token_vec(addr: Addr, js_type: JSType) -> Vec<JSToken> {
    vec![
        JSToken::Typeof,
        JSToken::Addr(addr),
        JSToken::EqEqEq,
        JSToken::JSType(js_type),
    ]
}

fn strict_eq(left: JSToken, right: JSToken) -> Vec<JSToken> {
    vec![left, JSToken::EqEqEq, right]
}

fn loose_not_eq(left: JSToken, right: JSToken) -> Vec<JSToken> {
    vec![left, JSToken::NotEq, right]
}

pub fn js_tokens_to_string(mut tokens: Vec<JSToken>) -> String {
    let mut string_vec: Vec<String> = Vec::new();
    for i in tokens.iter_mut() {
        string_vec.push(match i {
            JSToken::String(s) => format!("\"{}\"", s.clone()),
            JSToken::Addr(addr) => addr_to_string(addr.clone()),
            JSToken::Id(id) => id.clone(),
            JSToken::Number(n) => n.clone(),
            JSToken::EqEq => "==".to_string(),
            JSToken::EqEqEq => "===".to_string(),
            JSToken::NotEq => "!=".to_string(),
            JSToken::NotEqEq => "!==".to_string(),
            JSToken::In => "in ".to_string(),
            JSToken::And => "&&".to_string(),
            JSToken::Or => "||".to_string(),
            JSToken::LPar => "(".to_string(),
            JSToken::RPar => ")".to_string(),
            JSToken::Typeof => "typeof ".to_string(),
            JSToken::True => "true".to_string(),
            JSToken::False => "false".to_string(),
            JSToken::Undefined => "undefined".to_string(),
            JSToken::Null => "null".to_string(),
            JSToken::None => "".to_string(),
            JSToken::ArrayIsArray(addr) => format!("Array.isArray({})", addr_to_string(addr.clone())),
            JSToken::AddrLength(addr) => format!("{}.length", addr_to_string(addr.clone())),
            JSToken::ObjectKeysLength(addr) => format!("Object.keys({}).length", addr_to_string(addr.clone())),
            JSToken::JSType(t) => String::from(match t {
                JSType::String => "\"string\"",
                JSType::Number => "\"number\"",
                JSType::Function => "\"function\"",
                JSType::Boolean => "\"boolean\"",
                JSType::Object => "\"object\"",
                JSType::Undefined => "\"undefined\"",
                JSType::Symbol => "\"symbol\"",
                JSType::BigInt => "\"bigint\"",
            }),
            // _ => "".to_string(),
        })
    }
    return string_vec.join("");
}

fn addr_to_string(addr: Addr) -> String {
    let mut temp = vec![format!("{}", addr[0].clone())];
    for i in 1..addr.len() {
        temp.push(format!("[{}]", format!("\"{}\"", addr[i])));
    }
    return temp.join("");
}

pub fn parse_custom_types(entry: &mut Entry, interfaces: &Vec<Entry>) {
    let mut i = 0;
    while i < entry.value.len() {
        let current = &mut entry.value[i];
        match current {
            Value::Entry(e) => {
                parse_custom_types(e, interfaces);
            }
            Value::Type(Type::Custom(custom_type)) => {
                let custom_type_clone = custom_type.clone();
                for j in interfaces {
                    if let Key::Name(interface_name) = &j.key {
                        if interface_name.clone() == custom_type_clone {
                            let new = Value::Entry(Entry {
                                key: Key::None,
                                value: j.value.clone(),
                            })
                            .clone();
                            *current = new;
                        }
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }
}

pub fn merge_interfaces(interfaces: &mut Vec<Entry>) {
    let mut i = 0;
    while i < interfaces.len() {
        let current_name = if let Key::Name(str) = &interfaces[i].key {
            str.clone()
        } else {
            panic!("Not possible");
        };
        let mut j = i + 1;
        while j < interfaces.len() {
            if let Key::Name(str) = &interfaces[j].key {
                if str.clone() == current_name {
                    for k in 0..interfaces[j].value.len() {
                        let clone = interfaces[j].clone().value[k].clone();
                        interfaces[i].value.push(clone);
                    }
                    interfaces.remove(j);
                    j -= 1;
                } 
            } else {
                panic!("Not possible too");
            }
            j += 1;
        }
        i += 1;
    }
}
