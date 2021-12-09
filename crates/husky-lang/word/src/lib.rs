use bimap::BiMap;

use interner::Interner;

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
    Pub,
    Mod,
    Def,
    Fn,
    Func,
    Pattern,
    Struct,
    Rename,
    Enum,
    Props,
    Tmpl,
    Main,
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

pub type WordInterner = Interner<String, Word>;

pub fn new_word_interner() -> WordInterner {
    macro_rules! get_keyword_str {
        ($keyword: ident) => {{
            stringify!($keyword).to_lowercase()
        }};
    }

    macro_rules! build_keyword_bimap {
        [$($variant:ident),*] => {{
            BiMap::from_iter(vec![$((get_keyword_str!($variant), Word::Keyword(Keyword::$variant))),*])
        }};
    }

    return WordInterner::new(build_keyword_bimap![
        Use, Pub, Mod, Def, Func, Pattern, Struct, Rename, Enum, Props, Tmpl, Main, Let, Var, If,
        Elif, Else, Switch, Case, DeFault, For, Ext, ForExt, While, Do, Break, Return
    ]);
}

pub trait InternWord {
    fn provide_word_interner(&self) -> &WordInterner;

    fn string_to_word(&self, raw: &str) -> Word {
        self.provide_word_interner().id_ref(raw)
    }

    fn word_to_string(&self, word: Word) -> String {
        self.provide_word_interner().thing(word)
    }
}
