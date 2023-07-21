mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDefn {
    TypeItem(TypeItemSynNodeDefn),
    TraitItem(TraitItemSynNodeDefn),
    TraitForTypeItem(TraitForTypeItemSynNodeDefn),
}

impl AssociatedItemSynNodeDefn {
    pub fn node_path(self, _db: &dyn SynDefnDb) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNodeDefn::TypeItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn node_decl(self, db: &dyn SynDefnDb) -> AssociatedItemSynNodeDecl {
        match self {
            AssociatedItemSynNodeDefn::TypeItem(node_defn) => node_defn.node_decl(db).into(),
            AssociatedItemSynNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitForTypeItem(node_defn) => {
                node_defn.node_decl(db).into()
            }
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            AssociatedItemSynNodeDefn::TypeItem(node_defn) => node_defn.expr_region(db),
            AssociatedItemSynNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitForTypeItem(node_defn) => {
                Some(node_defn.expr_region(db))
            }
        }
    }
}

impl HasSynNodeDefn for AssociatedItemSynNodePath {
    type NodeDefn = AssociatedItemSynNodeDefn;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        match self {
            AssociatedItemSynNodePath::TypeItem(node_path) => node_path.node_defn(db).into(),
            AssociatedItemSynNodePath::TraitItem(node_path) => node_path.node_defn(db).into(),
            AssociatedItemSynNodePath::TraitForTypeItem(node_path) => {
                node_path.node_defn(db).into()
            }
            AssociatedItemSynNodePath::IllFormedItem(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemSynDefn {
    TypeItem(TypeItemSynDefn),
    TraitItem(TraitItemSynDefn),
    TraitForTypeItem(TraitForTypeItemSynDefn),
}

impl AssociatedItemSynDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> AssociatedItemSynDecl {
        match self {
            AssociatedItemSynDefn::TypeItem(defn) => defn.decl(db).into(),
            AssociatedItemSynDefn::TraitItem(_) => todo!(),
            AssociatedItemSynDefn::TraitForTypeItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            AssociatedItemSynDefn::TypeItem(defn) => defn.expr_region(db),
            AssociatedItemSynDefn::TraitItem(_) => todo!(),
            AssociatedItemSynDefn::TraitForTypeItem(defn) => Some(defn.expr_region(db)),
        }
    }

    pub fn path(self, _db: &dyn SynDefnDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemSynDefn::TypeItem(_) => todo!(),
            AssociatedItemSynDefn::TraitItem(_) => todo!(),
            AssociatedItemSynDefn::TraitForTypeItem(_) => todo!(),
        }
    }
}

impl HasDefn for AssociatedItemPath {
    type Defn = AssociatedItemSynDefn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        Ok(match self {
            AssociatedItemPath::TypeItem(decl) => decl.defn(db)?.into(),
            AssociatedItemPath::TraitItem(decl) => decl.defn(db)?.into(),
            AssociatedItemPath::TraitForTypeItem(decl) => decl.defn(db)?.into(),
        })
    }
}
