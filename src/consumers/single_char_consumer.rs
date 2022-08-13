use std::fmt::Debug;

use var_8::ToUTF8Chars;

use crate::Token;

use super::Result;

pub fn single_char_consumer<'a, Typ>(wanted: &str, inp: &str, typ: &'static Typ) -> Result<Typ>
where
    Typ: PartialEq + Eq + Debug,
{
    if let Some(char) = inp.utf8_chars().next() {
        if char.is(wanted) {
            return Ok(Some(Token {
                typ,
                val: inp.to_owned(),
            }));
        }

        return Ok(None);
    }

    Err("single character consumption failed, end of input reached")
}

#[cfg(test)]
mod test {
    use crate::Token;

    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    struct TokenTyp;

    #[test]
    fn not_matching() {
        assert_eq!(single_char_consumer("+", ":", &TokenTyp), Ok(None))
    }

    #[test]
    fn matching() {
        assert_eq!(
            single_char_consumer(":", ":", &TokenTyp),
            Ok(Some(mk_tok(":"))),
        )
    }

    #[test]
    fn matching_utf8() {
        assert_eq!(
            single_char_consumer("🏳️‍🌈", "🏳️‍🌈", &TokenTyp),
            Ok(Some(mk_tok("🏳️‍🌈"))),
        )
    }

    fn mk_tok(val: &str) -> Token<TokenTyp> {
        return Token {
            typ: &TokenTyp,
            val: val.to_owned(),
        };
    }
}
