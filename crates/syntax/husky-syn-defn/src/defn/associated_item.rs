mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDefn {
    TypeItem(TypeItemSynNodeDefn),
    TraitItem(TraitItemSynNodeDefn),
    TraitForTypeItem(TraitForTypeItemSynNodeDefn),
}

impl AssociatedItemSynNodeDefn {
    pub fn syn_node_path(self, _db: &dyn SynDefnDb) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNodeDefn::TypeItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> AssociatedItemSynNodeDecl {
        match self {
            AssociatedItemSynNodeDefn::TypeItem(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            AssociatedItemSynNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitForTypeItem(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            AssociatedItemSynNodeDefn::TypeItem(syn_node_defn) => syn_node_defn.syn_expr_region(db),
            AssociatedItemSynNodeDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDefn::TraitForTypeItem(syn_node_defn) => {
                Some(syn_node_defn.syn_expr_region(db))
            }
        }
    }
}

impl HasSynNodeDefn for AssociatedItemSynNodePath {
    type SynNodeDefn = AssociatedItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
            AssociatedItemSynNodePath::IllFormedItem(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb)]
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

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            AssociatedItemSynDefn::TypeItem(defn) => defn.syn_expr_region(db),
            AssociatedItemSynDefn::TraitItem(_) => todo!(),
            AssociatedItemSynDefn::TraitForTypeItem(defn) => Some(defn.syn_expr_region(db)),
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

impl HasSynDefn for AssociatedItemPath {
    type SynDefn = AssociatedItemSynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(match self {
            AssociatedItemPath::TypeItem(decl) => decl.syn_defn(db)?.into(),
            AssociatedItemPath::TraitItem(decl) => decl.syn_defn(db)?.into(),
            AssociatedItemPath::TraitForTypeItem(decl) => decl.syn_defn(db)?.into(),
        })
    }
}
