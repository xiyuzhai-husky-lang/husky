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
            WordPtr::RawOpnVariant(opr) => opr.deref(),
            WordPtr::Decorator(decorator) => decorator.deref(),
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

pub fn new_word_interner() -> WordAllocator {
    WordAllocator::new(&[
        ConfigKeyword::Dataset.into(),
        Keyword::Use.into(),
        Keyword::Mod.into(),
        Keyword::Main.into(),
        Keyword::Visual.into(),
        LiasonKeyword::Mut.into(),
        Paradigm::LazyFunctional.into(),
        Paradigm::EagerProcedural.into(),
        Paradigm::EagerFunctional.into(),
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
        RootIdentifier::Debug.into(),
        RootIdentifier::Std.into(),
        RootIdentifier::Core.into(),
        RootIdentifier::Debug.into(),
        RootIdentifier::Void.into(),
        RootIdentifier::I32.into(),
        RootIdentifier::F32.into(),
        RootIdentifier::B32.into(),
        RootIdentifier::B64.into(),
        RootIdentifier::Bool.into(),
        RootIdentifier::True.into(),
        RootIdentifier::False.into(),
        RootIdentifier::Vec.into(),
        RootIdentifier::Array.into(),
        RootIdentifier::Tuple.into(),
        RootIdentifier::Fp.into(),
        RootIdentifier::Fn.into(),
        RootIdentifier::FnMut.into(),
        RootIdentifier::FnOnce.into(),
        RootIdentifier::Datasets.into(),
        RootIdentifier::DatasetType.into(),
        RootIdentifier::VisualType.into(),
        RootIdentifier::CloneTrait.into(),
        RootIdentifier::CopyTrait.into(),
        RootIdentifier::PartialEqTrait.into(),
        RootIdentifier::EqTrait.into(),
        RootIdentifier::TypeType.into(),
        RootIdentifier::ModuleType.into(),
        RootIdentifier::Ref.into(),
        ContextualIdentifier::Crate.into(),
        ContextualIdentifier::Input.into(),
        ContextualIdentifier::ThisValue.into(),
        ContextualIdentifier::ThisType.into(),
        WordOpr::And.into(),
        WordOpr::Or.into(),
        WordOpr::As.into(),
        Decorator::Pub.into(),
        Decorator::Private.into(),
        Decorator::Async.into(),
        Decorator::Static.into(),
    ])
}

pub trait InternWord {
    fn word_allocator(&self) -> &WordAllocator;
    fn intern_word(&self, word: &str) -> WordPtr {
        self.word_allocator().alloc_from_ref(word)
    }
    fn custom_ident(&self, word: &str) -> CustomIdentifier {
        self.intern_word(word).opt_custom().unwrap()
    }
}
