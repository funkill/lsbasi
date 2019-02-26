#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Whitespace,
    Integer,
    Plus,
    Eof,
}

#[derive(Debug, Eq, PartialEq)]
struct Token {
    _type: Type,
    value: Option<String>,
}

impl Token {
    fn whitespace() -> Token {
        Token {
            _type: Type::Whitespace,
            value: None,
        }
    }

    fn integer(value: String) -> Token {
        Token {
            _type: Type::Integer,
            value: Some(value),
        }
    }

    fn plus() -> Token {
        Token {
            _type: Type::Plus,
            value: Some("+".into()),
        }
    }

    fn eof() -> Token {
        Token {
            _type: Type::Eof,
            value: None,
        }
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(text: &str) -> i64 {
        let stream = Self::tokenize(text);
        assert_eq!(stream.len(), 4, "Wrong parsing input!");
        assert_eq!(stream[3]._type, Type::Eof, "Wrong end of line!");
        assert_eq!(stream[1]._type, Type::Plus, "Wrong operation type!");
        assert_eq!(stream[0]._type, Type::Integer, "Wrong first operand type!");
        assert_eq!(stream[2]._type, Type::Integer, "Wrong second operand type!");

        stream.get(0).unwrap().value.clone().unwrap().parse::<i64>().unwrap()
            + stream.get(2).unwrap().value.clone().unwrap().parse::<i64>().unwrap()
    }

    fn tokenize(text: &str) -> Vec<Token> {
        let mut tokens = vec![];
        let mut chars = text.chars().into_iter().peekable();

        while chars.peek() != None {
            let mut val = String::new();
            while let Some(curr) = chars.next() {
                let curr_type = detect_char_type(&curr);
                if curr_type == Type::Whitespace {
                    continue;
                }

                val.push(curr);
                if let Some(next) = chars.peek() {
                    let next_type = detect_char_type(&next);
                    if next_type != curr_type {
                        let token = Token {
                            _type: curr_type,
                            value: Some(val),
                        };
                        tokens.push(token);
                        break;
                    }
                } else {
                    let token = Token {
                        _type: curr_type,
                        value: Some(val),
                    };
                    tokens.push(token);
                    break;
                }
            }
        }

        tokens
    }
}

fn detect_char_type(item: &char) -> Type {
    match item {
        '0'...'9' => Type::Integer,
        ' ' => Type::Whitespace,
        '+' => Type::Plus,
        '\n' => Type::Eof,
        _ => panic!("Parse error!"),
    }
}

#[cfg(test)]
mod detect_char_type_tests {
    use super::{detect_char_type, Type};

    macro_rules! add_test {
        ($({name: $name:ident, parsed: $item:expr, expected_type: $expected:expr},)+) => {
            $(
                #[test]
                fn $name() {
                    let result = detect_char_type(&$item);

                    assert_eq!(result, $expected);
                }
            )+
        };
    }

    add_test!(
        {
            name: parse_zero,
            parsed: '0',
            expected_type: Type::Integer
        },
        {
            name: parse_one,
            parsed: '1',
            expected_type: Type::Integer
        },
        {
            name: parse_two,
            parsed: '2',
            expected_type: Type::Integer
        },
        {
            name: parse_three,
            parsed: '3',
            expected_type: Type::Integer
        },
        {
            name: parse_four,
            parsed: '4',
            expected_type: Type::Integer
        },
        {
            name: parse_five,
            parsed: '5',
            expected_type: Type::Integer
        },
        {
            name: parse_six,
            parsed: '6',
            expected_type: Type::Integer
        },
        {
            name: parse_seven,
            parsed: '7',
            expected_type: Type::Integer
        },
        {
            name: parse_eight,
            parsed: '8',
            expected_type: Type::Integer
        },
        {
            name: parse_nine,
            parsed: '9',
            expected_type: Type::Integer
        },
        {
            name: parse_plus,
            parsed: '+',
            expected_type: Type::Plus
        },
        {
            name: parse_eof,
            parsed: '\n',
            expected_type: Type::Eof
        },
    );

    #[test]
    #[should_panic]
    fn parse_wrong_symbol() {
        detect_char_type(&'a');
    }
}

#[cfg(test)]
mod evaluate_tests {
    use super::Interpreter;

    macro_rules! add_test {
        ($($name:ident: $eval:expr, result: $res:expr,)+) => {
            $(
                #[test]
                fn $name() {
                    let result = Interpreter::evaluate($eval);
                    assert_eq!(result, $res);
                }
            )+
        };
    }

    add_test!(
        eval: "1+2\n", result: 3,
        eval_0: " 1+2\n", result: 3,
        eval_1: "1 +2\n", result: 3,
        eval_2: "1+ 2\n", result: 3,
        eval_3: "1+2 \n", result: 3,
        eval_4: " 1 +2\n", result: 3,
        eval_5: " 1+ 2\n", result: 3,
        eval_6: " 1+2 \n", result: 3,
        eval_7: "12+2\n", result: 14,
    );

}
