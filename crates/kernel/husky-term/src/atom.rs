mod category;
mod entity;
mod literal;
mod universe;
mod variable;

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
    Category {
        category_kind: TermCategoryKind,
    },
    Universe(TermUniverse),
}

impl std::fmt::Display for TermAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TermAtom {
    pub(crate) fn ty_term(&self) -> TermCow {
        todo!()
    }

    pub(crate) fn variant(&self) -> &TermAtomVariant {
        &self.variant
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
            variant: TermAtomVariant::Category { category_kind },
            ty_itd: None,
        }
    }

    pub(crate) fn ty_itd(&self) -> Option<Ty> {
        self.ty_itd
    }
}
