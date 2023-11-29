mod ill_formed_item;
mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::ill_formed_item::*;
pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;
use husky_coword::Ident;
use husky_entity_kind::TraitItemKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDecl {
    TypeItem(TypeItemSynNodeDecl),
    TraitItem(TraitItemSynNodeDecl),
    TraitForTypeItem(TraitForTypeItemSynNodeDecl),
    IllFormedItem(IllFormedItemSynNodeDecl),
}

impl AssociatedItemSynNodeDecl {
    pub fn syn_node_path(self, db: &::salsa::Db) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDecl::IllFormedItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.template_parameters(db)
            }
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.template_parameters(db)
            }
            AssociatedItemSynNodeDecl::TraitForTypeItem(_) => todo!(),
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }
}

impl HasSynNodeDecl for AssociatedItemSynNodePath {
    type NodeDecl = AssociatedItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        match self {
            AssociatedItemSynNodePath::TypeItem(path) => path.syn_node_decl(db).into(),
            AssociatedItemSynNodePath::TraitItem(path) => path.syn_node_decl(db).into(),
            AssociatedItemSynNodePath::TraitForTypeItem(path) => path.syn_node_decl(db).into(),
            AssociatedItemSynNodePath::IllFormedItem(path) => path.syn_node_decl(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum AssociatedItemSynDecl {
    TypeItem(TypeItemSynDecl),
    TraitItem(TraitItemSynDecl),
    TraitForTypeItem(TraitForTypeItemSynDecl),
}

impl AssociatedItemSynDecl {
    pub fn path(self, db: &::salsa::Db) -> AssociatedItemPath {
        match self {
            AssociatedItemSynDecl::TypeItem(decl) => decl.path(db).into(),
            AssociatedItemSynDecl::TraitItem(decl) => decl.path(db).into(),
            AssociatedItemSynDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            AssociatedItemSynDecl::TypeItem(decl) => decl.template_parameters(db),
            AssociatedItemSynDecl::TraitItem(decl) => decl.template_parameters(db),
            AssociatedItemSynDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateSynParameterData]> {
        match self {
            AssociatedItemSynDecl::TypeItem(syn_decl) => syn_decl.parenate_parameters(db),
            AssociatedItemSynDecl::TraitItem(syn_decl) => syn_decl.parenate_parameters(db),
            AssociatedItemSynDecl::TraitForTypeItem(syn_decl) => syn_decl.parenate_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AssociatedItemSynDecl::TypeItem(decl) => decl.syn_expr_region(db),
            AssociatedItemSynDecl::TraitItem(decl) => decl.syn_expr_region(db),
            AssociatedItemSynDecl::TraitForTypeItem(decl) => decl.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for AssociatedItemPath {
    type Decl = AssociatedItemSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        match self {
            AssociatedItemPath::TypeItem(slf) => slf.syn_decl(db).map(Into::into),
            AssociatedItemPath::TraitItem(slf) => slf.syn_decl(db).map(Into::into),
            AssociatedItemPath::TraitForTypeItem(slf) => slf.syn_decl(db).map(Into::into),
        }
    }
}

pub trait HasItemDeclsMap {
    type ItemDecls;

    fn item_syn_decls_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]>;
}

pub trait HasItemDecls {
    type ItemDecls;

    fn item_syn_decls<'a>(self, db: &'a ::salsa::Db) -> DeclResult<&'a Self::ItemDecls>;
}
