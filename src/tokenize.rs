// use std::str::Chars;

// use var_8::{ToUTF8Chars, UTF8Chars};

// use crate::Token;

// #[derive(PartialEq, Eq, Debug)]
// pub enum TokenTyp {
//     Str,
// }

// pub fn tokenize<'a>(inp: &'a str) -> Result<Token<'a, TokenTyp>, &'static str> {
//     // Skip opening quote
//     let inp = &inp[1..];

//     let mut bytes_len: usize = 1;

//     let mut previous_was_backslash = false;
//     for char in inp.utf8_chars() {
//         println!("{:?} {}", char, previous_was_backslash);
        
//         if previous_was_backslash {
//             bytes_len += char.bytes_len();
//             previous_was_backslash = false;
//             continue;
//         }

//         previous_was_backslash = char.is("\\");

//         let is_end = char.is("\"");
//         if is_end {
//             const EXCLUDE_END_QUOTE: usize = 1;

//             return Ok(Token {
//                 typ: &TokenTyp::Str,
//                 val: &inp.clone(),
//             });
//         }

//         bytes_len += char.bytes_len();
//     }

//     Err("probably an unterminated string")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn string() {
//         assert_eq!(
//             tokenize(&"\"this is a great string\"").unwrap(),
//             mk_str("this is a great string"),
//         );
//     }

//     #[test]
//     fn string_with_escaped_quote() {
//         assert_eq!(
//             tokenize(&"\"what a \\\"great\\\" tokenizer\"").unwrap(),
//             mk_str("what a \"great\" tokenizer"),
//         )
//     }

//     fn mk_str(val: &str) -> Token<TokenTyp> {
//         Token {
//             typ: &TokenTyp::Str,
//             val,
//         }
//     }
// }
