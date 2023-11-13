use crate::{inter_parser::*, lexer::tokenize};
fn _test_(src: &str) -> Vec<Entry> {
    let tokens = tokenize(src.to_string());
    let mut interfaces = parse_interfaces(tokens);
    for i in interfaces.iter_mut() {
        value_walk(i, parse_generics);
    }
    for i in interfaces.iter_mut() {
        value_walk(i, parse_arrays);
    }
    for i in interfaces.iter_mut() {
        parse_and(i);
    }
    for i in interfaces.iter_mut() {
        parse_or(i);
    }
    return interfaces;
}

#[cfg(test)]
mod tests {
    use crate::lexer::Type;

    use super::*;

    #[test]
    fn test_parser() {
        let raw = "
        interface inter {
            _1: number;
        }
        ";
        let exp: Vec<Entry> = vec![Entry {
            key: EKey::Name("inter".to_string()),
            value: vec![EValue::Entry(Entry {
                key: EKey::Name("_1".to_string()),
                value: vec![EValue::Type(Type::Number)],
            })],
        }];
        assert_eq!(_test_(raw), exp);

        let raw = "
        interface inter {
            key_1: number;
            key_2: string | { key_3: object };
        }
        ";
        let exp: Vec<Entry> = vec![Entry {
            key: EKey::Name("inter".to_string()),
            value: vec![
                EValue::Entry(Entry {
                    key: EKey::Name("key_1".to_string()),
                    value: vec![EValue::Type(Type::Number)],
                }),
                EValue::Entry(Entry {
                    key: EKey::Name("key_2".to_string()),
                    value: vec![EValue::Entry(Entry {
                        key: EKey::Or,
                        value: vec![
                            EValue::Type(Type::String),
                            EValue::Entry(Entry {
                                key: EKey::None,
                                value: vec![EValue::Entry(Entry {
                                    key: EKey::Name("key_3".to_string()),
                                    value: vec![EValue::Type(Type::Object)],
                                })],
                            }),
                        ],
                    })],
                }),
            ],
        }];
        assert_eq!(_test_(raw), exp);

        let raw = "
        interface inter {
            _1: number[];
        }
        ";
        let exp: Vec<Entry> = vec![Entry {
            key: EKey::Name("inter".to_string()),
            value: vec![EValue::Entry(Entry {
                key: EKey::Name("_1".to_string()),
                value: vec![EValue::Entry(Entry {
                    key: EKey::GenericName(GenericName::Array),
                    value: vec![EValue::Type(Type::Number)],
                })],
            })],
        }];
        assert_eq!(_test_(raw), exp);

        let raw = "
        interface inter {
            _1: (number | string)[];
        }
        ";
        let exp: Vec<Entry> = vec![Entry {
            key: EKey::Name("inter".to_string()),
            value: vec![EValue::Entry(Entry {
                key: EKey::Name("_1".to_string()),
                value: vec![EValue::Entry(Entry {
                    key: EKey::GenericName(GenericName::Array),
                    value: vec![EValue::Entry(Entry {
                        key: EKey::Or,
                        value: vec![EValue::Type(Type::Number), EValue::Type(Type::String)],
                    })],
                })],
            })],
        }];
        assert_eq!(_test_(raw), exp);
    }
}
