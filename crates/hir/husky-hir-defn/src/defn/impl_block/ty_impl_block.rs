use super::*;
use husky_entity_path::path::impl_block::ty_impl_block::TypeImplBlockPath;
use husky_hir_decl::decl::TypeImplBlockHirDecl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[salsa::as_id(jar = HirDefnJar)]
pub struct TypeImplBlockHirDefn {
    hir_decl: TypeImplBlockHirDecl,
}

impl From<TypeImplBlockHirDefn> for HirDefn {
    fn from(hir_defn: TypeImplBlockHirDefn) -> Self {
        HirDefn::ImplBlock(hir_defn.into())
    }
}

impl HasHirDefn for TypeImplBlockPath {
    type HirDefn = TypeImplBlockHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        Some(TypeImplBlockHirDefn {
            hir_decl: self.hir_decl(db)?,
        })
    }
}

impl TypeImplBlockHirDefn {
    pub fn hir_decl(self) -> TypeImplBlockHirDecl {
        self.hir_decl
    }

    pub fn path(self, db: &::salsa::Db) -> TypeImplBlockPath {
        self.hir_decl.path(db)
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        ty_impl_block_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_impl_block_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn ty_impl_block_deps(db: &::salsa::Db, hir_defn: TypeImplBlockHirDefn) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl();
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.self_ty(db));
    builder.finish()
}

#[salsa::tracked]
fn ty_impl_block_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeImplBlockHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
