mod derive;

use husky_coword::coword_menu;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrSynNodeDecl {
    Derive(DeriveAttrSynNodeDecl),
}

impl AttrSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            AttrSynNodeDecl::Derive(syn_node_decl) => syn_node_decl.syn_expr_region(db),
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
}

impl AttrSynDecl {
    #[inline(always)]
    fn from_node_decl(
        db: &::salsa::Db,
        path: AttrItemPath,
        syn_node_decl: AttrSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            AttrSynNodeDecl::Derive(syn_node_decl) => {
                DeriveAttrSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }
}

impl HasSynDecl for AttrItemPath {
    type Decl = AttrSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        attr_syn_decl(db, self)
    }
}

// #[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn attr_syn_decl(db: &::salsa::Db, path: AttrItemPath) -> DeclResult<AttrSynDecl> {
    AttrSynDecl::from_node_decl(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn attr_decl_works() {}
