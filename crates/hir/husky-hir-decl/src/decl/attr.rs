pub mod backprop;
pub mod derive;
pub mod effect;
pub mod test;

use self::{
    backprop::BackpropAttrHirDecl, derive::DeriveAttrHirDecl, effect::EffectAttrHirDecl,
    test::TestAttrHirDecl,
};
use super::*;
use husky_entity_path::path::attr::AttrItemPath;
use husky_syn_decl::decl::AttrSynDecl;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AttrHirDecl {
    Backprop(BackpropAttrHirDecl),
    Derive(DeriveAttrHirDecl),
    Effect(EffectAttrHirDecl),
    Test(TestAttrHirDecl),
}

impl AttrHirDecl {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrHirDecl::Backprop(slf) => slf.path(db),
            AttrHirDecl::Derive(slf) => slf.path(db),
            AttrHirDecl::Effect(slf) => slf.path(db),
            AttrHirDecl::Test(slf) => slf.path(db),
        }
    }
}

impl HasHirDecl for AttrItemPath {
    type HirDecl = AttrHirDecl;

    fn hir_decl(self, db: &salsa::Db) -> Option<Self::HirDecl> {
        attr_hir_decl(db, self)
    }
}

#[salsa::tracked]
fn attr_hir_decl(db: &::salsa::Db, path: AttrItemPath) -> Option<AttrHirDecl> {
    match path.syn_decl(db).unwrap() {
        AttrSynDecl::Backprop(syn_decl) => {
            Some(BackpropAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
        AttrSynDecl::Derive(syn_decl) => {
            Some(DeriveAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
        AttrSynDecl::Effect(syn_decl) => {
            Some(EffectAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
        AttrSynDecl::Test(syn_decl) => Some(TestAttrHirDecl::from_syn(path, syn_decl, db).into()),
    }
}
