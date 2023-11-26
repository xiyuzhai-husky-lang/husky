mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDataDefn {
    TypeItem(TypeItemSynNodeDefn),
    TraitItem(TraitItemSynNodeDefn),
    TraitForTypeItem(TraitForTypeItemSynNodeDefn),
}

impl AssociatedItemSynNodeDataDefn {
    pub fn syn_node_path(self, _db: &::salsa::Db) -> AssociatedItemSynNodeDataPath {
        match self {
            AssociatedItemSynNodeDataDefn::TypeItem(_) => todo!(),
            AssociatedItemSynNodeDataDefn::TraitItem(_) => todo!(),
            AssociatedItemSynNodeDataDefn::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn syn_node_decl(self, db: &::salsa::Db) -> AssociatedItemSynNodeDataDecl {
        match self {
            AssociatedItemSynNodeDataDefn::TypeItem(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            AssociatedItemSynNodeDataDefn::TraitItem(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            AssociatedItemSynNodeDataDefn::TraitForTypeItem(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &::salsa::Db,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            AssociatedItemSynNodeDataDefn::TypeItem(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            AssociatedItemSynNodeDataDefn::TraitItem(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            AssociatedItemSynNodeDataDefn::TraitForTypeItem(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
        }
    }
}

impl HasSynNodeDefn for AssociatedItemSynNodeDataPath {
    type SynNodeDefn = AssociatedItemSynNodeDataDefn;

    fn syn_node_defn(self, db: &::salsa::Db) -> Self::SynNodeDefn {
        match self {
            AssociatedItemSynNodeDataPath::TypeItem(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
            AssociatedItemSynNodeDataPath::TraitItem(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
            AssociatedItemSynNodeDataPath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.syn_node_defn(db).into()
            }
            AssociatedItemSynNodeDataPath::IllFormedItem(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynDefn {
    TypeItem(TypeItemSynDefn),
    TraitItem(TraitItemSynDefn),
    TraitForTypeItem(TraitForTypeItemSynDefn),
}

impl AssociatedItemSynDefn {
    pub fn decl(self, db: &::salsa::Db) -> AssociatedItemSynDecl {
        match self {
            AssociatedItemSynDefn::TypeItem(defn) => defn.decl(db).into(),
            AssociatedItemSynDefn::TraitItem(_) => todo!(),
            AssociatedItemSynDefn::TraitForTypeItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &::salsa::Db,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            AssociatedItemSynDefn::TypeItem(defn) => defn.body_with_syn_expr_region(db),
            AssociatedItemSynDefn::TraitItem(defn) => defn.body_with_syn_expr_region(db),
            AssociatedItemSynDefn::TraitForTypeItem(defn) => defn.body_with_syn_expr_region(db),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> AssociatedItemPath {
        match self {
            AssociatedItemSynDefn::TypeItem(syn_defn) => syn_defn.path(db).into(),
            AssociatedItemSynDefn::TraitItem(syn_defn) => syn_defn.path(db).into(),
            AssociatedItemSynDefn::TraitForTypeItem(syn_defn) => syn_defn.path(db).into(),
        }
    }
}

impl HasSynDefn for AssociatedItemPath {
    type SynDefn = AssociatedItemSynDefn;

    fn syn_defn(self, db: &::salsa::Db) -> SynDefnResult<Self::SynDefn> {
        Ok(match self {
            AssociatedItemPath::TypeItem(decl) => decl.syn_defn(db)?.into(),
            AssociatedItemPath::TraitItem(decl) => decl.syn_defn(db)?.into(),
            AssociatedItemPath::TraitForTypeItem(decl) => decl.syn_defn(db)?.into(),
        })
    }
}
