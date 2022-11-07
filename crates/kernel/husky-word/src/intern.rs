use crate::{ident::ContextualIdentifier, *};
use interner::{Interned, Interner};
use std::{borrow::Borrow, ops::Deref};

pub type WordInterner = Interner<Word>;

impl Deref for Word {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Word::Keyword(keyword) => keyword.deref(),
            Word::Identifier(ident) => ident.deref(),
            Word::Opr(opr) => opr.deref(),
            Word::Decorator(decorator) => decorator.deref(),
            Word::Pattern(patt) => patt.deref(),
        }
    }
}

impl Borrow<str> for Word {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl Interned for Word {
    type T = str;

    type Owned = String;

    fn new_intern_ptr(id: usize, target: &'static Self::T) -> Self {
        Self::Identifier(Identifier::Custom(CustomIdentifier(target)))
    }

    fn new_itr() -> Interner<Self> {
        new_word_itr()
    }
}

fn new_word_itr() -> WordInterner {
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
        RootBuiltinIdentifier::Trait.into(),
        RootBuiltinIdentifier::Module.into(),
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
    fn word_itr(&self) -> &WordInterner;
    fn it_word(&self, word: &str) -> Word {
        self.word_itr().intern_borrowed(word)
    }
    fn it_ident(&self, word: &str) -> Identifier {
        assert!(is_valid_ident(word));
        self.word_itr().intern_borrowed(word).ident()
    }
    fn custom_ident(&self, word: &str) -> CustomIdentifier {
        self.it_word(word).opt_custom().unwrap()
    }
}

fn is_valid_ident(word: &str) -> bool {
    let mut chars = word.chars();
    if let Some(start) = chars.next() {
        if !(start.is_alphabetic() || start == '_') {
            return false;
        }
    }
    for c in chars {
        if !(c.is_alphanumeric() || c == '_') {
            return false;
        }
    }
    true
}

#[test]
fn test_is_valid_ident() {
    assert_eq!(is_valid_ident("a"), true);
    assert_eq!(is_valid_ident("b"), true);
    assert_eq!(is_valid_ident("c"), true);
    assert_eq!(is_valid_ident("d"), true);
    assert_eq!(is_valid_ident("e"), true);
    assert_eq!(is_valid_ident("g"), true);
    assert_eq!(is_valid_ident("h"), true);
    assert_eq!(is_valid_ident("i"), true);
    assert_eq!(is_valid_ident("j"), true);
    assert_eq!(is_valid_ident("a1"), true);
    assert_eq!(is_valid_ident("a2"), true);
    assert_eq!(is_valid_ident("a3"), true);
    assert_eq!(is_valid_ident("_a1"), true);
    assert_eq!(is_valid_ident("_1"), true);
    assert_eq!(is_valid_ident("_"), true);
    assert_eq!(is_valid_ident("9"), false);
    assert_eq!(is_valid_ident("9a"), false);
    assert_eq!(is_valid_ident(" "), false);
    assert_eq!(is_valid_ident("*"), false);
    assert_eq!(is_valid_ident("&"), false);
    assert_eq!(is_valid_ident(":"), false);
    assert_eq!(is_valid_ident("{"), false);
    assert_eq!(is_valid_ident("}"), false);
}

impl InternWord for WordInterner {
    fn word_itr(&self) -> &WordInterner {
        self
    }
}
