---
minutes: 30
---

# Exercise: Expression Evaluation

Let's write a simple recursive evaluator for arithmetic expressions.

The `Box` type here is a smart pointer, and will be covered in detail later in
the course. An expression can be "boxed" with `Box::new` as seen in the tests.
To evaluate a boxed expression, use the deref operator (`*`) to "unbox" it:
`eval(*boxed_expr)`.

Some expressions cannot be evaluated and will return an error. The standard
[`Result<Value, String>`](https://doc.rust-lang.org/std/result/enum.Result.html)
type is an enum that represents either a successful value (`Ok(Value)`) or an
error (`Err(String)`). We will cover this type in detail later.

Copy and paste the code into the Rust playground, and begin implementing `eval`.
The final product should pass the tests. It may be helpful to use `todo!()` and
get the tests to pass one-by-one. You can also skip a test temporarily with
`#[ignore]`:

```none
#[test]
#[ignore]
fn test_value() { .. }
```

If you finish early, try writing a test that results in division by zero or
integer overflow. How could you handle this with `Result` instead of a panic?

```rust
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    todo!()
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

```
