use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    cmd::Extension,
    lexer::Type,
    parsers::{Entry, Generic, Key, Value},
};

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
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

type Addr = Vec<String>;

pub fn interfaces_to_js_string(interfaces: Vec<Entry>, write_path_extension: Extension) -> String {
    interfaces
        .into_par_iter()
        .map(|i| {
            let entries_len = i.value.len();
            let interface_name = if let Key::Name(name) = i.key {
                name
            } else {
                panic!("Name of interface not found");
            };
            let string = i
                .value
                .into_par_iter()
                .map(|j| {
                    let all = to_js_token(j, vec!["o".to_string()]);
                    js_tokens_to_string(all)
                })
                .collect::<Vec<String>>()
                .join("");
            let return_body = return_body(entries_len, string);
            format!("{}\n", function_dec(interface_name, return_body, write_path_extension))
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn to_js_token(value: Value, addr: Vec<String>) -> Vec<JSToken> {
    match value {
        Value::Entry(e) => match e.key {
            Key::Name(n) => {
                let new_addr = [addr.clone(), vec![n.clone()]].concat();
                let token_vec: Vec<JSToken> = e
                    .value
                    .par_iter()
                    .map(|val| to_js_token(val.clone(), new_addr.clone()))
                    .into_par_iter()
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
            Key::Optional(n) => {
                let new_addr = [addr.clone(), vec![n.clone()]].concat();
                let token_vec: Vec<JSToken> = e
                    .value
                    .par_iter()
                    .map(|val| to_js_token(val.clone(), new_addr.clone()))
                    .into_par_iter()
                    .flatten()
                    .collect();
                let res = [
                    vec![JSToken::And],
                    vec![JSToken::LPar],
                    vec![
                        JSToken::String(n.clone()),
                        JSToken::In,
                        JSToken::Addr(addr.clone()),
                        JSToken::EqEqEq,
                        JSToken::False,
                    ],
                    vec![JSToken::Or],
                    vec![JSToken::String(n), JSToken::In, JSToken::Addr(addr), JSToken::And],
                    token_vec.clone(),
                    vec![JSToken::RPar],
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
                        .par_iter()
                        .map(|val| to_js_token(val.clone(), new_addr.clone()))
                        .into_par_iter()
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
                let l = to_js_token(e.value[0].clone(), addr.clone());
                let r = to_js_token(e.value[1].clone(), addr.clone());
                let res = [vec![JSToken::LPar], l, vec![JSToken::Or], r, vec![JSToken::RPar]].concat();
                return res;
            }
            Key::And => {
                let l = to_js_token(e.value[0].clone(), addr.clone());
                let r = to_js_token(e.value[1].clone(), addr.clone());
                let res = [vec![JSToken::LPar], l, vec![JSToken::And], r, vec![JSToken::RPar]].concat();
                return res;
            }
            Key::None => {
                let entries_len = e.value.len();
                let token_vec: Vec<JSToken> = e
                    .value
                    .par_iter()
                    .map(|val| to_js_token(val.clone(), addr.clone()))
                    .into_par_iter()
                    .flatten()
                    .collect();
                let res = [
                    vec![JSToken::LPar],
                    typeof_token(addr.clone(), JSType::Object),
                    vec![JSToken::And],
                    loose_not_eq(JSToken::Addr(addr.clone()), JSToken::Null),
                    vec![JSToken::And],
                    strict_eq(
                        JSToken::ObjectKeysLength(addr.clone()),
                        JSToken::Number(entries_len.to_string()),
                    ),
                    token_vec,
                    vec![JSToken::RPar],
                ]
                .concat();
                return res;
            }
            Key::Paren => {
                let token_vec = e
                    .value
                    .par_iter()
                    .map(|val| to_js_token(val.clone(), addr.clone()))
                    .into_par_iter()
                    .flatten()
                    .collect();

                // let res = [vec![JSToken::LPar], token_vec, vec![JSToken::RPar]].concat();
                // return res;
                return token_vec;
            }
            Key::Tuple => {
                let mut token_vec: Vec<JSToken> = e
                    .value
                    .par_iter()
                    .enumerate()
                    .map(|i| {
                        vec![
                            to_js_token(i.1.clone(), [addr.clone(), vec![i.0.to_string()]].concat()),
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
        Value::Type(Type::Number) => return typeof_token(addr, JSType::Number),
        Value::Type(Type::String) => return typeof_token(addr, JSType::String),
        Value::Type(Type::Object) => return typeof_token(addr, JSType::Object),
        Value::Type(Type::Boolean) => return typeof_token(addr, JSType::Boolean),
        Value::Type(Type::Undefined) => return typeof_token(addr, JSType::Undefined),
        Value::Type(Type::Function) => return typeof_token(addr, JSType::Function),
        Value::Type(Type::Symbol) => return typeof_token(addr, JSType::Symbol),
        Value::Type(Type::BigInt) => return typeof_token(addr, JSType::BigInt),
        Value::Type(Type::False) => return strict_eq(JSToken::Addr(addr), JSToken::False),
        Value::Type(Type::True) => return strict_eq(JSToken::Addr(addr), JSToken::True),
        Value::Type(Type::Null) => return strict_eq(JSToken::Addr(addr), JSToken::Null),
        Value::Type(Type::Any) => return vec![JSToken::True],
        Value::Type(Type::Unknown) => return vec![JSToken::True],
        Value::Type(Type::StringLit(str)) => return strict_eq(JSToken::Addr(addr), JSToken::String(str)),
        Value::Type(Type::NumberLit(str)) => return strict_eq(JSToken::Addr(addr), JSToken::Number(str)),
        Value::Type(Type::Custom(_)) => typeof_token(addr, JSType::Object),
        _ => vec![JSToken::None],
    }
}

fn typeof_token(addr: Addr, js_type: JSType) -> Vec<JSToken> {
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

pub fn js_tokens_to_string(tokens: Vec<JSToken>) -> String {
    tokens
        .par_iter()
        .map(|i| {
            match i {
                JSToken::String(s) => {
                    format!("\"{}\"", escape_double_q(s.clone()))
                }
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
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn addr_to_string(addr: Addr) -> String {
    let mut temp = vec![format!("{}", addr[0].clone())];
    for i in 1..addr.len() {
        temp.push(format!("[{}]", format!("\"{}\"", addr[i])));
    }
    return temp.join("");
}

pub fn function_dec(name: String, return_body: String, extension: Extension) -> String {
    match extension {
        Extension::JS => format!("export function is{name}(o){{return({return_body})}}"),
        Extension::TS | Extension::DTS => {
            format!("export function is{name}(o: unknown): o is {name}{{return({return_body})}}")
        }
    }
}

pub fn return_body(entries_len: usize, return_body: String) -> String {
    format!("o!=null&&typeof o===\"object\"&&Object.keys(o).length==={entries_len}{return_body}")
}

fn escape_double_q(string: String) -> String {
    let mut string = Vec::from(string);
    let mut i = 0;
    while i < string.len() {
        let c = string[i] as char;
        if c == '"' {
            if i == 0 {
                string.insert(i, b'\\');
                i += 1;
            } else {
                if string[i - 1] != b'\\' {
                    string.insert(i, b'\\');
                    i += 1;
                }
            }
        }
        i += 1;
    }
    String::from_utf8(string).unwrap()
}
