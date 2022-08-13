use std::result;

use crate::Token;

mod single_char_consumer;
mod string_consumer;

pub type Result<TokenTyp> = result::Result<Option<Token<TokenTyp>>, &'static str>;
