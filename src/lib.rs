use std::fmt::Debug;

pub mod consumers;

mod tokenize;

#[derive(PartialEq, Eq, Debug)]
pub struct Token<Typ: PartialEq + Eq + Debug + 'static> {
    pub typ: &'static Typ,
    pub val: String,
}
