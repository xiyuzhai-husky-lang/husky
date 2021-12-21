use interner::Interner;

use crate::*;

pub type WordInterner = Interner<String, Word>;

pub fn new_word_interner() -> WordInterner {
    return WordInterner::new_from::<&'static str, Word>(vec![
        ("use", Keyword::Use.into()),
        ("mod", Keyword::Mod.into()),
        ("main", Keyword::Main.into()),
        ("test", Keyword::Test.into()),
        ("proc", Keyword::Proc.into()),
        ("func", Keyword::Func.into()),
        ("def", Keyword::Def.into()),
        ("pattern", Keyword::Pattern.into()),
        ("struct", Keyword::Struct.into()),
        ("rename", Keyword::Rename.into()),
        ("enum", Keyword::Enum.into()),
        ("props", Keyword::Props.into()),
        ("main", Keyword::Main.into()),
        ("let", Keyword::Let.into()),
        ("var", Keyword::Var.into()),
        ("if", Keyword::If.into()),
        ("elif", Keyword::Elif.into()),
        ("else", Keyword::Else.into()),
        ("switch", Keyword::Switch.into()),
        ("match", Keyword::Match.into()),
        ("case", Keyword::Case.into()),
        ("default", Keyword::DeFault.into()),
        ("for", Keyword::For.into()),
        ("ext", Keyword::Ext.into()),
        ("forExt", Keyword::ForExt.into()),
        ("while", Keyword::While.into()),
        ("do", Keyword::Do.into()),
        ("break", Keyword::Break.into()),
        ("return", Keyword::Return.into()),
        ("builtin", Reserved::Debug.into()),
        ("std", Reserved::Std.into()),
        ("core", Reserved::Core.into()),
        ("debug", Reserved::Debug.into()),
        ("i32", Reserved::I32.into()),
        ("f32", Reserved::F32.into()),
        ("Vec", Reserved::Vec.into()),
        ("Tuple", Reserved::Tuple.into()),
        ("Fp", Reserved::Fp.into()),
        ("Fn", Reserved::Fn.into()),
        ("FnMut", Reserved::FnMut.into()),
        ("FnOnce", Reserved::FnOnce.into()),
    ]);
}

pub trait InternWord {
    fn word_interner(&self) -> &WordInterner;

    fn string_to_word(&self, string: &str) -> Word {
        self.word_interner().id_by_ref(string)
    }
}

pub fn convert_ident<T>(this: &dyn InternWord, ident: Identifier, f: impl FnOnce(&str) -> T) -> T {
    this.word_interner().apply(Word::Identifier(ident), f)
}
