use super::token::Token;
use super::token::TokenType;
use super::Diagnostic;
use std::collections::HashMap;

// TODO: Add support for block comments
/// Scans for tokens and returns a collection of [`Token`]'s or a diagnostic if there is an error.
pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Diagnostic> {
    let keywords = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);
    let mut tokens = Vec::new();
    let mut line: usize = 1;
    let mut source = source.chars().peekable();
    // TODO: Change source.peek() to .next() and remove source.next() inside
    while let Some(char) = source.peek() {
        match char {
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::LeftParen,
                    line,
                });
                source.next();
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RightParen,
                    line,
                });
                source.next();
            }
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::LeftBrace,
                    line,
                });
                source.next();
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::RightBrace,
                    line,
                });
                source.next();
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    line,
                });
                source.next();
            }
            '.' => {
                tokens.push(Token {
                    token_type: TokenType::Dot,
                    line,
                });
                source.next();
            }
            '-' => {
                tokens.push(Token {
                    token_type: TokenType::Minus,
                    line,
                });
                source.next();
            }
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    line,
                });
                source.next();
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    line,
                });
                source.next();
            }
            '*' => {
                tokens.push(Token {
                    token_type: TokenType::Star,
                    line,
                });
                source.next();
            }
            '!' => {
                source.next();
                if let Some('=') = source.peek() {
                    tokens.push(Token {
                        token_type: TokenType::BangEqual,
                        line,
                    });
                    source.next();
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Bang,
                        line,
                    });
                }
            }
            '=' => {
                source.next();
                if let Some('=') = source.peek() {
                    tokens.push(Token {
                        token_type: TokenType::EqualEqual,
                        line,
                    });
                    source.next();
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Equal,
                        line,
                    });
                }
            }
            '<' => {
                source.next();
                if let Some('=') = source.peek() {
                    tokens.push(Token {
                        token_type: TokenType::LessEqual,
                        line,
                    });
                    source.next();
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Less,
                        line,
                    });
                }
            }
            '>' => {
                source.next();
                if let Some('=') = source.peek() {
                    tokens.push(Token {
                        token_type: TokenType::GreaterEqual,
                        line,
                    });
                    source.next();
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Greater,
                        line,
                    });
                }
            }
            '/' => {
                source.next();
                if let Some('/') = source.peek() {
                    while let Some(&char) = source.peek() {
                        if char != '\n' {
                            source.next();
                        } else {
                            break;
                        }
                    }
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Slash,
                        line,
                    });
                }
            }
            ' ' => {
                source.next();
            }
            '\r' => {
                source.next();
            }
            '\t' => {
                source.next();
            }
            '\n' => {
                line += 1;
                source.next();
            }
            '"' => {
                source.next();
                let mut string = String::new();
                loop {
                    match source.next() {
                        None | Some('\n') => {
                            return Err(Diagnostic::LoxError {
                                line,
                                message: format!("Unterminated string literal: \"{string}\""),
                            })
                        }
                        Some('"') => break,
                        Some(char) => string.push(char),
                    }
                }
                tokens.push(Token {
                    token_type: TokenType::String(string),
                    line,
                });
            }
            &num @ '0'..='9' => {
                let mut number = String::from(num);
                source.next();
                loop {
                    match source.peek() {
                        None => break,
                        Some(&num @ '0'..='9') => {
                            source.next();
                            number.push(num);
                        }
                        Some('.') => {
                            let mut source_next = source.clone();
                            source_next.next();
                            if let Some('0'..='9') = source_next.peek() {
                                number.push('.');
                                source.next();
                                while let Some(&num @ '0'..='9') = source.peek() {
                                    number.push(num);
                                    source.next();
                                }
                                break;
                            } else {
                                break;
                            }
                        }
                        Some(_) => break,
                    }
                }
                let num = number.parse::<f64>();
                match num {
                    Ok(num) => tokens.push(Token {
                        token_type: TokenType::Number(num),
                        line,
                    }),
                    Err(_) => {
                        return Err(Diagnostic::LoxError {
                            line,
                            message: format!("Invalid number literal: `{number}`"),
                        })
                    }
                }
            }
            // No reserved keyword is capitalized therefore it's not checked
            &char @ 'A'..='Z' => {
                source.next();
                let mut identifier = String::from(char);
                while let Some(&char @ ('A'..='Z' | 'a'..='z' | '0'..='9' | '_')) = source.peek() {
                    identifier.push(char);
                    source.next();
                }
            }
            &char @ 'a'..='z' => {
                source.next();
                let mut identifier = String::from(char);
                // NOTE: Potential optimization by checking if capital to exclude keywords
                while let Some(&char @ ('A'..='Z' | 'a'..='z' | '0'..='9' | '_')) = source.peek() {
                    identifier.push(char);
                    source.next();
                }
                match keywords.get(identifier.as_str()) {
                    Some(value) => tokens.push(Token {
                        token_type: value.clone(),
                        line,
                    }),
                    None => tokens.push(Token {
                        token_type: TokenType::Identifier(identifier),
                        line,
                    }),
                }
            }
            char => {
                return Err(Diagnostic::LoxError {
                    line,
                    message: format!("Unexpected character: '{char}'"),
                })
            }
        }
    }
    tokens.push(Token {
        token_type: TokenType::Eof,
        line,
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "\
// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= == // operators";

    #[test]
    fn comment() {
        let tokens = vec![Token {
            token_type: TokenType::Eof,
            line: 1,
        }];
        assert_eq!(scan_tokens(SOURCE.lines().nth(0).unwrap()), Ok(tokens))
    }

    #[test]
    fn delimiters() {
        let tokens = vec![
            Token {
                token_type: TokenType::LeftParen,
                line: 1,
            },
            Token {
                token_type: TokenType::LeftParen,
                line: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                line: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                line: 1,
            },
            Token {
                token_type: TokenType::LeftBrace,
                line: 1,
            },
            Token {
                token_type: TokenType::RightBrace,
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                line: 1,
            },
        ];
        assert_eq!(scan_tokens(SOURCE.lines().nth(1).unwrap()), Ok(tokens))
    }

    #[test]
    fn operators() {
        let tokens = vec![
            Token {
                token_type: TokenType::Bang,
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                line: 1,
            },
            Token {
                token_type: TokenType::Slash,
                line: 1,
            },
            Token {
                token_type: TokenType::Equal,
                line: 1,
            },
            Token {
                token_type: TokenType::Less,
                line: 1,
            },
            Token {
                token_type: TokenType::Greater,
                line: 1,
            },
            Token {
                token_type: TokenType::LessEqual,
                line: 1,
            },
            Token {
                token_type: TokenType::EqualEqual,
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                line: 1,
            },
        ];
        assert_eq!(scan_tokens(SOURCE.lines().nth(2).unwrap()), Ok(tokens))
    }

    #[test]
    fn basic() {
        let tokens = vec![
            Token {
                token_type: TokenType::LeftParen,
                line: 2,
            },
            Token {
                token_type: TokenType::LeftParen,
                line: 2,
            },
            Token {
                token_type: TokenType::RightParen,
                line: 2,
            },
            Token {
                token_type: TokenType::RightParen,
                line: 2,
            },
            Token {
                token_type: TokenType::LeftBrace,
                line: 2,
            },
            Token {
                token_type: TokenType::RightBrace,
                line: 2,
            },
            Token {
                token_type: TokenType::Bang,
                line: 3,
            },
            Token {
                token_type: TokenType::Star,
                line: 3,
            },
            Token {
                token_type: TokenType::Plus,
                line: 3,
            },
            Token {
                token_type: TokenType::Minus,
                line: 3,
            },
            Token {
                token_type: TokenType::Slash,
                line: 3,
            },
            Token {
                token_type: TokenType::Equal,
                line: 3,
            },
            Token {
                token_type: TokenType::Less,
                line: 3,
            },
            Token {
                token_type: TokenType::Greater,
                line: 3,
            },
            Token {
                token_type: TokenType::LessEqual,
                line: 3,
            },
            Token {
                token_type: TokenType::EqualEqual,
                line: 3,
            },
            Token {
                token_type: TokenType::Eof,
                line: 3,
            },
        ];
        assert_eq!(scan_tokens(SOURCE), Ok(tokens))
    }

    #[test]
    fn unexpected_character() {
        let source = "\
// the next line contains an error
!=/$";
        let error = Diagnostic::LoxError {
            line: 2,
            message: String::from("Unexpected character: '$'"),
        };
        assert_eq!(scan_tokens(source), Err(error));
    }

    #[test]
    fn string() {
        let source = "\
// this is a comment
\"Hello world!\" // this is a string literal";
        let tokens = vec![
            Token {
                token_type: TokenType::String(String::from("Hello world!")),
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        assert_eq!(scan_tokens(source), Ok(tokens));
    }

    #[test]
    fn unterminated_string() {
        let source = "\
// this is a comment
\"Hello world! // this is a string literal";
        let error = Diagnostic::LoxError {
            line: 2,
            message: String::from(
                "Unterminated string literal: \"Hello world! // this is a string literal\"",
            ),
        };
        assert_eq!(scan_tokens(source), Err(error));
    }

    #[test]
    fn number() {
        let source = "\
// this is a comment
1234 12.34 // this is a number literal";
        let source_digit = "0";
        let source_newline = "\
123
456
678";
        let tokens = vec![
            Token {
                token_type: TokenType::Number(1234f64),
                line: 2,
            },
            Token {
                token_type: TokenType::Number(12.34),
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        let tokens_digit = vec![
            Token {
                token_type: TokenType::Number(0f64),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                line: 1,
            },
        ];
        let tokens_newline = vec![
            Token {
                token_type: TokenType::Number(123f64),
                line: 1,
            },
            Token {
                token_type: TokenType::Number(456f64),
                line: 2,
            },
            Token {
                token_type: TokenType::Number(678f64),
                line: 3,
            },
            Token {
                token_type: TokenType::Eof,
                line: 3,
            },
        ];
        assert_eq!(scan_tokens(source), Ok(tokens));
        assert_eq!(scan_tokens(source_digit), Ok(tokens_digit));
        assert_eq!(scan_tokens(source_newline), Ok(tokens_newline));
    }

    #[test]
    fn invalid_number() {
        let source_unexpected_char = "\
// this is a comment
12$34 // this is an invalid number literal";
        let source_leading_dot = "\
// this is a comment
.123 // this is a number literal";
        let source_trailing_dot = "\
// this is a comment
123. // this is a number literal";
        let source_dots = "\
1.2.3 // this is a number literal
1 // this is line 2";
        let error_unexpected_char = Diagnostic::LoxError {
            line: 2,
            message: String::from("Unexpected character: '$'"),
        };
        let tokens_leading_dot = vec![
            Token {
                token_type: TokenType::Dot,
                line: 2,
            },
            Token {
                token_type: TokenType::Number(123f64),
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        let tokens_trailing_dot = vec![
            Token {
                token_type: TokenType::Number(123f64),
                line: 2,
            },
            Token {
                token_type: TokenType::Dot,
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        let tokens_dots = vec![
            Token {
                token_type: TokenType::Number(1.2f64),
                line: 1,
            },
            Token {
                token_type: TokenType::Dot,
                line: 1,
            },
            Token {
                token_type: TokenType::Number(3f64),
                line: 1,
            },
            Token {
                token_type: TokenType::Number(1f64),
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];

        assert_eq!(
            scan_tokens(source_unexpected_char),
            Err(error_unexpected_char)
        );
        assert_eq!(scan_tokens(source_leading_dot), Ok(tokens_leading_dot));
        assert_eq!(scan_tokens(source_trailing_dot), Ok(tokens_trailing_dot));
        assert_eq!(scan_tokens(source_dots), Ok(tokens_dots));
    }

    #[test]
    fn keyword() {
        let source = "\
// this is a comment
and class else false for fun if nil or print return super this true var while";
        let source_newline = "\
and // this is a comment
class
else
false
for
fun
if
nil
or
print
return
super
this
true
var
while";
        let tokens = vec![
            Token {
                token_type: TokenType::And,
                line: 2,
            },
            Token {
                token_type: TokenType::Class,
                line: 2,
            },
            Token {
                token_type: TokenType::Else,
                line: 2,
            },
            Token {
                token_type: TokenType::False,
                line: 2,
            },
            Token {
                token_type: TokenType::For,
                line: 2,
            },
            Token {
                token_type: TokenType::Fun,
                line: 2,
            },
            Token {
                token_type: TokenType::If,
                line: 2,
            },
            Token {
                token_type: TokenType::Nil,
                line: 2,
            },
            Token {
                token_type: TokenType::Or,
                line: 2,
            },
            Token {
                token_type: TokenType::Print,
                line: 2,
            },
            Token {
                token_type: TokenType::Return,
                line: 2,
            },
            Token {
                token_type: TokenType::Super,
                line: 2,
            },
            Token {
                token_type: TokenType::This,
                line: 2,
            },
            Token {
                token_type: TokenType::True,
                line: 2,
            },
            Token {
                token_type: TokenType::Var,
                line: 2,
            },
            Token {
                token_type: TokenType::While,
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        let tokens_newline = vec![
            Token {
                token_type: TokenType::And,
                line: 1,
            },
            Token {
                token_type: TokenType::Class,
                line: 2,
            },
            Token {
                token_type: TokenType::Else,
                line: 3,
            },
            Token {
                token_type: TokenType::False,
                line: 4,
            },
            Token {
                token_type: TokenType::For,
                line: 5,
            },
            Token {
                token_type: TokenType::Fun,
                line: 6,
            },
            Token {
                token_type: TokenType::If,
                line: 7,
            },
            Token {
                token_type: TokenType::Nil,
                line: 8,
            },
            Token {
                token_type: TokenType::Or,
                line: 9,
            },
            Token {
                token_type: TokenType::Print,
                line: 10,
            },
            Token {
                token_type: TokenType::Return,
                line: 11,
            },
            Token {
                token_type: TokenType::Super,
                line: 12,
            },
            Token {
                token_type: TokenType::This,
                line: 13,
            },
            Token {
                token_type: TokenType::True,
                line: 14,
            },
            Token {
                token_type: TokenType::Var,
                line: 15,
            },
            Token {
                token_type: TokenType::While,
                line: 16,
            },
            Token {
                token_type: TokenType::Eof,
                line: 16,
            },
        ];
        assert_eq!(scan_tokens(source), Ok(tokens));
        assert_eq!(scan_tokens(source_newline), Ok(tokens_newline));
        assert_eq!(source_newline.lines().count(), 16);
    }

    #[test]
    fn identifier() {
        let source = "\
foo bar
foobar";
        let source_keywords = "\
printnil
print nil";
        let tokens = vec![
            Token {
                token_type: TokenType::Identifier(String::from("foo")),
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier(String::from("bar")),
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier(String::from("foobar")),
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        let tokens_keywords = vec![
            Token {
                token_type: TokenType::Identifier(String::from("printnil")),
                line: 1,
            },
            Token {
                token_type: TokenType::Print,
                line: 2,
            },
            Token {
                token_type: TokenType::Nil,
                line: 2,
            },
            Token {
                token_type: TokenType::Eof,
                line: 2,
            },
        ];
        assert_eq!(scan_tokens(source), Ok(tokens));
        assert_eq!(scan_tokens(source_keywords), Ok(tokens_keywords));
    }
}
