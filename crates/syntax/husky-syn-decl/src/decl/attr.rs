pub mod affect;
pub mod backprop;
pub mod derive;
pub mod marker;
pub mod task;
pub mod test;

use self::derive::*;
use self::{
    affect::{AffectAttrSynDecl, AffectAttrSynNodeDecl},
    backprop::{BackpropAttrSynDecl, BackpropAttrSynNodeDecl},
    task::{TaskAttrSynDecl, TaskAttrSynNodeDecl},
    test::{TestAttrSynDecl, TestAttrSynNodeDecl},
};
use super::*;
use husky_coword::coword_menu;
use husky_entity_path::path::attr::AttrItemPath;
use husky_entity_tree::node::attr::AttrSynNodePath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynNodeDecl {
    Backprop(BackpropAttrSynNodeDecl),
    Derive(DeriveAttrSynNodeDecl),
    Effect(AffectAttrSynNodeDecl),
    Task(TaskAttrSynNodeDecl),
    Test(TestAttrSynNodeDecl),
    Err,
}

/// # getters
impl AttrSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> Option<SynExprRegion> {
        Some(match self {
            AttrSynNodeDecl::Derive(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Backprop(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Effect(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Task(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Test(slf) => slf.syn_expr_region(db),
            AttrSynNodeDecl::Err => return None,
        })
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            AttrSynNodeDecl::Derive(slf) => slf.errors(db),
            AttrSynNodeDecl::Backprop(slf) => slf.errors(db),
            AttrSynNodeDecl::Effect(slf) => slf.errors(db),
            AttrSynNodeDecl::Task(slf) => slf.errors(db),
            AttrSynNodeDecl::Test(slf) => slf.errors(db),
            AttrSynNodeDecl::Err => todo!(),
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
        attr_ident if attr_ident == coword_menu.task_ident() => {
            AttrSynNodeDecl::Task(TaskAttrSynNodeDecl::new(db, syn_node_path))
        }
        attr_ident if attr_ident == coword_menu.test_ident() => {
            AttrSynNodeDecl::Test(TestAttrSynNodeDecl::new(db, syn_node_path))
        }
        _ => AttrSynNodeDecl::Err,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynDecl {
    Backprop(BackpropAttrSynDecl),
    Derive(DeriveAttrSynDecl),
    Effect(AffectAttrSynDecl),
    Task(TaskAttrSynDecl),
    Test(TestAttrSynDecl),
}

/// # constructor
impl AttrSynDecl {
    #[inline(always)]
    fn from_node(
        db: &::salsa::Db,
        path: AttrItemPath,
        syn_node_decl: AttrSynNodeDecl,
    ) -> SynDeclResult<Self> {
        Ok(match syn_node_decl {
            AttrSynNodeDecl::Derive(syn_node_decl) => {
                DeriveAttrSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            AttrSynNodeDecl::Backprop(syn_node_decl) => {
                BackpropAttrSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            AttrSynNodeDecl::Effect(syn_node_decl) => {
                AffectAttrSynDecl::from_node(path, syn_node_decl, db)?.into()
            }
            AttrSynNodeDecl::Task(syn_node_decl) => {
                TaskAttrSynDecl::from_node(path, syn_node_decl, db)?.into()
            }
            AttrSynNodeDecl::Test(node_decl) => {
                TestAttrSynDecl::from_node(path, node_decl, db)?.into()
            }
            AttrSynNodeDecl::Err => todo!(),
        })
    }
}

/// # getters
impl AttrSynDecl {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrSynDecl::Backprop(slf) => slf.path(db),
            AttrSynDecl::Derive(slf) => slf.path(db),
            AttrSynDecl::Task(slf) => slf.path(db),
            AttrSynDecl::Test(slf) => slf.path(db),
            AttrSynDecl::Effect(slf) => slf.path(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AttrSynDecl::Backprop(slf) => slf.syn_expr_region(db),
            AttrSynDecl::Derive(slf) => slf.syn_expr_region(db),
            AttrSynDecl::Task(slf) => slf.syn_expr_region(db),
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
    AttrSynDecl::from_node(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn attr_decl_works() {}
