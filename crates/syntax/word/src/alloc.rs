use std::{borrow::Borrow, ops::Deref};

use unique_allocator::{UniqueAllocator, UniqueAllocatorPtr};

use crate::{ident::ContextualIdentifier, *};

pub type WordAllocator = UniqueAllocator<str, String, WordPtr>;

impl Deref for WordPtr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            WordPtr::Keyword(keyword) => keyword.deref(),
            WordPtr::Identifier(ident) => ident.deref(),
        }
    }
}

impl Borrow<str> for WordPtr {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl From<&'static str> for WordPtr {
    fn from(target: &'static str) -> Self {
        Self::Identifier(Identifier::Custom(CustomIdentifier(target)))
    }
}

impl UniqueAllocatorPtr for WordPtr {
    type Thing = str;
}

pub fn new_word_unique_allocator() -> WordAllocator {
    WordAllocator::new(&[
        ConfigKeyword::Dataset.into(),
        Keyword::Use.into(),
        Keyword::Mod.into(),
        Keyword::Def.into(),
        Keyword::Main.into(),
        RoutineKeyword::Test.into(),
        RoutineKeyword::Proc.into(),
        RoutineKeyword::Func.into(),
        TyKeyword::Struct.into(),
        TyKeyword::Rename.into(),
        TyKeyword::Enum.into(),
        TyKeyword::Props.into(),
        TyKeyword::Record.into(),
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
        BuiltinIdentifier::True.into(),
        BuiltinIdentifier::False.into(),
        BuiltinIdentifier::Vec.into(),
        BuiltinIdentifier::Array.into(),
        BuiltinIdentifier::Tuple.into(),
        BuiltinIdentifier::Fp.into(),
        BuiltinIdentifier::Fn.into(),
        BuiltinIdentifier::FnMut.into(),
        BuiltinIdentifier::FnOnce.into(),
        BuiltinIdentifier::Datasets.into(),
        BuiltinIdentifier::DatasetType.into(),
        ContextualIdentifier::Input.into(),
        ContextualIdentifier::ThisData.into(),
        ContextualIdentifier::ThisType.into(),
    ])
}

pub trait InternWord {
    fn word_allocator(&self) -> &WordAllocator;
    fn intern_word(&self, word: &str) -> WordPtr {
        self.word_allocator().alloc_from_ref(word)
    }
    fn custom_ident(&self, word: &str) -> CustomIdentifier {
        self.intern_word(word).custom().unwrap()
    }
}
