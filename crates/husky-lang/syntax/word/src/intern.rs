use std::{borrow::Borrow, ops::Deref};

use interner::{InternId, Interner};

use crate::{ident::ImplicitIdentifier, *};

pub type WordInterner = Interner<str, String, WordId>;

impl Deref for WordId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            WordId::Keyword(keyword) => keyword.deref(),
            WordId::Identifier(ident) => ident.deref(),
        }
    }
}

impl Borrow<str> for WordId {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl From<&'static str> for WordId {
    fn from(target: &'static str) -> Self {
        Self::Identifier(Identifier::Custom(CustomIdentifier(target)))
    }
}

impl InternId for WordId {
    type Thing = str;
}

pub fn new_word_interner() -> WordInterner {
    WordInterner::new(&[
        ConfigKeyword::Dataset.into(),
        Keyword::Use.into(),
        Keyword::Mod.into(),
        FuncKeyword::Main.into(),
        FuncKeyword::Test.into(),
        FuncKeyword::Proc.into(),
        FuncKeyword::Func.into(),
        FuncKeyword::Def.into(),
        TypeKeyword::Struct.into(),
        TypeKeyword::Rename.into(),
        TypeKeyword::Enum.into(),
        TypeKeyword::Props.into(),
        StmtKeyword::Let.into(),
        StmtKeyword::Var.into(),
        StmtKeyword::If.into(),
        StmtKeyword::Elif.into(),
        StmtKeyword::Else.into(),
        StmtKeyword::Switch.into(),
        StmtKeyword::Match.into(),
        StmtKeyword::Case.into(),
        StmtKeyword::DeFault.into(),
        StmtKeyword::For.into(),
        StmtKeyword::ForExt.into(),
        StmtKeyword::While.into(),
        StmtKeyword::Do.into(),
        StmtKeyword::Break.into(),
        StmtKeyword::Return.into(),
        StmtKeyword::Assert.into(),
        BuiltinIdentifier::Debug.into(),
        BuiltinIdentifier::Std.into(),
        BuiltinIdentifier::Core.into(),
        BuiltinIdentifier::Debug.into(),
        BuiltinIdentifier::I32.into(),
        BuiltinIdentifier::F32.into(),
        BuiltinIdentifier::Vector.into(),
        BuiltinIdentifier::Array.into(),
        BuiltinIdentifier::Tuple.into(),
        BuiltinIdentifier::Fp.into(),
        BuiltinIdentifier::Fn.into(),
        BuiltinIdentifier::FnMut.into(),
        BuiltinIdentifier::FnOnce.into(),
        BuiltinIdentifier::DatasetType.into(),
        ImplicitIdentifier::Input.into(),
    ])
}

pub trait InternWord {
    fn word_interner(&self) -> &WordInterner;
    fn intern_word(&self, word: &str) -> WordId {
        self.word_interner().intern_ref(word)
    }
}
