use crate::{lexer::tokenize, parsers::*};
fn _test_(src: &str) -> Vec<Entry> {
    let tokens = tokenize(src.to_string());
    let mut interfaces = parse_interfaces(tokens);
    for i in interfaces.iter_mut() {
        for_each_value(i, parse_generics);
    }
    for i in interfaces.iter_mut() {
        for_each_value(i, parse_arrays);
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
            key: Key::Name("inter".to_string()),
            value: vec![Value::Entry(Entry {
                key: Key::Name("_1".to_string()),
                value: vec![Value::Type(Type::Number)],
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
            key: Key::Name("inter".to_string()),
            value: vec![
                Value::Entry(Entry {
                    key: Key::Name("key_1".to_string()),
                    value: vec![Value::Type(Type::Number)],
                }),
                Value::Entry(Entry {
                    key: Key::Name("key_2".to_string()),
                    value: vec![Value::Entry(Entry {
                        key: Key::Or,
                        value: vec![
                            Value::Type(Type::String),
                            Value::Entry(Entry {
                                key: Key::None,
                                value: vec![Value::Entry(Entry {
                                    key: Key::Name("key_3".to_string()),
                                    value: vec![Value::Type(Type::Object)],
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
            key: Key::Name("inter".to_string()),
            value: vec![Value::Entry(Entry {
                key: Key::Name("_1".to_string()),
                value: vec![Value::Entry(Entry {
                    key: Key::Generic(Generic::Array),
                    value: vec![Value::Type(Type::Number)],
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
            key: Key::Name("inter".to_string()),
            value: vec![Value::Entry(Entry {
                key: Key::Name("_1".to_string()),
                value: vec![Value::Entry(Entry {
                    key: Key::Generic(Generic::Array),
                    value: vec![Value::Entry(Entry {
                        key: Key::Or,
                        value: vec![Value::Type(Type::Number), Value::Type(Type::String)],
                    })],
                })],
            })],
        }];
        assert_eq!(_test_(raw), exp);
    }
}
