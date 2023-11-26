mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_regional_token::*;
use husky_token_data::{TokenData, TokenDataResult};

type SmallVecImpl<T> = smallvec::SmallVec<[T; 2]>;

/// A `NodeDecl` is a tolerant information-preserving declaration
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum ItemSynNodeDecl {
    Submodule(SubmoduleSynNodeDecl),
    MajorItem(MajorItemSynNodeDecl),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDataDecl),
    TypeVariant(TypeVariantSynNodeDecl),
    Attr(AttrSynNodeDecl),
}

impl ItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> Option<SynExprRegion> {
        match self {
            ItemSynNodeDecl::Submodule(_) => None,
            ItemSynNodeDecl::MajorItem(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            ItemSynNodeDecl::ImplBlock(syn_node_decl) => Some(syn_node_decl.syn_expr_region(db)),
            ItemSynNodeDecl::AssociatedItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db).into()
            }
            ItemSynNodeDecl::TypeVariant(_node_decl) => todo!(),
            ItemSynNodeDecl::Attr(syn_node_decl) => Some(syn_node_decl.syn_expr_region(db)),
        }
    }

    pub fn syn_node_path(self, db: &::salsa::Db) -> ItemSynNodePath {
        match self {
            ItemSynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::MajorItem(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::AssociatedItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            ItemSynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::Attr(_) => todo!(),
        }
    }

    pub fn node_decl_errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            ItemSynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::MajorItem(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::Attr(_) => todo!(),
        }
    }
}

/// A `Decl` is a strict version, handy for subsequent processing
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum SynDecl {
    Submodule(SubmoduleSynDecl),
    MajorItem(MajorItemSynDecl),
    ImplBlock(ImplBlockSynDecl),
    AssociatedItem(AssociatedItemSynDecl),
    TypeVariant(TypeVariantSynDecl),
    Attr(AttrSynDecl),
}

impl SynDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            SynDecl::Submodule(_) => todo!(),
            SynDecl::MajorItem(decl) => decl.template_parameters(db),
            SynDecl::ImplBlock(decl) => decl.template_parameters(db),
            SynDecl::AssociatedItem(decl) => decl.template_parameters(db),
            SynDecl::TypeVariant(_decl) => &[],
            SynDecl::Attr(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> Option<SynExprRegion> {
        match self {
            SynDecl::Submodule(_) => None,
            SynDecl::MajorItem(decl) => decl.syn_expr_region(db).into(),
            SynDecl::ImplBlock(decl) => decl.syn_expr_region(db).into(),
            SynDecl::AssociatedItem(decl) => decl.syn_expr_region(db).into(),
            SynDecl::TypeVariant(_decl) => todo!(),
            SynDecl::Attr(_) => todo!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> ItemPath {
        match self {
            SynDecl::Submodule(_) => todo!(),
            SynDecl::MajorItem(decl) => decl.path(db).into(),
            SynDecl::ImplBlock(decl) => decl.path(db).into(),
            SynDecl::AssociatedItem(decl) => decl.path(db).into(),
            SynDecl::TypeVariant(decl) => decl.path(db).into(),
            SynDecl::Attr(_) => todo!(),
        }
    }
}

pub trait HasSynNodeDecl: Copy {
    type NodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl;
}

impl HasSynNodeDecl for ItemSynNodePath {
    type NodeDecl = ItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        match self {
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::TypeVariant(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::ImplBlock(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::AssociatedItem(syn_node_path) => {
                syn_node_path.syn_node_decl(db).into()
            }
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::Attr(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
        }
    }
}

pub trait HasSynDecl: Copy {
    type Decl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl>;
}

impl HasSynDecl for ItemPath {
    type Decl = SynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        match self {
            ItemPath::Submodule(_, path) => path.syn_decl(db).map(Into::into),
            ItemPath::MajorItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::AssociatedItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::TypeVariant(_, path) => path.syn_decl(db).map(Into::into),
            ItemPath::ImplBlock(path) => path.syn_decl(db).map(Into::into),
            ItemPath::Attr(_, _) => todo!(),
        }
    }
}
