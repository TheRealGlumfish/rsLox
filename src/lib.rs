mod lexer;

use lexer::Token;

use std::fs;
use std::io;
use std::io::Write;
use std::process;

// TODO: Reconsider the types of errors
#[derive(Debug, PartialEq)]
pub enum Diagnostic {
    LoxError { line: usize, message: String },
    TokenError { token: Token, message: String },
}

pub fn run_file(path: &str) -> io::Result<()> {
    let file = fs::read_to_string(path)?;
    if let Err(err) = run(&file) {
        error(err);
        process::exit(65);
    }
    Ok(())
}

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

fn run(source: &str) -> Result<(), Diagnostic> {
    let tokens = lexer::scan_tokens(source)?;
    for token in tokens {
        println!("{token}");
    }
    Ok(()) // TODO: remove
}

fn error(diagnostic: Diagnostic) {
    match diagnostic {
        Diagnostic::LoxError { line, message } => report(line, "", &message),
        Diagnostic::TokenError { token, message } => todo!(),
    }
}

fn report(line: usize, err_where: &str, message: &str) {
    println!("[line {line} ] Error{err_where}: {message}");
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
