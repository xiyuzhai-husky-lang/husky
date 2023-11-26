use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
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

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        ty_impl_block_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_impl_block_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_impl_block_dependencies(
    db: &::salsa::Db,
    hir_defn: TypeImplBlockHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl();
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.self_ty(db));
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_impl_block_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeImplBlockHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
