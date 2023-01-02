mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VariantDecl {
    Props(PropsVariantDecl),
    Unit(UnitVariantDecl),
    Tuple(TupleVariantDecl),
}
impl VariantDecl {
    pub(crate) fn ast_idx(&self, db: &dyn DeclDb) -> AstIdx {
        todo!()
    }
}
