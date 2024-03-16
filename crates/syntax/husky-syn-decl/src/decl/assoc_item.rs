mod ill_formed_item;
mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::ill_formed_item::*;
pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemSynNodeDecl {
    TypeItem(TypeItemSynNodeDecl),
    TraitItem(TraitItemSynNodeDecl),
    TraitForTypeItem(TraitForTypeItemSynNodeDecl),
    IllFormedItem(IllFormedItemSynNodeDecl),
}

impl AssocItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AssocItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            AssocItemSynNodeDecl::TraitItem(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            AssocItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            AssocItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            AssocItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssocItemSynNodeDecl::TraitItem(syn_node_decl) => syn_node_decl.errors(db),
            AssocItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssocItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }
}

impl HasSynNodeDecl for AssocItemSynNodePath {
    type NodeDecl = AssocItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        match self {
            AssocItemSynNodePath::TypeItem(path) => path.syn_node_decl(db).into(),
            AssocItemSynNodePath::TraitItem(path) => path.syn_node_decl(db).into(),
            AssocItemSynNodePath::TraitForTypeItem(path) => path.syn_node_decl(db).into(),
            AssocItemSynNodePath::IllFormedItem(path) => path.syn_node_decl(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemSynDecl {
    TypeItem(TypeItemSynDecl),
    TraitItem(TraitItemSynDecl),
    TraitForTypeItem(TraitForTypeItemSynDecl),
}

impl AssocItemSynDecl {
    pub fn path(self, db: &::salsa::Db) -> AssocItemPath {
        match self {
            AssocItemSynDecl::TypeItem(decl) => decl.path(db).into(),
            AssocItemSynDecl::TraitItem(decl) => decl.path(db).into(),
            AssocItemSynDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            AssocItemSynDecl::TypeItem(decl) => decl.template_parameters(db),
            AssocItemSynDecl::TraitItem(decl) => decl.template_parameters(db),
            AssocItemSynDecl::TraitForTypeItem(decl) => decl.template_parameters(db),
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateParameterSyndicate]> {
        match self {
            AssocItemSynDecl::TypeItem(syn_decl) => syn_decl.parenate_parameters(db),
            AssocItemSynDecl::TraitItem(syn_decl) => syn_decl.parenate_parameters(db),
            AssocItemSynDecl::TraitForTypeItem(syn_decl) => syn_decl.parenate_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AssocItemSynDecl::TypeItem(decl) => decl.syn_expr_region(db),
            AssocItemSynDecl::TraitItem(decl) => decl.syn_expr_region(db),
            AssocItemSynDecl::TraitForTypeItem(decl) => decl.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for AssocItemPath {
    type Decl = AssocItemSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        match self {
            AssocItemPath::TypeItem(slf) => slf.syn_decl(db).map(Into::into),
            AssocItemPath::TraitItem(slf) => slf.syn_decl(db).map(Into::into),
            AssocItemPath::TraitForTypeItem(slf) => slf.syn_decl(db).map(Into::into),
        }
    }
}

pub trait HasItemDecls {
    type ItemDecls;

    fn item_syn_decls<'a>(self, db: &'a ::salsa::Db) -> DeclResult<&'a Self::ItemDecls>;
}
