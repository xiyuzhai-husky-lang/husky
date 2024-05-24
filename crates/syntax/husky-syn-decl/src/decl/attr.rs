pub mod affect;
pub mod backprop;
pub mod derive;
pub mod marker;
pub mod test;

use self::derive::*;
use self::{
    affect::{AffectAttrSynDecl, AffectAttrSynNodeDecl},
    backprop::{BackpropAttrSynDecl, BackpropAttrSynNodeDecl},
    test::{TestAttrSynDecl, TestAttrSynNodeDecl},
};
use super::*;
use husky_coword::coword_menu;
use husky_entity_path::path::attr::AttrItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynNodeDecl {
    Backprop(BackpropAttrSynNodeDecl),
    Derive(DeriveAttrSynNodeDecl),
    Effect(AffectAttrSynNodeDecl),
    Test(TestAttrSynNodeDecl),
}

/// # getters
impl AttrSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AttrSynNodeDecl::Derive(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Backprop(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Effect(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Test(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            AttrSynNodeDecl::Derive(slf) => slf.errors(db),
            AttrSynNodeDecl::Backprop(slf) => slf.errors(db),
            AttrSynNodeDecl::Effect(slf) => slf.errors(db),
            AttrSynNodeDecl::Test(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for AttrSynNodePath {
    type NodeDecl = AttrSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        attr_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
fn attr_syn_node_decl(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> AttrSynNodeDecl {
    let coword_menu = coword_menu(db);
    let attr_ident = syn_node_path.ident(db);
    match attr_ident {
        attr_ident if attr_ident == coword_menu.backprop_ident() => {
            AttrSynNodeDecl::Backprop(BackpropAttrSynNodeDecl::new(db, syn_node_path))
        }
        attr_ident if attr_ident == coword_menu.derive_ident() => {
            AttrSynNodeDecl::Derive(DeriveAttrSynNodeDecl::new(db, syn_node_path))
        }
        attr_ident if attr_ident == coword_menu.affect_ident() => {
            AttrSynNodeDecl::Effect(AffectAttrSynNodeDecl::new(db, syn_node_path))
        }
        attr_ident if attr_ident == coword_menu.test_ident() => {
            AttrSynNodeDecl::Test(TestAttrSynNodeDecl::new(db, syn_node_path))
        }
        _ => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynDecl {
    Backprop(BackpropAttrSynDecl),
    Derive(DeriveAttrSynDecl),
    Effect(AffectAttrSynDecl),
    Test(TestAttrSynDecl),
}

/// # constructor
impl AttrSynDecl {
    #[inline(always)]
    fn from_node_decl(
        db: &::salsa::Db,
        path: AttrItemPath,
        syn_node_decl: AttrSynNodeDecl,
    ) -> SynDeclResult<Self> {
        Ok(match syn_node_decl {
            AttrSynNodeDecl::Derive(syn_node_decl) => {
                DeriveAttrSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            AttrSynNodeDecl::Backprop(syn_node_decl) => {
                BackpropAttrSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            AttrSynNodeDecl::Effect(node_decl) => {
                AffectAttrSynDecl::from_node(path, node_decl, db)?.into()
            }
            AttrSynNodeDecl::Test(node_decl) => {
                TestAttrSynDecl::from_node(path, node_decl, db)?.into()
            }
        })
    }
}

/// # getters
impl AttrSynDecl {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrSynDecl::Backprop(slf) => slf.path(db),
            AttrSynDecl::Derive(slf) => slf.path(db),
            AttrSynDecl::Test(slf) => slf.path(db),
            AttrSynDecl::Effect(slf) => slf.path(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AttrSynDecl::Backprop(slf) => slf.syn_expr_region(db),
            AttrSynDecl::Derive(slf) => slf.syn_expr_region(db),
            AttrSynDecl::Test(slf) => slf.syn_expr_region(db),
            AttrSynDecl::Effect(slf) => slf.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for AttrItemPath {
    type Decl = AttrSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> SynDeclResult<Self::Decl> {
        attr_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn attr_syn_decl(db: &::salsa::Db, path: AttrItemPath) -> SynDeclResult<AttrSynDecl> {
    AttrSynDecl::from_node_decl(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn attr_decl_works() {}
