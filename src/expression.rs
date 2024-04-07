//! Lox expressions.
//!
//! Lox expressions are modelled in a tree structure as a [`Expr`] type.
//! They can be evaluated using [`Expr::eval`], returning a [`LoxValue`] type.

use serde::Serialize;
use std::fmt;

use crate::Diagnostic;

use super::token::Token;

// TODO: Fix proper visibility and imports for modules

/// Expression types.
#[derive(Serialize)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Gropuping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This,
    Unary(Unary),
    Variable(Variable),
}

/// Assignment expression.
#[derive(Serialize)]
pub struct Assign {
    name: String,
    value: Box<Expr>,
}

/// Binary expression.
#[derive(Serialize)]
pub struct Binary {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: BinaryOp,
}

/// Binary expression operators.
#[derive(Serialize)]
pub enum BinaryOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Mul,
    Div,
    Add,
    Sub,
}

/// Function call expression.
#[derive(Serialize)]
pub struct Call {
    calle: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
}

#[derive(Serialize)]
pub struct Get {
    object: Box<Expr>,
    name: Token,
}

/// Grouping expression.
#[derive(Serialize)]
pub struct Grouping {
    expression: Box<Expr>,
}

/// Literal expression.
#[derive(Serialize)]
pub struct Literal {
    value: LiteralValue,
}

#[derive(Serialize)]
pub struct Logical {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token,
}

#[derive(Serialize)]
pub struct Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
}

#[derive(Serialize)]
pub struct Super {
    keyword: Token,
    method: Token,
}

/// Unary expression.
#[derive(Serialize)]
pub struct Unary {
    operand: Box<Expr>,
    operator: UnaryOp,
}

/// Unary expression operators.
#[derive(Serialize)]
pub enum UnaryOp {
    Not,
    Neg,
}

/// Variable expression.
#[derive(Serialize)]
pub struct Variable {
    name: String,
}

/// Literal type.
#[derive(Serialize)]
pub enum LiteralValue {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

impl Expr {
    /// Prints the expression tree to the standard output in JSON format using [`serde_json`].
    ///
    /// # Panics
    /// Meant to be used for debugging purposes only.
    /// Unintended side-effects may arise.
    /// Refer to [`serde_json`] documentation for safe usage.
    pub fn print(&self) {
        let expr_json = serde_json::to_string_pretty(self).unwrap();
        println!("{}", expr_json);
    }

    /// Evaluates an expression and returns its value, [`Diagnostic`] is returned if there is an
    /// error.
    pub fn eval(&self) -> Result<LoxValue, Diagnostic> {
        match self {
            Expr::Literal(expr) => expr.eval(),
            Expr::Unary(expr) => expr.eval(),
            Expr::Binary(expr) => expr.eval(),
            _ => todo!(),
        }
    }
}

impl Binary {
    pub fn new(left: Expr, right: Expr, operator: BinaryOp) -> Self {
        // Add error checking code to panic if the operator is not a binary operator
        Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        }
    }

    fn eval(&self) -> Result<LoxValue, Diagnostic> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;
        match self.operator {
            BinaryOp::Add => Binary::add(left, right),
            BinaryOp::Sub => Binary::sub(left, right),
            BinaryOp::Mul => Binary::mul(left, right),
            BinaryOp::Div => Binary::div(left, right),
            BinaryOp::Less => Binary::lt(left, right),
            BinaryOp::LessEqual => Binary::le(left, right),
            BinaryOp::Greater => Binary::gt(left, right),
            BinaryOp::GreaterEqual => Binary::ge(left, right),
            BinaryOp::Equal => Binary::eq(left, right),
            BinaryOp::NotEqual => Binary::ne(left, right),
        }
    }

    fn add(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                // TODO: Add line information
                match right {
                    LoxValue::Number(right) => Ok(LoxValue::Number(left + right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be added to value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be added to value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::String(left) => {
                match right {
                    LoxValue::String(right) => Ok(LoxValue::String(left + &right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type String cannot be added to value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type String cannot be added to value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be added".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!("value [{left}] of type {} cannot be added", left.type_str()),
            }),
        }
    }

    fn sub(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                // TODO: Add line information
                match right {
                    LoxValue::Number(right) => Ok(LoxValue::Number(left - right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be subtracted by value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be subtraced by value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be subtracted from".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be subtracted from",
                    left.type_str()
                ),
            }),
        }
    }

    fn mul(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                // TODO: Add line information
                match right {
                    LoxValue::Number(right) => Ok(LoxValue::Number(left * right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be multiplied by value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be multiplied by value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be multiplied".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be multiplied",
                    left.type_str()
                ),
            }),
        }
    }

    fn div(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                // TODO: Add line information
                match right {
                    LoxValue::Number(right) => Ok(LoxValue::Number(left / right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be divided by value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be divided by value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be divided".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be divided",
                    left.type_str()
                ),
            }),
        }
    }

    fn lt(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                match right {
                // TODO: Add line information
                    LoxValue::Number(right) => Ok(LoxValue::Bool(left < right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be compared".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be compared",
                    left.type_str()
                ),
            }),
        }
    }

    fn le(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                match right {
                    // TODO: Add line information
                    LoxValue::Number(right) => Ok(LoxValue::Bool(left <= right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be compared".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be compared",
                    left.type_str()
                ),
            }),
        }
    }

    fn gt(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                match right {
                    // TODO: Add line information
                    LoxValue::Number(right) => Ok(LoxValue::Bool(left > right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be compared".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be compared",
                    left.type_str()
                ),
            }),
        }
    }

    fn ge(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                match right {
                    // TODO: Add line information
                    LoxValue::Number(right) => Ok(LoxValue::Bool(left >= right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            }
            LoxValue::Nil => Err(Diagnostic::LoxError {
                line: 69,
                message: "value [Nil] cannot be compared".to_string(),
            }),
            _ => Err(Diagnostic::LoxError {
                line: 69,
                message: format!(
                    "value [{left}] of type {} cannot be compared",
                    left.type_str()
                ),
            }),
        }
    }

    fn eq(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                match right {
                // TODO: Add line information
                    LoxValue::Number(right) => Ok(LoxValue::Bool(left == right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            },
            LoxValue::String(left) => {
                match right {
                    LoxValue::String(right) => Ok(LoxValue::Bool(left == right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type String cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            },
            LoxValue::Bool(left) => {
                match right {
                    LoxValue::Bool(right) => Ok(LoxValue::Bool(left == right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Bool cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Bool cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            },
            LoxValue::Nil => {
                match right {
                    LoxValue::Nil => Ok(LoxValue::Bool(true)),
                    _ => Ok(LoxValue::Bool(false)),
                }
            },
        }
    }

    fn ne(left: LoxValue, right: LoxValue) -> Result<LoxValue, Diagnostic> {
        match left {
            LoxValue::Number(left) => {
                match right {
                // TODO: Add line information
                    LoxValue::Number(right) => Ok(LoxValue::Bool(left != right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            },
            LoxValue::String(left) => {
                match right {
                    LoxValue::String(right) => Ok(LoxValue::Bool(left != right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type String cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Number cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            },
            LoxValue::Bool(left) => {
                match right {
                    LoxValue::Bool(right) => Ok(LoxValue::Bool(left != right)),
                    LoxValue::Nil => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Bool cannot be compared with value [Nil]") }),
                    _ => Err(Diagnostic::LoxError { line: 69, message: format!("value [{left}] of type Bool cannot be compared with value [{right}] of type {}", right.type_str() ) })
                }
            },
            LoxValue::Nil => {
                match right {
                    LoxValue::Nil => Ok(LoxValue::Bool(false)),
                    _ => Ok(LoxValue::Bool(true)),
                }
            },
        }
    }
}

impl Unary {
    pub fn new(operand: Expr, operator: UnaryOp) -> Self {
        // Add error checking code to panic if the operator is not a binary operator
        Unary {
            operand: Box::new(operand),
            operator,
        }
    }

    fn eval(&self) -> Result<LoxValue, Diagnostic> {
        let operand = self.operand.eval()?;
        match self.operator {
            UnaryOp::Not => Ok(LoxValue::Bool(!operand.is_truthy())),
            UnaryOp::Neg => {
                // TODO: Add line/col information
                // Wrap in function/implement traits
                // TODO: Check if bools can be negated
                match operand {
                    LoxValue::Number(num) => Ok(LoxValue::Number(-num)),
                    LoxValue::Nil => Err(Diagnostic::LoxError {
                        line: 69,
                        message: "value [Nil] cannot be negated".to_string(),
                    }),
                    _ => Err(Diagnostic::LoxError {
                        line: 69,
                        message: format!(
                            "value [{operand}] of type {} cannot be negated",
                            operand.type_str()
                        )
                        .to_string(),
                    }),
                }
            }
        }
    }
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Literal { value }
    }

    // TODO: Consider making `LiteralValue` Lox value
    // TODO: Remove clone and make LoxValue hold string references
    fn eval(&self) -> Result<LoxValue, Diagnostic> {
        match &self.value {
            LiteralValue::Bool(val) => Ok(LoxValue::Bool(*val)),
            LiteralValue::String(val) => Ok(LoxValue::String(val.clone())),
            LiteralValue::Nil => Ok(LoxValue::Nil),
            LiteralValue::Number(val) => Ok(LoxValue::Number(*val)),
        }
    }
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
}

/// Lox value.
#[derive(Serialize)]
pub enum LoxValue {
    // TODO: Add object type
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl LoxValue {
    fn is_truthy(&self) -> bool {
        match self {
            LoxValue::Nil => false,
            LoxValue::Bool(val) => *val,
            _ => true,
        }
    }

    // Returns the type of the Lox value as a string litral
    fn type_str(&self) -> &str {
        match self {
            LoxValue::Number(_) => "Number",
            LoxValue::String(_) => "String",
            LoxValue::Bool(_) => "Bool",
            LoxValue::Nil => "Nil",
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::Number(num) => write!(f, "{num}"),
            LoxValue::Bool(val) => write!(f, "{val}"),
            LoxValue::String(string) => write!(f, "{string}"), // TODO: Maybe wrap in quotes?
            LoxValue::Nil => write!(f, "nil"),
        }
    }
}

// TODO: Add tests
