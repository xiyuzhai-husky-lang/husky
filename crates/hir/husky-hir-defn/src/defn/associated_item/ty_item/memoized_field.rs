use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeMemoizedFieldHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TypeMemoizedFieldHirDefn> for AssociatedItemHirDefn {
    fn from(hir_defn: TypeMemoizedFieldHirDefn) -> Self {
        AssociatedItemHirDefn::TypeItem(hir_defn.into())
    }
}

impl From<TypeMemoizedFieldHirDefn> for HirDefn {
    fn from(hir_defn: TypeMemoizedFieldHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TypeMemoizedFieldHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: TypeItemPath,
        hir_decl: TypeMemoizedFieldHirDecl,
    ) -> TypeMemoizedFieldHirDefn {
        let Ok(TypeItemSynDefn::MemoizedField(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        TypeMemoizedFieldHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> Option<HirEagerExprRegion> {
        Some(self.eager_body_with_hir_eager_expr_region(db)?.1)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        ty_memoized_field_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_memoized_field_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_memoized_field_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TypeMemoizedFieldHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).impl_block(db));
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_memoized_field_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeMemoizedFieldHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
