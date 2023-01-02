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

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for VariantDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
