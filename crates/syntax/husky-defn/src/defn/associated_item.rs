mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemNodeDefn {
    TypeItem(TypeItemNodeDefn),
    TraitItem(TraitItemNodeDefn),
    TraitForTypeItem(TraitForTypeItemNodeDefn),
}

impl AssociatedItemNodeDefn {
    pub fn node_path(self, _db: &dyn DefnDb) -> AssociatedItemNodePath {
        match self {
            AssociatedItemNodeDefn::TypeItem(_) => todo!(),
            AssociatedItemNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemNodeDefn::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn node_decl(self, db: &dyn DefnDb) -> AssociatedItemNodeDecl {
        match self {
            AssociatedItemNodeDefn::TypeItem(node_defn) => node_defn.node_decl(db).into(),
            AssociatedItemNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemNodeDefn::TraitForTypeItem(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            AssociatedItemNodeDefn::TypeItem(node_defn) => node_defn.expr_region(db),
            AssociatedItemNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemNodeDefn::TraitForTypeItem(node_defn) => Some(node_defn.expr_region(db)),
        }
    }
}

impl HasNodeDefn for AssociatedItemNodePath {
    type NodeDefn = AssociatedItemNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        match self {
            AssociatedItemNodePath::TypeItem(node_path) => node_path.node_defn(db).into(),
            AssociatedItemNodePath::TraitItem(node_path) => node_path.node_defn(db).into(),
            AssociatedItemNodePath::TraitForTypeItem(node_path) => node_path.node_defn(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemDefn {
    TypeItem(TypeItemDefn),
    TraitItem(TraitItemDefn),
    TraitForTypeItem(TraitForTypeItemDefn),
}

impl AssociatedItemDefn {
    pub fn decl(self, db: &dyn DefnDb) -> AssociatedItemDecl {
        match self {
            AssociatedItemDefn::TypeItem(defn) => defn.decl(db).into(),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TraitForTypeItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            AssociatedItemDefn::TypeItem(defn) => defn.expr_region(db),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TraitForTypeItem(defn) => Some(defn.expr_region(db)),
        }
    }

    pub fn path(self, _db: &dyn DefnDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemDefn::TypeItem(_) => todo!(),
            AssociatedItemDefn::TraitItem(_) => todo!(),
            AssociatedItemDefn::TraitForTypeItem(_) => todo!(),
        }
    }
}

impl HasDefn for AssociatedItemPath {
    type Defn = AssociatedItemDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        Ok(match self {
            AssociatedItemPath::TypeItem(decl) => decl.defn(db)?.into(),
            AssociatedItemPath::TraitItem(decl) => decl.defn(db)?.into(),
            AssociatedItemPath::TraitForTypeItem(decl) => decl.defn(db)?.into(),
        })
    }
}
