pub mod affect;
pub mod backprop;
pub mod dep;
pub mod derive;
pub mod mark;
pub mod proj;
pub mod task;
pub mod test;

use self::{
    affect::AffectAttrHirDecl, backprop::BackpropAttrHirDecl, dep::DepAttrHirDecl,
    derive::DeriveAttrHirDecl, task::TaskAttrHirDecl, test::TestAttrHirDecl,
};
use super::*;
use husky_entity_path::path::attr::AttrItemPath;
use husky_syn_decl::decl::attr::AttrSynDecl;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AttrHirDecl {
    Affect(AffectAttrHirDecl),
    Backprop(BackpropAttrHirDecl),
    Dep(DepAttrHirDecl),
    Derive(DeriveAttrHirDecl),
    Task(TaskAttrHirDecl),
    Test(TestAttrHirDecl),
}

impl AttrHirDecl {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        match self {
            AttrHirDecl::Backprop(slf) => slf.path(db),
            AttrHirDecl::Dep(slf) => slf.path(db),
            AttrHirDecl::Derive(slf) => slf.path(db),
            AttrHirDecl::Affect(slf) => slf.path(db),
            AttrHirDecl::Task(slf) => slf.path(db),
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
    // todo: should use eth template??
    match path.syn_decl(db).unwrap() {
        AttrSynDecl::Affect(syn_decl) => {
            Some(AffectAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
        AttrSynDecl::Backprop(syn_decl) => {
            Some(BackpropAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
        AttrSynDecl::Dep(syn_decl) => Some(DepAttrHirDecl::from_syn(path, syn_decl, db).into()),
        AttrSynDecl::Derive(syn_decl) => {
            Some(DeriveAttrHirDecl::from_syn(path, syn_decl, db).into())
        }
        AttrSynDecl::Projection(syn_decl) => todo!(),
        AttrSynDecl::Singleton(syn_decl) => todo!(),
        AttrSynDecl::Task(syn_decl) => Some(TaskAttrHirDecl::from_syn(path, syn_decl, db).into()),
        AttrSynDecl::Test(syn_decl) => Some(TestAttrHirDecl::from_syn(path, syn_decl, db).into()),
    }
}
