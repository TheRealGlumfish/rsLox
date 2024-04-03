use super::Diagnostic;
use super::token::{Token, TokenType};
use super::expression::*;

/// A parser for a Lox token stream.
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new parser which will parse the `tokens`.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }
    
    // Checks if the EOF token has been reached.
    // TODO: Consider removing the EOF token
    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.pos += 1
        }
        self.previous()
    }

    fn check_token(&self, token_type: &TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
         for token_type in token_types {
             if self.check_token(token_type) {
                 self.advance();
                 return true;
             }
         }
         false
    }

    fn expression(&mut self) -> Result<Expr, Diagnostic> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.term()?;
        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Diagnostic> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(expr, right, operator));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary::new(right, operator)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        todo!()
    }
}

