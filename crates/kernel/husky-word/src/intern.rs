use crate::{ident::ContextualIdentifier, *};
use interner::{Interner, IsInternPtr};
use std::{borrow::Borrow, ops::Deref};

pub type WordInterner = Interner<WordPtr>;

impl Deref for WordPtr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            WordPtr::Keyword(keyword) => keyword.deref(),
            WordPtr::Identifier(ident) => ident.deref(),
            WordPtr::Opr(opr) => opr.deref(),
            WordPtr::Decorator(decorator) => decorator.deref(),
            WordPtr::Pattern(patt) => patt.deref(),
        }
    }
}

impl Borrow<str> for WordPtr {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl IsInternPtr for WordPtr {
    type T = str;

    type Owned = String;

    fn new_intern_ptr(id: usize, target: &'static Self::T) -> Self {
        Self::Identifier(Identifier::Custom(CustomIdentifier(target)))
    }
}

pub fn new_word_itr() -> WordInterner {
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
        StmtKeyword::Require.into(),
        RootBuiltinIdentifier::Debug.into(),
        RootBuiltinIdentifier::Std.into(),
        RootBuiltinIdentifier::Core.into(),
        RootBuiltinIdentifier::Debug.into(),
        RootBuiltinIdentifier::Void.into(),
        RootBuiltinIdentifier::I32.into(),
        RootBuiltinIdentifier::I64.into(),
        RootBuiltinIdentifier::F32.into(),
        RootBuiltinIdentifier::F64.into(),
        RootBuiltinIdentifier::B32.into(),
        RootBuiltinIdentifier::B64.into(),
        RootBuiltinIdentifier::Bool.into(),
        RootBuiltinIdentifier::True.into(),
        RootBuiltinIdentifier::False.into(),
        RootBuiltinIdentifier::Vec.into(),
        RootBuiltinIdentifier::Array.into(),
        RootBuiltinIdentifier::Tuple.into(),
        RootBuiltinIdentifier::Mor.into(),
        RootBuiltinIdentifier::ThickFp.into(),
        RootBuiltinIdentifier::Fn.into(),
        RootBuiltinIdentifier::FnMut.into(),
        RootBuiltinIdentifier::FnOnce.into(),
        RootBuiltinIdentifier::Domains.into(),
        RootBuiltinIdentifier::DatasetType.into(),
        RootBuiltinIdentifier::VisualType.into(),
        RootBuiltinIdentifier::CloneTrait.into(),
        RootBuiltinIdentifier::CopyTrait.into(),
        RootBuiltinIdentifier::PartialEqTrait.into(),
        RootBuiltinIdentifier::EqTrait.into(),
        RootBuiltinIdentifier::TypeType.into(),
        RootBuiltinIdentifier::TraitType.into(),
        RootBuiltinIdentifier::ModuleType.into(),
        RootBuiltinIdentifier::Ref.into(),
        RootBuiltinIdentifier::RefMut.into(),
        RootBuiltinIdentifier::Option.into(),
        ContextualIdentifier::Crate.into(),
        ContextualIdentifier::CrateInputValue.into(),
        ContextualIdentifier::TargetOutputType.into(),
        ContextualIdentifier::ThisValue.into(),
        ContextualIdentifier::ThisType.into(),
        WordOpr::And.into(),
        WordOpr::Or.into(),
        WordOpr::As.into(),
        WordOpr::Be.into(),
        Decorator::Pub.into(),
        Decorator::Private.into(),
        Decorator::Async.into(),
        Decorator::Static.into(),
        WordPattern::Some.into(),
        WordPattern::None.into(),
    ])
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

impl InternWord for WordInterner {
    fn word_allocator(&self) -> &WordInterner {
        self
    }
}
