//! Lox statements.
//!
//! Lox statements are also modelled as a tree structure similarly to expressions.

use serde::Serialize;

use crate::Diagnostic;

use super::expression::Expr;

/// Statement types.
#[derive(Serialize)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

impl Stmt {
    /// Prints the statement tree to the standard output in JSON format using [`serde_json`].
    ///
    /// # Panics
    /// Meant to be used for debugging purposes only.
    /// Unintended side-effects may arise.
    /// Refer to [`serde_json`] documentation for safe usage.
    pub fn print(&self) {
        let stmt_json = serde_json::to_string_pretty(self).unwrap();
        println!("{}", stmt_json);
    }

    pub fn execute(&self) -> Result<(), Diagnostic> {
        match self {
            Stmt::Expression(expr) => expr.eval().map(|_| Ok(()))?,
            Stmt::Print(expr) => {
                println!("{}", expr.eval()?);
                Ok(())
            }
        }
    }
}
