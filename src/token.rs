use serde::Serialize;
use std::fmt;

// TODO:: Consider chaning visiliblity
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

/// `TokenType` represents the different types of tokens used in the Lox lanaguage.
///
/// The grammar of the language is defined [here](https://craftinginterpreters.com/appendix-i.html).
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Misc.
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Consider using pretty printing or manually implmeneting display for token_type
        write!(f, "{:?}", self.token_type)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Identifier(string) => write!(f, "{string}"),
            TokenType::String(string) => write!(f, "{string}"),
            TokenType::Number(num) => write!(f, "{num}"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Bang => write!(f, "!"),
            token_type => write!(f, "{token_type}"),
        }
    }
}
