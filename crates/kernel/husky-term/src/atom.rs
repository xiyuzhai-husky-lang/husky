mod entity;
mod literal;
mod variable;

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
}

impl std::fmt::Display for TermAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TermAtom {
    pub fn ty_term(&self) -> TermCow {
        todo!()
    }

    pub(crate) fn new_literal(data: TermLiteralData, ty: Ty) -> Self {
        Self {
            variant: TermAtomVariant::Literal(data),
            ty: Some(ty),
        }
    }
}
