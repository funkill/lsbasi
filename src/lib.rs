#![feature(try_trait)]

use std::{
    fmt::{Display, Formatter},
    num::ParseIntError,
    option::NoneError,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Whitespace,
    Integer,
    Minus,
    Plus,
    Mul,
    Div,
    Eof,
}

struct Token {
    _type: Type,
    value: Option<String>,
}

impl Token {
    fn is_ops(&self) -> bool {
        match self._type {
            Type::Minus | Type::Plus | Type::Mul | Type::Div => true,
            _ => false,
        }
    }

    fn is_eof(&self) -> bool {
        self._type == Type::Eof
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(text: impl ToString) -> Result<Option<i64>, Error> {
        let stream = Self::tokenize(text.to_string());
        if let Some(first) = stream.first() {
            if first.is_eof() {
                return Ok(None);
            }

            if first.is_ops() {
                return Err(Error::with_message("Evaluated text must starts with integer!"));
            }

            let mut first = parse_number(first)?;
            let mut stream = stream.iter().skip(1);
            while let Some(op) = stream.next() {
                if op._type == Type::Eof {
                    break;
                }

                if !op.is_ops() {
                    return Err(Error::with_message("Item must be operand!"));
                }

                let second = match stream.next() {
                    Some(token) => token,
                    None => return Err(Error::with_message("Unexpected end of stream")),
                };
                let second = parse_number(&second)?;

                match &op._type {
                    Type::Plus => first += second,
                    Type::Minus => first -= second,
                    Type::Mul => first *= second,
                    Type::Div => {
                        if second == 0 {
                            return Err(Error::with_message("Division by zero!"));
                        }

                        first /= second
                    }
                    _ => unreachable!(),
                }
            }

            Ok(Some(first))
        } else {
            Ok(None)
        }
    }

    fn tokenize(text: String) -> Vec<Token> {
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
                if chars
                    .peek()
                    .map(detect_char_type)
                    .filter(|next_type| next_type == &curr_type)
                    .is_none()
                {
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

#[cfg(test)]
mod evaluate_tests {
    use super::Interpreter;

    macro_rules! add_test {
        ($($name:ident: $eval:expr, result: $res:expr,)+) => {
            $(
                #[test]
                fn $name() {
                    let result = Interpreter::evaluate($eval).unwrap().unwrap();
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
        eval_8: "1-2\n", result: -1,
        eval_9: "1*2\n", result: 2,
        eval_10: "4/2\n", result: 2,
        eval_11: "4 /2 * 5 + 5 - 3", result: 12,
    );

    #[test]
    fn check_division_by_zero() {
        let result = Interpreter::evaluate("1/0");

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn evaluate_empty_string() {
        let result = Interpreter::evaluate("");

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().is_none(), true);

        let result = Interpreter::evaluate("\n");

        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().is_none(), true);
    }

    macro_rules! check_strating_with_ops {
        ($({ $name:ident, eval: $eval:expr },)+) => {
            $(
                #[test]
                fn $name() {
                    let result = Interpreter::evaluate($eval);
                    assert_eq!(result.is_err(), true);
                }
            )+
        };
    }

    check_strating_with_ops!(
        {starting_with_add, eval: "+12"},
        {starting_with_sub, eval: "-12"},
        {starting_with_mul, eval: "*12"},
        {starting_with_div, eval: "/12"},
    );

    #[test]
    fn check_non_ops_operator() {
        let result = Interpreter::evaluate("1 1 +");

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn check_forgotten_second_operand() {
        let result = Interpreter::evaluate("1+");

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn check_wrong_first_argument() {
        let result = Interpreter::evaluate(" +1");

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn check_wrong_second_argument() {
        let result = Interpreter::evaluate("1++");

        assert_eq!(result.is_err(), true);
    }
}

fn parse_number(token: &Token) -> Result<i64, Error> {
    if token.is_ops() || token._type == Type::Eof || token._type == Type::Whitespace {
        return Err(Error::with_message("Evaluated text must be integer!"));
    }

    token.value.clone()?.parse().map_err(Into::into)
}

#[cfg(test)]
mod parse_number_tests {
    use super::{parse_number, Token, Type};

    #[test]
    fn parsing_number() {
        let token = Token {
            _type: Type::Integer,
            value: Some("0123456789".into()),
        };
        let value = parse_number(&token);

        assert_eq!(value.is_ok(), true);
        assert_eq!(value.ok(), Some(123456789));
    }

    macro_rules! parsing_non_integers {
        ($({ name: $name:ident, type: $type:expr},)+) => {
            $(
                #[test]
                fn $name() {
                    let token = Token { _type: $type, value: None };
                    let err = parse_number(&token);
                    assert_eq!(err.is_err(), true);
                }
            )+
        };
    }

    parsing_non_integers!(
        { name: parse_add, type: Type::Plus },
        { name: parse_sub, type: Type::Minus },
        { name: parse_mul, type: Type::Mul },
        { name: parse_div, type: Type::Div },
        { name: parse_eof, type: Type::Eof },
        { name: parse_whitespace, type: Type::Whitespace },
    );

    #[test]
    fn parse_none_value() {
        let token = Token {
            _type: Type::Integer,
            value: None,
        };
        let value = parse_number(&token);

        assert_eq!(value.is_err(), true);
    }

    #[test]
    fn parse_wrong_value() {
        let token = Token {
            _type: Type::Integer,
            value: Some("abc".into()),
        };
        let value = parse_number(&token);

        assert_eq!(value.is_err(), true);
    }
}

fn detect_char_type(item: &char) -> Type {
    match item {
        '0'...'9' => Type::Integer,
        ' ' => Type::Whitespace,
        '-' => Type::Minus,
        '+' => Type::Plus,
        '*' => Type::Mul,
        '/' => Type::Div,
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
            name: parse_minus,
            parsed: '-',
            expected_type: Type::Minus
        },
        {
            name: parse_plus,
            parsed: '+',
            expected_type: Type::Plus
        },
        {
            name: parse_mul,
            parsed: '*',
            expected_type: Type::Mul
        },
        {
            name: parse_div,
            parsed: '/',
            expected_type: Type::Div
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

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

impl Error {
    fn with_message(message: impl ToString) -> Self {
        let repr = Repr::Inner(message.to_string());

        Error { repr }
    }
}

#[derive(Debug)]
enum Repr {
    Inner(String),
    NoneError(NoneError),
    Other(Box<std::error::Error>),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &self.repr {
            Repr::Inner(message) => write!(f, "{}", message),
            Repr::NoneError(_) => write!(f, "Trying to unwrap None"),
            Repr::Other(e) => e.fmt(f),
        }
    }
}

impl From<NoneError> for Error {
    fn from(e: NoneError) -> Self {
        let repr = Repr::NoneError(e);
        Error { repr }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        let repr = Repr::Other(Box::new(e));
        Error { repr }
    }
}
