#[derive(Debug, Eq, PartialEq)]
enum Type {
    Whitespace,
    Integer,
    Plus,
    Eof,
}

#[derive(Debug, Eq, PartialEq)]
struct Token {
    _type: Type,
    value: Option<char>,
}

impl Token {
    fn whitespace() -> Token {
        Token {
            _type: Type::Whitespace,
            value: None,
        }
    }

    fn integer(value: char) -> Token {
        Token {
            _type: Type::Integer,
            value: Some(value),
        }
    }

    fn plus() -> Token {
        Token {
            _type: Type::Plus,
            value: Some('+'),
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

        stream.get(0).unwrap().value.unwrap().to_digit(10).unwrap() as i64
            + stream.get(2).unwrap().value.unwrap().to_digit(10).unwrap() as i64
    }

    fn tokenize(text: &str) -> Vec<Token> {
        text.chars()
            .into_iter()
            .filter_map(|item| {
                let token = tokenize(item);

                match token._type {
                    Type::Whitespace => None,
                    _ => Some(token),
                }
            })
            .collect::<Vec<Token>>()
    }
}

fn tokenize(item: char) -> Token {
    match item {
        item @ '0'...'9' => Token::integer(item),
        ' ' => Token::whitespace(),
        '+' => Token::plus(),
        '\n' => Token::eof(),
        _ => panic!("Parse error!"),
    }
}

#[cfg(test)]
mod tokenize_tests {
    use super::{tokenize, Token, Type};

    macro_rules! add_test {
        ($({name: $name:ident, parsed: $item:expr, expected_type: $type:expr, expected_value: $value:expr},)+) => {
            $(
                #[test]
                fn $name() {
                    let expected = Token { _type: $type, value: $value };
                    let result = tokenize($item);

                    assert_eq!(result, expected);
                }
            )+
        };
    }

    add_test!(
        {
            name: parse_zero,
            parsed: '0',
            expected_type: Type::Integer,
            expected_value: Some('0')
        },
        {
            name: parse_one,
            parsed: '1',
            expected_type: Type::Integer,
            expected_value: Some('1')
        },
        {
            name: parse_two,
            parsed: '2',
            expected_type: Type::Integer,
            expected_value: Some('2')
        },
        {
            name: parse_three,
            parsed: '3',
            expected_type: Type::Integer,
            expected_value: Some('3')
        },
        {
            name: parse_four,
            parsed: '4',
            expected_type: Type::Integer,
            expected_value: Some('4')
        },
        {
            name: parse_five,
            parsed: '5',
            expected_type: Type::Integer,
            expected_value: Some('5')
        },
        {
            name: parse_six,
            parsed: '6',
            expected_type: Type::Integer,
            expected_value: Some('6')
        },
        {
            name: parse_seven,
            parsed: '7',
            expected_type: Type::Integer,
            expected_value: Some('7')
        },
        {
            name: parse_eight,
            parsed: '8',
            expected_type: Type::Integer,
            expected_value: Some('8')
        },
        {
            name: parse_nine,
            parsed: '9',
            expected_type: Type::Integer,
            expected_value: Some('9')
        },
        {
            name: parse_plus,
            parsed: '+',
            expected_type: Type::Plus,
            expected_value: Some('+')
        },
        {
            name: parse_eof,
            parsed: '\n',
            expected_type: Type::Eof,
            expected_value: None
        },
    );

    #[test]
    #[should_panic]
    fn parse_wrong_symbol() {
        tokenize('a');
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
    );

    #[test]
    #[should_panic]
    fn failed_evaluate() {
        Interpreter::evaluate("12+2\n");
    }
}
