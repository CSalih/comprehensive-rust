---
minutes: 20
---

# Exercise: Rewriting with Result

The following implements a very simple parser for an expression language.
However, it handles errors by panicking. Rewrite it to instead use idiomatic
error handling and propagate errors to a return from `main`. Feel free to use
`thiserror` and `anyhow`.

HINT: start by fixing error handling in the `parse` function. Once that is
working correctly, update `Tokenizer` to implement
`Iterator<Item=Result<Token, TokenizerError>>` and handle that in the parser.

```rust
use std::iter::Peekable;
use std::str::Chars;

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

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Token::Number(num))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Token::Identifier(ident))
            }
            '+' => Some(Token::Operator(Op::Add)),
            '-' => Some(Token::Operator(Op::Sub)),
            _ => panic!("Unexpected character {c}"),
        }
    }
}

fn parse(input: &str) -> Expression {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Expression {
        let Some(tok) = tokens.next() else {
            panic!("Unexpected end of input");
        };
        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().expect("Invalid 32-bit integer'");
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => panic!("Unexpected token {tok:?}"),
        };
        // Look ahead to parse a binary operation if present.
        match tokens.next() {
            None => expr,
            Some(Token::Operator(op)) => Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)),
            ),
            Some(tok) => panic!("Unexpected token {tok:?}"),
        }
    }

    parse_expr(&mut tokens)
}

fn main() {
    let expr = parse("10+foo+20-30");
    println!("{expr:?}");
}
```
