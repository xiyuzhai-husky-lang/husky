use husky_print_utils::p;
use husky_word::RootBuiltinIdentifier::{self, *};
use std::ops::Deref;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ty(TermPtr);

impl std::ops::Deref for Ty {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Ty {
    pub(crate) fn new(term: TermPtr) -> TermResult<Self> {
        match *term.ty_term().deref() {
            Term::Atom(ref a) => match a.variant() {
                TermAtomVariant::Category {
                    category_kind: TermCategoryKind::Type,
                } => (),
                _ => return Err(TermError::TermIsNotTy),
            },
            _ => return Err(TermError::TermIsNotTy),
        }
        Ok(Self(term))
    }

    pub fn term(self) -> TermPtr {
        self.0
    }

    pub(crate) fn entity_ty_ty(db: &dyn TermQuery) -> Self {
        todo!()
        // Ty::new(db.it_term(TermUniverse::zeroth_ty_universe().into())).unwrap()
    }

    // pub fn universe_level(self) -> TermUniverseLevel {
    //     todo!()
    //     todo!()
    //     // match self.ty_term().deref() {
    //     //     Term::Universe(u) => u.level(),
    //     //     _ => unreachable!(),
    //     // }
    // }
    // void
    pub(crate) fn void(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, Void)
    }
    // i32
    pub(crate) fn i32(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, I32)
    }
    // i64
    pub(crate) fn i64(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, I64)
    }
    // f32: Ty,
    pub(crate) fn f32(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, F32)
    }
    // f64: Ty,
    pub(crate) fn f64(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, F64)
    }
    // b32: Ty,
    pub(crate) fn b32(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, B32)
    }
    // b64: Ty,
    pub(crate) fn b64(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, B64)
    }
    // bool: Ty,
    pub(crate) fn bool(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, Bool)
    }

    pub(crate) fn module(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, ModuleType)
    }

    pub(crate) fn trai(db: &dyn TermQuery) -> Ty {
        Self::root_builtin_ty(db, TraitType)
    }

    fn root_builtin_ty(db: &dyn TermQuery, ident: RootBuiltinIdentifier) -> Ty {
        Ty::new(TermEntity::root_builtin_entity(db, ident)).unwrap()
    }
}

impl Term {
    #[inline(always)]
    pub(crate) fn ty_term(&self) -> TermCow {
        match self {
            Term::Atom(a) => a.ty_term(),
            Term::Curry(c) => c.ty().term().into(),
            Term::Abstraction(abs) => abs.ty().term().into(),
            Term::Application(app) => app.ty().term().into(),
        }
    }

    // pub(crate) fn universe(&self) -> Universe {
    //     match self {
    //         Term::Type(u) => u.next().expect("todo"),
    //         Term::Curry(TermCurry { from, to }) => {
    //             from.universe().max(to.universe()).next().expect("todo")
    //         }
    //         Term::Variable(v) => v.universe(),
    //         Term::Abstraction(abs) => abs.universe(),
    //         Term::Application(app) => app.universe(),
    //     }
    // }
}
