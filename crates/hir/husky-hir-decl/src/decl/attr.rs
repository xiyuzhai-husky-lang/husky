mod derive;

use husky_syn_decl::AttrSynDecl;

pub use self::derive::*;

use super::*;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AttrHirDecl {
    Derive(DeriveAttrHirDecl),
}

impl AttrHirDecl {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrHirDecl::Derive(hir_decl) => hir_decl.path(db),
        }
    }
}

impl HasHirDecl for AttrItemPath {
    type HirDecl = AttrHirDecl;

    fn hir_decl(self, db: &salsa::Db) -> Option<Self::HirDecl> {
        attr_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn attr_hir_decl(db: &::salsa::Db, path: AttrItemPath) -> Option<AttrHirDecl> {
    match path.syn_decl(db).unwrap() {
        AttrSynDecl::Derive(syn_decl) => {
            Some(DeriveAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
    }
}
