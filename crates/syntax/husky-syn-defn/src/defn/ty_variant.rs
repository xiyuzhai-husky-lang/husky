mod props;
mod tuple;
mod unit;

pub use self::props::*;
pub use self::tuple::*;
pub use self::unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TypeVariantSynNodeDefn {
    Unit(UnitVariantSynNodeDefn),
    Tuple(TupleVariantSynNodeDefn),
    Props(PropsVariantSynNodeDefn),
}

impl TypeVariantSynNodeDefn {
    pub fn syn_node_decl(self, _db: &dyn SynDefnDb) -> TypeVariantSynNodeDecl {
        todo!()
    }
}

impl HasSynNodeDefn for TypeVariantSynNodePath {
    type SynNodeDefn = TypeVariantSynNodeDefn;

    fn syn_node_defn(self, _db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TypeVariantSynDefn {
    Unit(UnitVariantSynDefn),
    Tuple(TupleVariantSynDefn),
    Props(PropsVariantSynDefn),
}

impl TypeVariantSynDefn {
    pub fn decl(self, _db: &dyn SynDefnDb) -> SynDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn SynDefnDb) -> TypeVariantPath {
        todo!()
    }
}
