use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    stmt::VmirStmtArena,
    *,
};
use husky_linkage::linkage::Linkage;

#[salsa::tracked]
pub struct VmirRegion {
    pub linkage: Linkage,
    pub root_expr: VmirExprIdx,
    #[return_ref]
    pub vmir_expr_arena: VmirExprArena,
    #[return_ref]
    pub vmir_stmt_arena: VmirStmtArena,
}

#[salsa::tracked]
pub fn linkage_vmir_region(db: &::salsa::Db, linkage: Linkage) -> Option<VmirRegion> {
    let (root_hir_eager_expr_idx, mut builder) = VmirExprBuilder::new(linkage, db)?;
    let root_expr = root_hir_eager_expr_idx.to_vmir(&mut builder);
    let (vmir_expr_arena, vmir_stmt_arena) = builder.finish();
    Some(VmirRegion::new(
        db,
        linkage,
        root_expr,
        vmir_expr_arena,
        vmir_stmt_arena,
    ))
}

#[cfg(test)]
fn package_linkage_vmir_regions(
    db: &::salsa::Db,
    package: husky_vfs::PackagePath,
) -> Vec<(Linkage, Option<VmirRegion>)> {
    use husky_linkage::linkage::package_linkages;

    package_linkages(db, package)
        .iter()
        .map(|&linkage| (linkage, linkage_vmir_region(db, linkage)))
        .collect()
}

#[test]
fn package_linkage_vmir_regions_works() {
    DB::ast_expect_test_debug_with_db(
        package_linkage_vmir_regions,
        &AstTestConfig::new(
            "package_linkage_vmir_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}
