mod intern;

pub use intern::{new_word_interner, InternWord, WordInterner};

use common::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Word {
    Keyword(Keyword),
    Identifier(Identifier),
}
impl From<u32> for Word {
    fn from(raw: u32) -> Self {
        Word::Identifier(Identifier(raw))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Identifier(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Use,
    Mod,
    Main,
    Test,
    Proc,
    Func,
    Pattern,
    Struct,
    Rename,
    Enum,
    Props,
    Let,
    Var,
    If,
    Elif,
    Else,
    Switch,
    Case,
    DeFault,
    For,
    Ext,
    ForExt,
    While,
    Do,
    Break,
    Return,
}

pub fn use_string<F, Q>(this: &(impl InternWord + ?Sized), word: Word, f: F) -> Q
where
    F: Fn(&str) -> Q,
{
    this.provide_word_interner().use_thing(word, f)
}
