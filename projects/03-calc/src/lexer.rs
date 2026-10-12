use crate::{CalcError, Token};

/// Turn source text into tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let _ = input;
    todo!("milestone 1: walk `input.chars().peekable()` and match on each char")
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn empty() {
        assert_eq!(tokenize("   ").unwrap(), vec![]);
    }

    #[test]
    fn numbers_and_operators() {
        assert_eq!(
            tokenize("1 + 2.5*3").unwrap(),
            vec![Num(1.0), Plus, Num(2.5), Star, Num(3.0)]
        );
    }

    #[test]
    fn parens_and_minus() {
        assert_eq!(
            tokenize("-(4/2)").unwrap(),
            vec![Minus, LParen, Num(4.0), Slash, Num(2.0), RParen]
        );
    }

    #[test]
    fn identifiers_and_let() {
        assert_eq!(
            tokenize("let r_2 = r1").unwrap(),
            vec![
                Ident("let".into()),
                Ident("r_2".into()),
                Equals,
                Ident("r1".into())
            ]
        );
    }

    #[test]
    fn unexpected_char() {
        assert_eq!(tokenize("1 # 2"), Err(CalcError::UnexpectedChar('#')));
    }
}
