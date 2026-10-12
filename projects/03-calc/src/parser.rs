use crate::{CalcError, Expr, Token};

/// Parse a full token list into a single expression. Leftover tokens are an error.
pub fn parse(tokens: &[Token]) -> Result<Expr, CalcError> {
    let _ = tokens;
    todo!("milestone 2: recursive descent – see the grammar in the README")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Op, lexer::tokenize};

    fn p(src: &str) -> Result<Expr, CalcError> {
        parse(&tokenize(src).expect("tests assume the lexer works"))
    }

    fn num(n: f64) -> Box<Expr> {
        Box::new(Expr::Num(n))
    }

    #[test]
    fn single_number() {
        assert_eq!(p("42").unwrap(), Expr::Num(42.0));
    }

    #[test]
    fn precedence() {
        // 1 + (2 * 3)
        assert_eq!(
            p("1 + 2 * 3").unwrap(),
            Expr::Binary {
                op: Op::Add,
                left: num(1.0),
                right: Box::new(Expr::Binary {
                    op: Op::Mul,
                    left: num(2.0),
                    right: num(3.0)
                }),
            }
        );
    }

    #[test]
    fn left_associative() {
        // (8 - 3) - 2
        assert_eq!(
            p("8 - 3 - 2").unwrap(),
            Expr::Binary {
                op: Op::Sub,
                left: Box::new(Expr::Binary {
                    op: Op::Sub,
                    left: num(8.0),
                    right: num(3.0)
                }),
                right: num(2.0),
            }
        );
    }

    #[test]
    fn parentheses_and_unary_minus() {
        assert_eq!(
            p("-(x)").unwrap(),
            Expr::Neg(Box::new(Expr::Var("x".into())))
        );
    }

    #[test]
    fn errors() {
        assert_eq!(p("2 +"), Err(CalcError::UnexpectedEnd));
        assert_eq!(p("(1"), Err(CalcError::UnexpectedEnd));
        assert_eq!(p("1 )"), Err(CalcError::UnexpectedToken(Token::RParen)));
    }
}
