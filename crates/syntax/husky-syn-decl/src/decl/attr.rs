pub mod backprop;
pub mod derive;
pub mod effect;
pub mod test;

pub use self::derive::*;

use self::{
    backprop::BackpropAttrSynNodeDecl,
    effect::EffectAttrSynNodeDecl,
    test::{TestAttrSynDecl, TestAttrSynNodeDecl},
};
use super::*;
use husky_coword::coword_menu;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynNodeDecl {
    Backprop(BackpropAttrSynNodeDecl),
    Derive(DeriveAttrSynNodeDecl),
    Effect(EffectAttrSynNodeDecl),
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
    // ad hoc
    let coword_menu = coword_menu(db);
    let attr_ident = syn_node_path.ident(db);
    if attr_ident == coword_menu.derive_ident() {
        AttrSynNodeDecl::Derive(DeriveAttrSynNodeDecl::new(db, syn_node_path))
    } else {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynDecl {
    Derive(DeriveAttrSynDecl),
    Test(TestAttrSynDecl),
}

/// # constructor
impl AttrSynDecl {
    #[inline(always)]
    fn from_node_decl(
        db: &::salsa::Db,
        path: AttrItemPath,
        node_decl: AttrSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match node_decl {
            AttrSynNodeDecl::Derive(node_decl) => {
                DeriveAttrSynDecl::from_node_decl(db, path, node_decl)?.into()
            }
            AttrSynNodeDecl::Backprop(_) => todo!(),
            AttrSynNodeDecl::Effect(_) => todo!(),
            AttrSynNodeDecl::Test(node_decl) => TestAttrSynDecl::from_node(node_decl, db)?.into(),
        })
    }
}

/// # getters
impl AttrSynDecl {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrSynDecl::Derive(slf) => slf.path(db),
            AttrSynDecl::Test(slf) => slf.path(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AttrSynDecl::Derive(slf) => slf.syn_expr_region(db),
            AttrSynDecl::Test(slf) => slf.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for AttrItemPath {
    type Decl = AttrSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        attr_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn attr_syn_decl(db: &::salsa::Db, path: AttrItemPath) -> DeclResult<AttrSynDecl> {
    AttrSynDecl::from_node_decl(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn attr_decl_works() {}
