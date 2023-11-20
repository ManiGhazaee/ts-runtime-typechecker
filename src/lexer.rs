use crate::next;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Id(String),
    Number(usize),
    String(String),
    Undefined(String),
    Type(Type),
    Key(String),
    Interface,
    Export,
    Colon,
    Eq,
    Dash,
    Slash,
    Comment,
    QM, // QuestionMark
    EOE, // EndOfEntry
    EOF, // EndOfFile
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Custom(String),
    Oper(Oper),
    StringLit(String),
    NumberLit(String),
    Punct(Punct),
    True,
    False,
    String,
    Number,
    Object,
    Boolean,
    Null,
    Undefined,
    Function,
    Symbol,
    BigInt,
    Any,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Oper {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punct {
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    LPar,
    RPar,
    // Comma,
    LAngleB,
    RAngleB,
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
                let string = String::from_utf8_lossy(&src_vec[(i + 1)..j]).to_string();
                i = j;
                Token::String(string)
            }
            '\'' => {
                let mut j = i + 1;
                while j < src_vec_len && src_vec[j] as char != '\'' {
                    j += 1;
                }
                let string = String::from_utf8_lossy(&src_vec[(i + 1)..j]).to_string();
                i = j;
                Token::String(string)
            }
            '&' => Token::Type(Type::Oper(Oper::And)),
            '|' => Token::Type(Type::Oper(Oper::Or)),
            '-' => Token::Dash,
            '<' => Token::Type(Type::Punct(Punct::LAngleB)),
            '>' => Token::Type(Type::Punct(Punct::RAngleB)),
            // ',' => Token::Type(Type::Punct(Punct::Comma)),
            '(' => Token::Type(Type::Punct(Punct::LPar)),
            ')' => Token::Type(Type::Punct(Punct::RPar)),
            '[' => Token::Type(Type::Punct(Punct::LBrack)),
            ']' => Token::Type(Type::Punct(Punct::RBrack)),
            '{' => Token::Type(Type::Punct(Punct::LBrace)),
            '}' => Token::Type(Type::Punct(Punct::RBrace)),
            '=' => Token::Eq,
            ':' => Token::Colon,
            '?' => Token::QM,
            _ => {
                if c.is_alphabetic() || c == '_' || c == '$' {
                    let mut _c = c;
                    let mut temp: String = String::new();
                    let mut j: usize = i;
                    while (!is_skippable(&_c) && _c.is_alphanumeric()) || _c == '_' || _c == '$' {
                        temp += _c.to_string().as_str();
                        j += 1;
                        if j == src_vec_len {
                            break;
                        }
                        _c = src_vec[j] as char;
                    }
                    i = j - 1;
                    match temp.as_str() {
                        "interface" => Token::Interface,
                        "export" => Token::Export,
                        "true" => Token::Type(Type::True),
                        "false" => Token::Type(Type::False),
                        "string" => Token::Type(Type::String),
                        "number" => Token::Type(Type::Number),
                        "object" => Token::Type(Type::Object),
                        "boolean" => Token::Type(Type::Boolean),
                        "null" => Token::Type(Type::Null),
                        "undefined" => Token::Type(Type::Undefined),
                        "symbol" => Token::Type(Type::Symbol),
                        "bigint" => Token::Type(Type::BigInt),
                        "any" => Token::Type(Type::Any),
                        "Function" => Token::Type(Type::Function),
                        _ => Token::Id(temp),
                    }
                } else if c.is_numeric() {
                    let mut _c = c;
                    let mut temp: String = String::new();
                    let mut j: usize = i;
                    while (!is_skippable(&_c) && _c.is_numeric()) || _c == '_' {
                        temp += _c.to_string().as_str();
                        j += 1;
                        if j == src_vec_len {
                            break;
                        }
                        _c = src_vec[j] as char;
                    }
                    i = j - 1;
                    Token::Number(temp.replace("_", "").parse().unwrap())
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
