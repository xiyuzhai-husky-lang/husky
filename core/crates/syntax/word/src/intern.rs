use crate::{ident::ContextualIdentifier, *};
use interner::{Intern, Interner};
use singleton::singleton;
use std::{borrow::Borrow, ops::Deref};

pub type WordInterner = Interner<str, String, WordPtr>;

singleton! { WordInterner }

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

impl Intern for WordPtr {
    type Thing = str;
}

pub fn new_word_interner() -> Arc<WordInternerSingletonKeeper> {
    Arc::new(
        WordInterner::new(&[
            ConfigKeyword::Task.into(),
            Keyword::Use.into(),
            Keyword::Mod.into(),
            Keyword::Main.into(),
            Keyword::Visual.into(),
            LiasonKeyword::Mut.into(),
            Paradigm::LazyFunctional.into(),
            Paradigm::EagerProcedural.into(),
            Paradigm::EagerFunctional.into(),
            TyKeyword::Struct.into(),
            TyKeyword::Enum.into(),
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
            RootIdentifier::Mor.into(),
            RootIdentifier::Fp.into(),
            RootIdentifier::Fn.into(),
            RootIdentifier::FnMut.into(),
            RootIdentifier::FnOnce.into(),
            RootIdentifier::Domains.into(),
            RootIdentifier::DatasetType.into(),
            RootIdentifier::VisualType.into(),
            RootIdentifier::CloneTrait.into(),
            RootIdentifier::CopyTrait.into(),
            RootIdentifier::PartialEqTrait.into(),
            RootIdentifier::EqTrait.into(),
            RootIdentifier::TypeType.into(),
            RootIdentifier::TraitType.into(),
            RootIdentifier::ModuleType.into(),
            RootIdentifier::Ref.into(),
            RootIdentifier::Option.into(),
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
        .into(),
    )
}

pub trait InternWord {
    fn word_allocator(&self) -> &WordInterner;
    fn intern_word(&self, word: &str) -> WordPtr {
        self.word_allocator().intern_borrowed(word)
    }
    fn custom_ident(&self, word: &str) -> CustomIdentifier {
        self.intern_word(word).opt_custom().unwrap()
    }
}

pub fn word_interner() -> &'static WordInterner {
    __access_singleton()
}

pub fn intern_word(text: &str) -> WordPtr {
    word_interner().intern_borrowed(text)
}
