use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    stmt::VmirStmtArena,
    *,
};
use husky_linkage::linkage::Linkage;
use husky_task::linktime::VirtualLinktime;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirRegion<LinkageImpl: IsLinkageImpl> {
    linkage: Linkage,
    root_expr: VmirExprIdx<LinkageImpl>,
    vmir_expr_arena: VmirExprArena<LinkageImpl>,
    vmir_stmt_arena: VmirStmtArena<LinkageImpl>,
}

pub type VirtualVmirRegion = VmirRegion<Linkage>;

/// # constructors

impl<LinkageImpl: IsLinkageImpl> VmirRegion<LinkageImpl> {
    pub fn new(
        linkage: Linkage,
        root_expr: VmirExprIdx<LinkageImpl>,
        vmir_expr_arena: VmirExprArena<LinkageImpl>,
        vmir_stmt_arena: VmirStmtArena<LinkageImpl>,
    ) -> Self {
        Self {
            linkage,
            root_expr,
            vmir_expr_arena,
            vmir_stmt_arena,
        }
    }
}

/// # getters

impl<LinkageImpl: IsLinkageImpl> VmirRegion<LinkageImpl> {
    pub fn linkage(&self) -> Linkage {
        self.linkage
    }

    pub fn root_expr(&self) -> VmirExprIdx<LinkageImpl> {
        self.root_expr
    }

    pub fn vmir_expr_arena(&self) -> &VmirExprArena<LinkageImpl> {
        &self.vmir_expr_arena
    }

    pub fn vmir_stmt_arena(&self) -> &VmirStmtArena<LinkageImpl> {
        &self.vmir_stmt_arena
    }
}

pub(crate) fn linkage_vmir_region<'comptime, Linktime: IsLinktime>(
    db: &'comptime ::salsa::Db,
    linkage: Linkage,
    linktime: &'comptime Linktime,
) -> Option<VmirRegion<Linktime::LinkageImpl>> {
    let (root_hir_eager_expr_idx, mut builder) = VmirBuilder::new(linkage, db, linktime)?;
    let root_expr = root_hir_eager_expr_idx.to_vmir(&mut builder);
    let (vmir_expr_arena, vmir_stmt_arena) = builder.finish();
    Some(VmirRegion::new(
        linkage,
        root_expr,
        vmir_expr_arena,
        vmir_stmt_arena,
    ))
}

pub fn linkage_virtual_vmir_region(
    db: &::salsa::Db,
    linkage: Linkage,
) -> Option<&VmirRegion<Linkage>> {
    linkage_virtual_vmir_region_aux(db, linkage).as_ref()
}

#[salsa::tracked(return_ref)]
pub fn linkage_virtual_vmir_region_aux(
    db: &::salsa::Db,
    linkage: Linkage,
) -> Option<VmirRegion<Linkage>> {
    let (root_hir_eager_expr_idx, mut builder) = VmirBuilder::new(linkage, db, &VirtualLinktime)?;
    let root_expr = root_hir_eager_expr_idx.to_vmir(&mut builder);
    let (vmir_expr_arena, vmir_stmt_arena) = builder.finish();
    Some(VmirRegion::new(
        linkage,
        root_expr,
        vmir_expr_arena,
        vmir_stmt_arena,
    ))
}

#[cfg(test)]
fn package_linkage_linkage_vmir_regions(
    db: &::salsa::Db,
    package: husky_vfs::PackagePath,
) -> Vec<(Linkage, Option<&VmirRegion<Linkage>>)> {
    use husky_linkage::linkage::package_linkages;

    package_linkages(db, package)
        .iter()
        .map(|&linkage| (linkage, linkage_virtual_vmir_region(db, linkage)))
        .collect()
}

#[test]
fn package_linkage_vmir_regions_works() {
    DB::ast_expect_test_debug_with_db(
        package_linkage_linkage_vmir_regions,
        &AstTestConfig::new(
            "package_linkage_linkage_vmir_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}
