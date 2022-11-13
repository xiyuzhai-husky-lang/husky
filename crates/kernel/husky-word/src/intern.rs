use crate::{ident::ContextualIdentifier, *};
use interner::{Internable, InternedRefWrapper, Interner};
use std::{borrow::Borrow, ops::Deref};

pub type WordInterner = Interner<Word>;

impl Deref for WordItd {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            WordItd::Keyword(keyword) => keyword.deref(),
            WordItd::Identifier(ident) => ident.deref(),
            WordItd::Opr(opr) => opr.deref(),
            WordItd::Decorator(decorator) => decorator.deref(),
            WordItd::Pattern(patt) => patt.deref(),
        }
    }
}

impl Borrow<str> for WordItd {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl Internable for Word {
    type Ref<'a> = WordRef<'a>;

    type Interned = WordItd;

    fn new_itr() -> Interner<Self> {
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

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Ref<'static> {
        WordRef(itd.as_str())
    }

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        WordRef(&self.0)
    }

    fn new_itd(&'static self, _id: usize) -> Self::Interned {
        WordItd::Identifier(Identifier::Custom(CustomIdentifier(
            InternedRefWrapper::new(&self.0),
        )))
    }

    fn try_direct_from_ref<'a>(_r: Self::Ref<'a>) -> Option<Self::Interned> {
        None
    }

    unsafe fn cast_to_static_ref<'a>(r: Self::Ref<'a>) -> Self::Ref<'static> {
        WordRef(&*(r.0 as *const _))
    }
}

// Itd {
//     type Ref = str;

//     type Owned = String;

//     fn new_interned(id: usize, target: &'static Self::Ref) -> Self {
//         Self::Identifier(Identifier::Custom(CustomIdentifier(target)))
//     }

//     fn new_itr() -> Interner<Self> {
//         new_word_itr()
//     }

//     fn opt_atom_itd(t: &Self::Ref) -> Option<Self> {
//         None
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WordRef<'a>(&'a str);

impl<'a> From<WordRef<'a>> for Word {
    fn from(value: WordRef) -> Self {
        Word(value.0.into())
    }
}

// impl From<WordItd> for WordBorrowed<'static> {
//     fn from(value: WordItd) -> Self {
//         todo!()
//     }
// }

pub trait InternWord {
    fn word_itr(&self) -> &WordInterner;
    fn it_word(&self, word: &str) -> WordItd {
        assert!(is_valid_word(word));
        self.word_itr().intern_borrowed(WordRef(word))
    }
    fn it_ident(&self, word: &str) -> Identifier {
        assert!(is_valid_word(word));
        self.word_itr().intern_borrowed(WordRef(word)).ident()
    }
    fn custom_ident(&self, word: &str) -> CustomIdentifier {
        self.it_word(word).opt_custom().unwrap()
    }
}

#[test]
fn test_is_valid_ident() {
    assert_eq!(is_valid_word("a"), true);
    assert_eq!(is_valid_word("b"), true);
    assert_eq!(is_valid_word("c"), true);
    assert_eq!(is_valid_word("d"), true);
    assert_eq!(is_valid_word("e"), true);
    assert_eq!(is_valid_word("g"), true);
    assert_eq!(is_valid_word("h"), true);
    assert_eq!(is_valid_word("i"), true);
    assert_eq!(is_valid_word("j"), true);
    assert_eq!(is_valid_word("a1"), true);
    assert_eq!(is_valid_word("a2"), true);
    assert_eq!(is_valid_word("a3"), true);
    assert_eq!(is_valid_word("_a1"), true);
    assert_eq!(is_valid_word("_1"), true);
    assert_eq!(is_valid_word("_"), true);
    assert_eq!(is_valid_word("9"), false);
    assert_eq!(is_valid_word("9a"), false);
    assert_eq!(is_valid_word(" "), false);
    assert_eq!(is_valid_word("*"), false);
    assert_eq!(is_valid_word("&"), false);
    assert_eq!(is_valid_word(":"), false);
    assert_eq!(is_valid_word("{"), false);
    assert_eq!(is_valid_word("}"), false);
}

impl InternWord for WordInterner {
    fn word_itr(&self) -> &WordInterner {
        self
    }
}
