mod props;
mod tuple;
mod unit;

pub use self::props::*;
pub use self::tuple::*;
pub use self::unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantSynNodeDefn {
    Unit(UnitVariantSynNodeDefn),
    Tuple(TupleVariantSynNodeDefn),
    Props(PropsVariantSynNodeDefn),
}

impl TypeVariantSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> TypeVariantSynNodeDecl {
        todo!()
    }
}

impl HasSynNodeDefn for TypeVariantSynNodePath {
    type NodeDefn = TypeVariantSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantDefn {
    Unit(UnitVariantSynDefn),
    Tuple(TupleVariantSynDefn),
    Props(PropsVariantSynDefn),
}

impl TypeVariantDefn {
    pub fn decl(self, _db: &dyn SynDefnDb) -> Decl {
        todo!()
    }

    pub fn path(self, _db: &dyn SynDefnDb) -> TypeVariantPath {
        todo!()
    }
}
