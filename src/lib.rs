pub mod expression;
pub mod peg_parser;
pub mod statement;

use peg_parser::lox_parser;

use std::fs;
use std::io;
use std::io::Write;
use std::process;

// TODO: Reconsider the types of errors
#[derive(Debug, PartialEq)]
pub enum Diagnostic {
    LoxError {
        line: usize,
        message: String,
    },
    ParseError {
        error: peg::error::ParseError<<str as peg::Parse>::PositionRepr>,
    },
}

/// Loads a file and executes it.
///
/// If an I/O error is occured it returns the error and terminates early.
/// If an error is occured in the users program, it prints the diagnostic and terminates.
pub fn run_file(path: &str) -> io::Result<()> {
    let file = fs::read_to_string(path)?;
    if let Err(err) = run(&file) {
        let exit_code = match &err {
            Diagnostic::LoxError {
                line: _,
                message: _,
            } => 70, // EX_SOFTWARE
            Diagnostic::ParseError { error: _ } => 65, // EX_DATAERR
        };
        error(err);
        process::exit(exit_code);
    }
    Ok(())
}

/// Starts a prompt, accepting input from the user and executing the code when a newline occurs.
///
/// The prompt can be exited with `Ctrl-D`.
/// If an error occurs the diagnostic is printed to the user and execution continues.
///
/// # Errors
///
/// This function returns a [`std::io::Result`], terminating early if an I/O error occurs.
pub fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.is_empty() {
            println!();
            break Ok(());
        };
        match run(&line) {
            Ok(()) => (),
            Err(err) => error(err),
        }
    }
}

/// Executes the source code and returns a diagnostic if an error occurs.
pub fn run(source: &str) -> Result<(), Diagnostic> {
    let stmts = lox_parser::program(source)?;
    for stmt in &stmts {
        stmt.print();
    }
    for stmt in stmts {
        stmt.execute()?
    }
    Ok(())
}

impl From<peg::error::ParseError<<str as peg::Parse>::PositionRepr>> for Diagnostic {
    // TODO: Audit
    fn from(value: peg::error::ParseError<<str as peg::Parse>::PositionRepr>) -> Diagnostic {
        Diagnostic::ParseError { error: value }
    }
}

// TODO: Consider consolididating error and report as an implementation of `std::fmt::Display`
// TODO: Remove report
/// Prints a diagnostic to the standard error.
///
/// # Panics
///
/// Panics if writting to [`std::io::stderr`] fails.
pub fn error(diagnostic: Diagnostic) {
    match diagnostic {
        Diagnostic::LoxError { line, message } => report(line, "", &message),
        Diagnostic::ParseError { error } => eprintln!("Parse Error: {error}"),
    }
}

// Prints a formated diagnostic to standard error.
fn report(line: usize, err_where: &str, message: &str) {
    eprintln!("[Line: {line}] Error{err_where}: {message}");
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
