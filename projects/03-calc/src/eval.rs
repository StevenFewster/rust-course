use std::collections::HashMap;

use crate::{CalcError, Expr};

/// Evaluate an expression with the given variables in scope.
pub fn eval(expr: &Expr, vars: &HashMap<String, f64>) -> Result<f64, CalcError> {
    let _ = (expr, vars);
    todo!("milestone 3: one recursive `match` over `expr`")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::tokenize, parser::parse};

    fn run(src: &str, vars: &HashMap<String, f64>) -> Result<f64, CalcError> {
        eval(&parse(&tokenize(src)?)?, vars)
    }

    #[test]
    fn arithmetic() {
        let v = HashMap::new();
        assert_eq!(run("2 + 3 * 4", &v), Ok(14.0));
        assert_eq!(run("(2 + 3) * 4", &v), Ok(20.0));
        assert_eq!(run("-3 * -2", &v), Ok(6.0));
        assert_eq!(run("10 / 4", &v), Ok(2.5));
    }

    #[test]
    fn variables() {
        let v = HashMap::from([("r".to_string(), 2.0)]);
        assert_eq!(run("r * r + 1", &v), Ok(5.0));
        assert_eq!(run("q", &v), Err(CalcError::UnknownVariable("q".into())));
    }

    #[test]
    fn division_by_zero() {
        assert_eq!(
            run("1 / (2 - 2)", &HashMap::new()),
            Err(CalcError::DivisionByZero)
        );
    }

    #[test]
    fn error_messages_are_readable() {
        let msg = CalcError::UnknownVariable("q".into()).to_string();
        assert!(msg.contains('q'), "message should name the variable: {msg}");
    }
}
