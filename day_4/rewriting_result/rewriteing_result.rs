use anyhow::Result;
use std::iter::Peekable;
use std::str::Chars;
use thiserror::Error;

/// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    /// A reference to a variable.
    Var(String),
    /// A literal number.
    Number(u32),
    /// A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

#[derive(Error, Debug)]
enum TokenizerError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(String),
    #[error("Invalid 32-bit integer")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Number(num)))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Identifier(ident)))
            }
            '+' => Some(Ok(Token::Operator(Op::Add))),
            '-' => Some(Ok(Token::Operator(Op::Sub))),
            _ => Some(Err(TokenizerError::UnexpectedToken(c.to_string()))),
        }
    }
}

fn parse(input: &str) -> Result<Expression, TokenizerError> {
    let mut tokens = tokenize(input);

    fn parse_expr(tokens: &mut Tokenizer) -> Result<Expression, TokenizerError> {
        let Some(Ok(tok)) = tokens.next() else {
            return Err(TokenizerError::UnexpectedEndOfInput);
        };
        let left_expr = match tok {
            Token::Number(num) => {
                let v = num.parse()?;
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => return Err(TokenizerError::UnexpectedToken(format!("{tok:?}"))), // left side cannot be an operator
        };

        // Look ahead to parse a binary operation if present.
        Ok(match tokens.next() {
            None => left_expr,
            Some(Ok(Token::Operator(op))) => {
                Expression::Operation(Box::new(left_expr), op, Box::new(parse_expr(tokens)?))
            }
            Some(token) => return Err(TokenizerError::UnexpectedToken(format!("{token:?}"))),
        })
    }

    parse_expr(&mut tokens)
}

fn main() {
    let tokens = vec![
        "10+foo+20-30", // Valid
        "",             // Err(UnexpectedEndOfInput)
        "#",            // Err(UnexpectedToken("#"))
        "10foo+20-30",  // Err(UnexpectedToken(Identifier("foo")))
        "+20",          // Err(UnexpectedEndOfInput)
        "20+",          // Err(UnexpectedEndOfInput)
    ];

    for token in tokens {
        println!("'{token}':\t {:?}", parse(token));
    }
}

#[test]
fn test_valid_parse() {
    assert!(parse("10-30").is_ok());
    assert!(parse("a+20-30").is_ok());
    assert!(parse("a+b-c").is_ok());
    assert!(parse("10+n+20-30").is_ok());
}

#[test]
fn test_invalid_parse() {
    assert!(matches!(
        parse(""),
        Err(TokenizerError::UnexpectedEndOfInput)
    ));
    assert!(matches!(
        parse("a+"),
        Err(TokenizerError::UnexpectedEndOfInput)
    ));
    assert!(matches!(
        parse("a+-c"),
        Err(TokenizerError::UnexpectedToken(_))
    ));
    assert!(matches!(
        parse("10/30"),
        Err(TokenizerError::UnexpectedToken(_))
    ));
    assert!(matches!(
        parse("+30"),
        Err(TokenizerError::UnexpectedToken(_))
    ));
    assert!(matches!(
        parse("+30+"),
        Err(TokenizerError::UnexpectedToken(_))
    ));
    assert!(matches!(
        parse("30+"),
        Err(TokenizerError::UnexpectedEndOfInput)
    ));
    assert!(matches!(
        parse("10000000000000000000000000000000000000000"),
        Err(TokenizerError::InvalidNumber(_))
    ));
}
