use super::expression::*;

// TODO: Add quiet! and expect! error messages for identifiers, etc.
peg::parser! {
    /// Parser for Lox language grammar, currently supporting expressions only
    pub grammar lox_parser() for str {


        pub rule expression() -> Expr = equality()
        // pub rule expression() -> Expr = unary()

        // pub rule assignment() = ( call() "." )? IDENTIFIER() "=" assignment() / logic_or()

        // pub rule logic_or() = logic_and() ( "or" logic_and() )*
        // pub rule logic_and() = equality() ( "and" equality() )*
        rule equality() -> Expr = left:comparison() right:equality_pure()* { if right.is_empty() {left} else {flatten_binary(left,right)} }
        rule equality_pure() -> (BinaryOp, Expr) = op:(EQ() / NE()) expr:comparison() { (op, expr) }


        rule comparison() -> Expr = left:term() right:comparison_pure()* { if right.is_empty() {left} else {flatten_binary(left,right)} }
        rule comparison_pure() -> (BinaryOp, Expr) = op:(LE() / GE() / GT() / LT()) expr:term() { (op, expr) }

        rule term() -> Expr = left:factor() right:term_pure()*  { if right.is_empty() {left} else {flatten_binary(left, right)} }
        rule term_pure() -> (BinaryOp, Expr) = op:(ADD() / SUB()) expr:factor() { (op, expr) }

        rule factor() -> Expr = left:unary() right:factor_pure()* { if right.is_empty() { left } else { flatten_binary(left, right) } }
        rule factor_pure() -> (BinaryOp, Expr) = op:(DIV() / MUL()) expr:unary() { (op, expr) }

        rule unary() -> Expr = unary_pure() / primary()
        rule unary_pure() -> Expr = op:(NOT() / NEG()) expr:unary() { Expr::Unary(Unary::new(expr, op))}

        // pub rule call() = primary() ( "(" arguments()? ")" / "." IDENTIFIER() )*
        rule primary() -> Expr = literal() / variable() / brackets()
                             // / "super" "." IDENTIFIER()


        rule literal() -> Expr = literal:(TRUE_LITERAL() / FALSE_LITERAL() / NUMBER_LITERAL() / STRING_LITERAL() / NIL_LITERAL()) { Expr::Literal(literal) }
        rule variable() -> Expr = _ ident:$IDENTIFIER() _ { Expr::Variable(Variable::new(ident.to_string())) }
        rule brackets() -> Expr = _ "(" _ expr:expression() _ ")" _ { expr }

        // pub rule function() = IDENTIFIER() "(" parameters? ")" block()
        // parameters     → IDENTIFIER ( "," IDENTIFIER )*
        // rule arguments() -> Vec<Expr> = expression() ** ","

        // rule THIS() -> Expr = "this" { Expr::This }
        rule NUMBER_LITERAL() -> Literal = _ num:NUMBER() _ { Literal::new(LiteralValue::Number(num)) }
        rule STRING_LITERAL() -> Literal = _ string:STRING() _ { Literal::new(LiteralValue::String(string)) }
        rule TRUE_LITERAL() -> Literal = _ "true" _ { Literal::new(LiteralValue::Bool(true)) }
        rule FALSE_LITERAL() -> Literal = _ "false" _ { Literal::new(LiteralValue::Bool(true)) }
        rule NIL_LITERAL() -> Literal = _ "nil" _  { Literal::new(LiteralValue::Nil) }

        rule NEG() -> UnaryOp = _ "-" _ { UnaryOp::Neg }
        rule NOT() -> UnaryOp = _ "!" _ { UnaryOp::Not }
        rule LT() -> BinaryOp = _ "<" _ { BinaryOp::Less }
        rule LE() -> BinaryOp = _ "<=" _ { BinaryOp::LessEqual }
        rule GT() -> BinaryOp = _ ">" _ { BinaryOp::Greater }
        rule GE() -> BinaryOp = _ ">=" _ { BinaryOp::GreaterEqual }
        rule EQ() -> BinaryOp = _ "==" _ { BinaryOp::Equal }
        rule NE() -> BinaryOp = _ "!=" _ { BinaryOp::NotEqual }
        rule MUL() -> BinaryOp = _ "*" _ { BinaryOp::Mul }
        rule DIV() -> BinaryOp = _ "/" _ { BinaryOp::Mul }
        rule ADD() -> BinaryOp = _ "+" _ { BinaryOp::Add }
        rule SUB() -> BinaryOp = _ "-" _ { BinaryOp::Sub }

        pub rule NUMBER() -> f64 = num:$(DIGIT()+ ( "." DIGIT()+)?) { num.parse().unwrap() }
        pub rule STRING() -> String = "\"" string:$([^'"']*) "\"" { String::from(string) }
        rule IDENTIFIER() = ALPHA() ( ALPHA() / DIGIT() )*
        rule ALPHA() = ['a'..='z' | 'A'..='Z' | '_']
        rule DIGIT() = ['0'..='9']

        // Match whitespace and comments
        rule _ = "//"[^'\n']*['\n'] / [' ' | '\n' | '\r' |'\t']*
    }
}

fn flatten_binary(left: Expr, mut expr_list: Vec<(BinaryOp, Expr)>) -> Expr {
    let (op, right) = expr_list.pop().expect("Factors list should never be zero");
    let left_expr = if expr_list.is_empty() {
        left
    } else {
        flatten_binary(left, expr_list)
    };
    Expr::Binary(Binary::new(left_expr, right, op))
}

// TODO: Add tests for the rest of the parser
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!(
            lox_parser::STRING("\"Hello World\""),
            Ok("Hello World".to_string())
        );
        assert!(lox_parser::STRING("Hello World").is_err());
        assert!(lox_parser::STRING("Hello World\"").is_err());
        assert!(lox_parser::STRING("\"Hello World").is_err());
    }

    #[test]
    fn number() {
        assert_eq!(lox_parser::NUMBER("1.2345"), Ok(1.2345));
        assert_eq!(lox_parser::NUMBER("12345"), Ok(12345f64));
        assert!(lox_parser::NUMBER("12345asdf").is_err());
        assert!(lox_parser::NUMBER("123,45").is_err());
    }

    #[test]
    fn variable() {}
}
