use std::str::Chars;

use var_8::{ToUTF8Chars, UTF8Chars};

use crate::Token;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenTyp {
    Str,
}

pub fn string_consumer<'a>(inp: &str) -> Result<Token<TokenTyp>, &'static str> {
    let mut val = String::new();

    let mut previous_was_backslash = false;
    for char in inp.utf8_chars().skip(1) {
        if previous_was_backslash {
            // Simply push the next characters. As it's being escaped, it should
            // not be interpreted by any other logic.
            val += char.as_str();
            previous_was_backslash = false;
            continue;
        }

        if char.is("\\") {
            previous_was_backslash = true;
            continue;
        }

        let is_end = char.is("\"");
        if is_end {
            return Ok(Token {
                typ: &TokenTyp::Str,
                val,
            });
        }

        val += char.as_str();
    }

    Err("probably an unterminated string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!(
            string_consumer(&"\"this is a great string\"").unwrap(),
            mk_str("this is a great string"),
        );
    }

    #[test]
    fn string_with_quotes() {
        assert_eq!(
            string_consumer(&"\"what a \\\"great\\\" tokenizer\"").unwrap(),
            mk_str("what a \"great\" tokenizer"),
        )
    }

    fn mk_str(val: &str) -> Token<TokenTyp> {
        Token {
            typ: &TokenTyp::Str,
            val: val.to_owned(),
        }
    }
}
