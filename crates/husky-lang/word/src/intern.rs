use interner::Interner;

use crate::*;

pub type WordInterner = Interner<String, Word>;

pub fn new_word_interner() -> WordInterner {
    macro_rules! get_keyword_str {
        ($keyword: ident) => {{
            stringify!($keyword).to_lowercase()
        }};
    }

    macro_rules! build_keyword_bimap {
    [$($variant:ident),*] => {{
         vec![$((get_keyword_str!($variant), Word::Keyword(Keyword::$variant))),*]
    }};
}

    return WordInterner::new(build_keyword_bimap![
        Use, Mod, Main, Test, Proc, Func, Pattern, Struct, Rename, Enum, Props, Main, Let, Var, If,
        Elif, Else, Switch, Case, DeFault, For, Ext, ForExt, While, Do, Break, Return
    ]);
}

pub trait InternWord {
    fn provide_word_interner(&self) -> &WordInterner;

    fn string_to_word(&self, string: &str) -> Word {
        self.provide_word_interner().id_by_ref(string)
    }
}
