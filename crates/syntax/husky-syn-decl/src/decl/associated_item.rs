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
pub enum AssociatedItemSynNodeDataDecl {
    TypeItem(TypeItemSynNodeDecl),
    TraitItem(TraitItemSynNodeDecl),
    TraitForTypeItem(TraitForTypeItemSynNodeDecl),
    IllFormedItem(IllFormedItemSynNodeDecl),
}

impl AssociatedItemSynNodeDataDecl {
    pub fn syn_node_path(self, db: &::salsa::Db) -> AssociatedItemSynNodeDataPath {
        match self {
            AssociatedItemSynNodeDataDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDataDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDataDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDataDecl::IllFormedItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            AssociatedItemSynNodeDataDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.template_parameters(db)
            }
            AssociatedItemSynNodeDataDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.template_parameters(db)
            }
            AssociatedItemSynNodeDataDecl::TraitForTypeItem(_) => todo!(),
            AssociatedItemSynNodeDataDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AssociatedItemSynNodeDataDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            AssociatedItemSynNodeDataDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            AssociatedItemSynNodeDataDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            AssociatedItemSynNodeDataDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            AssociatedItemSynNodeDataDecl::TypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDataDecl::TraitItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDataDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.errors(db)
            }
            AssociatedItemSynNodeDataDecl::IllFormedItem(_) => todo!(),
        }
    }
}

impl HasSynNodeDecl for AssociatedItemSynNodeDataPath {
    type NodeDecl = AssociatedItemSynNodeDataDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        match self {
            AssociatedItemSynNodeDataPath::TypeItem(path) => path.syn_node_decl(db).into(),
            AssociatedItemSynNodeDataPath::TraitItem(path) => path.syn_node_decl(db).into(),
            AssociatedItemSynNodeDataPath::TraitForTypeItem(path) => path.syn_node_decl(db).into(),
            AssociatedItemSynNodeDataPath::IllFormedItem(path) => path.syn_node_decl(db).into(),
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

    fn syn_decl(self, _db: &::salsa::Db) -> DeclResult<Self::Decl> {
        todo!()
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
