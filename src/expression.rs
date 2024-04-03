use serde::Serialize;

use super::token::Token;

// TODO: Fix proper visibility and imports for modules

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

#[derive(Serialize)]
pub enum UnaryOp {
    Not,
    Neg
}

#[derive(Serialize)]
pub struct Assign {
    name: String,
    value: Box<Expr>
}

#[derive(Serialize)]
pub struct Binary {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: BinaryOp,
}

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

#[derive(Serialize)]
pub struct Grouping {
    expression: Box<Expr>,
}

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

#[derive(Serialize)]
pub struct Unary {
    operand: Box<Expr>,
    operator: UnaryOp,
}

#[derive(Serialize)]
pub struct Variable {
    name: String,
}

#[derive(Serialize)]
pub enum LiteralValue {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

impl Expr {
    pub fn print(&self) {
        let expr_json = serde_json::to_string_pretty(self).unwrap();
        println!("{}", expr_json);
    }
}

impl Binary {
    pub fn new(left: Expr, right: Expr, operator: BinaryOp) -> Self {
        // Add error checking code to panic if the operator is not a binary operator
        Binary { left: Box::new(left), right: Box::new(right), operator }         
    }
}

impl Unary {
    pub fn new(operand: Expr, operator: UnaryOp) -> Self {
        // Add error checking code to panic if the operator is not a binary operator
        Unary { operand: Box::new(operand), operator }
    }
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Literal { value }
    }
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
}

// trait ExprVisitor<R> {
//     fn visit_assign(&mut self, expr: &Assign) -> R;
//     fn visit_binary(&mut self, expr: &Binary) -> R;
//     fn visit_call(&mut self, expr: &Call) -> R;
//     fn visit_get(&mut self, expr: &Get) -> R;
//     fn visit_grouping(&mut self, expr: &Grouping) -> R;
//     fn visit_literal(&mut self, expr: &Literal) -> R;
//     fn visit_logical(&mut self, expr: &Logical) -> R;
//     fn visit_set(&mut self, expr: &Set) -> R;
//     fn visit_super(&mut self, expr: &Super) -> R;
//     fn visit_this(&mut self, expr: &This) -> R;
//     fn visit_unary(&mut self, expr: &Unary) -> R;
//     fn visit_variable(&mut self, expr: &Variable) -> R;
// }
