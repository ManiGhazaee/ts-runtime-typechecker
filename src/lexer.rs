use crate::next;

#[derive(Debug)]
pub enum Token {
    Id(String),
    Number(usize),
    String(String),
    Interface,
    Type,
    Export,
    TrueType,
    FalseType,
    StringType,
    NumberType,
    ObjectType,
    ArrayType,
    BooleanType,
    NullType,
    UndefinedType,
    AnyType,
    Pipe,
    Colon,
    Comma,
    Amper,
    Semi,
    Eq,
    Slash,
    Comment,
    LPar,
    RPar,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    EOF,
    Greater,
    Less,
    Undefined(String),
}

pub fn is_skippable(char: &char) -> bool {
    char == &'\0' || char == &' ' || char == &'\t' || char == &'\r' || char == &'\n'
}

pub fn tokenize(src: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let src_vec = Vec::from(src);
    let src_vec_len = src_vec.len();
    let mut i: usize = 0;

    while i < src_vec_len {
        let c: char = src_vec[i] as char;

        if is_skippable(&c) {
            i += 1;
            continue;
        }

        tokens.push(match c {
            '/' => {
                next! { src_vec, i,
                    Token::Slash,
                    '/' => {
                        let mut j = i + 1;
                        while j < src_vec_len && src_vec[j] as char != '\n' {
                            j += 1;
                        }
                        i = j;
                        Token::Comment
                    },
                }
            }
            '"' => {
                let mut j = i + 1;
                while j < src_vec_len && src_vec[j] as char != '"' {
                    j += 1;
                }
                let string = String::from_utf8_lossy(&src_vec[i..=j]).to_string();
                i = j;
                Token::String(string)
            }
            '\'' => {
                let mut j = i + 1;
                while j < src_vec_len && src_vec[j] as char != '\'' {
                    j += 1;
                }
                let string = String::from_utf8_lossy(&src_vec[i..=j]).to_string();
                i = j;
                Token::String(string)
            }
            ';' => Token::Semi,
            '<' => Token::Less,
            '>' => Token::Greater,
            '|' => Token::Pipe,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '&' => Token::Amper,
            '(' => Token::LPar,
            ')' => Token::RPar,
            '[' => Token::LBrack,
            ']' => Token::RBrack,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '=' => Token::Eq,
            _ => {
                if c.is_alphabetic() {
                    let mut _c = c;
                    let mut temp: String = String::new();
                    let mut j: usize = i;
                    while !is_skippable(&_c) && _c.is_alphanumeric() {
                        temp += _c.to_string().as_str();
                        j += 1;
                        if j == src_vec_len {
                            break;
                        }
                        _c = src_vec[j] as char;
                    }
                    i = j - 1;
                    match temp.as_str() {
                        "true" => Token::TrueType,
                        "false" => Token::FalseType,
                        "interface" => Token::Interface,
                        "type" => Token::Type,
                        "export" => Token::Export,
                        "string" => Token::StringType,
                        "number" => Token::NumberType,
                        "object" => Token::ObjectType,
                        "Array" => Token::ArrayType,
                        "boolean" => Token::BooleanType,
                        "null" => Token::NullType,
                        "undefined" => Token::UndefinedType,
                        "any" => Token::AnyType,
                        _ => Token::Id(temp),
                    }
                } else if c.is_numeric() {
                    let mut _c = c;
                    let mut temp: String = String::new();
                    let mut j: usize = i;
                    while !is_skippable(&_c) && _c.is_numeric() {
                        temp += _c.to_string().as_str();
                        j += 1;
                        if j == src_vec_len {
                            break;
                        }
                        _c = src_vec[j] as char;
                    }
                    i = j - 1;
                    Token::Number(temp.parse().unwrap())
                } else {
                    Token::Undefined(c.to_string())
                }
            }
        });
        i += 1;
    }
    tokens.push(Token::EOF);
    return tokens;
}
