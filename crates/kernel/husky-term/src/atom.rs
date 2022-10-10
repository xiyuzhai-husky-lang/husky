mod category;
mod entity;
mod literal;
mod variable;

pub use category::*;
pub use entity::*;
pub use literal::*;
pub use variable::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermAtom {
    variant: TermAtomVariant,
    ty: Option<Ty>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermAtomVariant {
    Literal(TermLiteralData),
    Variable {
        variable_variant: TermVariableVariant,
    },
    Entity {/* todo */},
    Category {
        category_kind: TermCategoryKind,
    },
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
            ty: Some(ty),
        }
    }
}
