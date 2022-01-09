use interner::Interner;

use crate::*;

pub type WordInterner = Interner<String, Word>;

pub fn new_word_interner() -> WordInterner {
    return WordInterner::new_from::<&'static str, Word>(vec![
        ("dataset", ConfigKeyword::Dataset.into()),
        ("use", Keyword::Use.into()),
        ("mod", Keyword::Mod.into()),
        ("main", FuncKeyword::Main.into()),
        ("test", FuncKeyword::Test.into()),
        ("proc", FuncKeyword::Proc.into()),
        ("func", FuncKeyword::Func.into()),
        ("def", FuncKeyword::Def.into()),
        ("pattern", FuncKeyword::Pattern.into()),
        ("struct", TypeKeyword::Struct.into()),
        ("rename", TypeKeyword::Rename.into()),
        ("enum", TypeKeyword::Enum.into()),
        ("props", TypeKeyword::Props.into()),
        ("let", StmtKeyword::Let.into()),
        ("var", StmtKeyword::Var.into()),
        ("if", StmtKeyword::If.into()),
        ("elif", StmtKeyword::Elif.into()),
        ("else", StmtKeyword::Else.into()),
        ("switch", StmtKeyword::Switch.into()),
        ("match", StmtKeyword::Match.into()),
        ("case", StmtKeyword::Case.into()),
        ("default", StmtKeyword::DeFault.into()),
        ("for", StmtKeyword::For.into()),
        ("ext", StmtKeyword::Ext.into()),
        ("forext", StmtKeyword::ForExt.into()),
        ("while", StmtKeyword::While.into()),
        ("do", StmtKeyword::Do.into()),
        ("break", StmtKeyword::Break.into()),
        ("return", StmtKeyword::Return.into()),
        ("assert", StmtKeyword::Assert.into()),
        ("builtin", BuiltinIdentifier::Debug.into()),
        ("std", BuiltinIdentifier::Std.into()),
        ("core", BuiltinIdentifier::Core.into()),
        ("debug", BuiltinIdentifier::Debug.into()),
        ("i32", BuiltinIdentifier::I32.into()),
        ("f32", BuiltinIdentifier::F32.into()),
        ("Vec", BuiltinIdentifier::Vector.into()),
        ("Array", BuiltinIdentifier::Array.into()),
        ("Tuple", BuiltinIdentifier::Tuple.into()),
        ("Fp", BuiltinIdentifier::Fp.into()),
        ("Fn", BuiltinIdentifier::Fn.into()),
        ("FnMut", BuiltinIdentifier::FnMut.into()),
        ("FnOnce", BuiltinIdentifier::FnOnce.into()),
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
