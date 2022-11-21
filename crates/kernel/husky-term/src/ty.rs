use husky_identifier::*;
use salsa::AsId;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ty(Term);

impl AsId for Ty {
    fn as_id(self) -> salsa::Id {
        todo!()
    }

    fn from_id(id: salsa::Id) -> Self {
        todo!()
    }
}

impl Ty {
    pub(crate) fn new(term: Term) -> TermResult<Self> {
        // TODO: type checking
        Ok(Ty(term))
        // if let Some(ty_itd) = term.ty_itd() {
        //     Self::check_is_category(ty_itd.term())?
        // } else {
        //     Self::check_is_category(term)?
        // }
        // Ok(Self(term))
    }

    fn check_is_category(_term: Term) -> TermResult<()> {
        todo!()
        // match term.deref() {
        //     TermData::Application(app) => match app.m().deref() {
        //         TermData::Atom(a) => match a {
        //             TermAtom::Literal(_) => todo!(),
        //             TermAtom::Variable { variable_variant } => todo!(),
        //             TermAtom::Entity { .. } => todo!(),
        //             TermAtom::Category(category_kind) => match category_kind {
        //                 TermCategory::Type => todo!(),
        //                 TermCategory::Sort => Ok(()),
        //                 TermCategory::TermData => todo!(),
        //             },
        //             TermAtom::Universe(_) => todo!(),
        //         },
        //         TermData::Curry(_) => todo!(),
        //         TermData::Abstraction(_) => todo!(),
        //         TermData::Application(_) => todo!(),
        //         TermData::Subentity(_) => todo!(),
        //         TermData::TraitImpl(_) => todo!(),
        //     },
        //     _ => Err(todo!()),
        // }
    }

    fn check_ty_itd(ty: Ty) -> TermResult<()> {
        todo!()
        // match ty.term().borrowed() {
        //     TermRef::Atom(_a) => todo!(),
        //     _ => return Err(TermError::TermIsNotTy),
        // }
    }

    pub fn term(self) -> Term {
        self.0
    }

    // pub fn universe_level(self) -> TermUniverseLevel {
    //     todo!()
    //     todo!()
    //     // match self.ty_term().deref() {
    //     //     TermData::Universe(u) => u.level(),
    //     //     _ => unreachable!(),
    //     // }
    // }
    // void
    pub(crate) fn void(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, Void, menu1)
    }
    // i32
    pub(crate) fn i32(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, I32, menu1)
    }

    pub(crate) fn i64(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, I64, menu1)
    }

    pub(crate) fn f32(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, F32, menu1)
    }

    pub(crate) fn f64(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, F64, menu1)
    }

    pub(crate) fn b32(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, B32, menu1)
    }

    pub(crate) fn b64(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, B64, menu1)
    }

    pub(crate) fn bool(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, Bool, menu1)
    }

    pub(crate) fn module(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, Module, menu1)
    }

    pub(crate) fn trai(db: &dyn TermDb, menu1: &TermMenu1) -> Ty {
        todo!()
        // Self::root_builtin_ty(db, Trait, menu1)
    }

    fn root_builtin_ty(db: &dyn TermDb, ident: Identifier, menu1: &TermMenu1) -> Ty {
        todo!()
        // Ty::new(TermData::root_builtin_entity(db, ident, menu1.ty0())).unwrap()
    }
}

impl TermData {
    // #[inline(always)]
    // pub(crate) fn ty_term(&self) -> TermCow {
    //     match self {
    //         TermData::Atom(a) => a.ty_term(),
    //         TermData::Curry(c) => c.ty().term().into(),
    //         TermData::Abstraction(abs) => abs.ty().term().into(),
    //         TermData::Application(app) => app.ty_term(),
    //     }
    // }

    // pub(crate) fn universe(&self) -> Universe {
    //     match self {
    //         TermData::Type(u) => u.next().expect("todo"),
    //         TermData::Curry(TermCurry { from, to }) => {
    //             from.universe().max(to.universe()).next().expect("todo")
    //         }
    //         TermData::Variable(v) => v.universe(),
    //         TermData::Abstraction(abs) => abs.universe(),
    //         TermData::Application(app) => app.universe(),
    //     }
    // }
}
