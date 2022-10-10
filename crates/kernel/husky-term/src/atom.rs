mod category;
mod entity;
mod literal;
mod universe;
mod variable;

use std::ops::Deref;

pub use category::*;
pub use entity::*;
use husky_entity_path::EntityPathPtr;
pub use literal::*;
pub use universe::*;
pub use variable::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermAtom {
    variant: TermAtomVariant,
    ty_itd: Option<Ty>,
}

impl Into<Term> for TermAtom {
    fn into(self) -> Term {
        Term::Atom(self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermAtomVariant {
    Literal(TermLiteralData),
    Variable {
        variable_variant: TermVariableVariant,
    },
    Entity {
        path: EntityPathPtr,
    },
    CategoryKind(TermCategoryKind),
    Universe(TermUniverse),
}

impl std::fmt::Display for TermAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            TermAtomVariant::Literal(ref l) => l.fmt(f),
            TermAtomVariant::Variable {
                ref variable_variant,
            } => variable_variant.fmt(f),
            TermAtomVariant::Entity { path } => path.fmt(f),
            TermAtomVariant::CategoryKind(category_kind) => todo!(),
            TermAtomVariant::Universe(_) => todo!(),
        }
    }
}

impl TermAtom {
    pub(crate) fn ty_term(&self) -> TermCow {
        todo!()
    }

    pub(crate) fn variant(&self) -> &TermAtomVariant {
        &self.variant
    }

    pub fn universe(&self) -> TermUniverse {
        if let Some(ty_itd) = self.ty_itd {
            match ty_itd.term().deref() {
                Term::Application(app) => match app.m().deref() {
                    Term::Atom(m) => match m.variant() {
                        TermAtomVariant::Literal(_) => todo!(),
                        TermAtomVariant::Variable { variable_variant } => todo!(),
                        TermAtomVariant::Entity { path } => todo!(),
                        TermAtomVariant::CategoryKind(category_kind) => match category_kind {
                            TermCategoryKind::Type => todo!(),
                            TermCategoryKind::Sort => return app.n().as_universe().unwrap(),
                            TermCategoryKind::Term => todo!(),
                        },
                        TermAtomVariant::Universe(_) => todo!(),
                    },
                    Term::Curry(_) => todo!(),
                    Term::Abstraction(_) => todo!(),
                    Term::Application(_) => todo!(),
                },
                _ => (),
            }
            TermUniverse::zero()
        } else {
            match self.variant {
                TermAtomVariant::Literal(_) => todo!(),
                TermAtomVariant::Variable {
                    ref variable_variant,
                } => todo!(),
                TermAtomVariant::Entity { path } => todo!(),
                TermAtomVariant::CategoryKind(category_kind) => todo!(),
                TermAtomVariant::Universe(_) => todo!(),
            }
        }
    }

    pub(crate) fn new_literal(data: TermLiteralData, ty: Ty) -> Self {
        Self {
            variant: TermAtomVariant::Literal(data),
            ty_itd: Some(ty),
        }
    }

    pub(crate) fn new_universe(i: u8) -> Self {
        Self {
            variant: TermAtomVariant::Universe(TermUniverse::new(i)),
            ty_itd: None,
        }
    }

    pub(crate) fn new_category(category_kind: TermCategoryKind) -> Self {
        Self {
            variant: TermAtomVariant::CategoryKind(category_kind),
            ty_itd: None,
        }
    }

    pub(crate) fn ty_itd(&self) -> Option<Ty> {
        self.ty_itd
    }
}
