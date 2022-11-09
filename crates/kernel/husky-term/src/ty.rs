use husky_print_utils::p;
use husky_word::RootBuiltinIdentifier::{self, *};
use std::ops::Deref;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ty(TermItd);

impl Ty {
    pub(crate) fn new(term: TermItd) -> TermResult<Self> {
        todo!()
        // if let Some(ty_itd) = term.ty_itd() {
        //     Self::check_is_category(ty_itd.term())?
        // } else {
        //     Self::check_is_category(term)?
        // }
        // Ok(Self(term))
    }

    fn check_is_category(term: TermItd) -> TermResult<()> {
        todo!()
        // match term.deref() {
        //     Term::Application(app) => match app.m().deref() {
        //         Term::Atom(a) => match a {
        //             TermAtom::Literal(_) => todo!(),
        //             TermAtom::Variable { variable_variant } => todo!(),
        //             TermAtom::Entity { .. } => todo!(),
        //             TermAtom::Category(category_kind) => match category_kind {
        //                 TermCategory::Type => todo!(),
        //                 TermCategory::Sort => Ok(()),
        //                 TermCategory::Term => todo!(),
        //             },
        //             TermAtom::Universe(_) => todo!(),
        //         },
        //         Term::Curry(_) => todo!(),
        //         Term::Abstraction(_) => todo!(),
        //         Term::Application(_) => todo!(),
        //         Term::Subentity(_) => todo!(),
        //         Term::TraitImpl(_) => todo!(),
        //     },
        //     _ => Err(todo!()),
        // }
    }

    fn check_ty_itd(ty: Ty) -> TermResult<()> {
        match ty.term() {
            TermItd::Atom(a) => todo!(),
            _ => return Err(TermError::TermIsNotTy),
        }
    }

    pub fn term(self) -> TermItd {
        self.0
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
    pub(crate) fn void(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, Void, menu1)
    }
    // i32
    pub(crate) fn i32(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, I32, menu1)
    }

    pub(crate) fn i64(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, I64, menu1)
    }

    pub(crate) fn f32(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, F32, menu1)
    }

    pub(crate) fn f64(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, F64, menu1)
    }

    pub(crate) fn b32(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, B32, menu1)
    }

    pub(crate) fn b64(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, B64, menu1)
    }

    pub(crate) fn bool(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, Bool, menu1)
    }

    pub(crate) fn module(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, Module, menu1)
    }

    pub(crate) fn trai(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        Self::root_builtin_ty(db, Trait, menu1)
    }

    fn root_builtin_ty(db: &dyn TermDb, ident: RootBuiltinIdentifier, menu1: &TermMenu1) -> Ty {
        Ty::new(TermOwned::root_builtin_entity(db, ident, menu1.ty0())).unwrap()
    }
}

impl TermOwned {
    // #[inline(always)]
    // pub(crate) fn ty_term(&self) -> TermCow {
    //     match self {
    //         Term::Atom(a) => a.ty_term(),
    //         Term::Curry(c) => c.ty().term().into(),
    //         Term::Abstraction(abs) => abs.ty().term().into(),
    //         Term::Application(app) => app.ty_term(),
    //     }
    // }

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

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
