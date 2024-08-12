use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    stmt::VmirStmtArena,
    *,
};
use husky_linket::linket::Linket;
use husky_virtual_linket_impl::VirtualLinketImpl;
use husky_virtual_linktime::VirtualLinktime;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirRegion<LinketImpl: IsLinketImpl> {
    linket: Linket,
    root_expr: VmirExprIdx<LinketImpl>,
    vmir_expr_arena: VmirExprArena<LinketImpl>,
    vmir_stmt_arena: VmirStmtArena<LinketImpl>,
}

pub type VirtualVmirRegion = VmirRegion<VirtualLinketImpl>;

/// # constructors

impl<LinketImpl: IsLinketImpl> VmirRegion<LinketImpl> {
    pub fn new(
        linket: Linket,
        root_expr: VmirExprIdx<LinketImpl>,
        vmir_expr_arena: VmirExprArena<LinketImpl>,
        vmir_stmt_arena: VmirStmtArena<LinketImpl>,
    ) -> Self {
        Self {
            linket,
            root_expr,
            vmir_expr_arena,
            vmir_stmt_arena,
        }
    }
}

/// # getters

impl<LinketImpl: IsLinketImpl> VmirRegion<LinketImpl> {
    pub fn linket(&self) -> Linket {
        self.linket
    }

    pub fn root_expr(&self) -> VmirExprIdx<LinketImpl> {
        self.root_expr
    }

    pub fn vmir_expr_arena(&self) -> &VmirExprArena<LinketImpl> {
        &self.vmir_expr_arena
    }

    pub fn vmir_stmt_arena(&self) -> &VmirStmtArena<LinketImpl> {
        &self.vmir_stmt_arena
    }
}

pub(crate) fn linket_vmir_region<'comptime, Linktime: IsLinktime>(
    linket: Linket,
    db: &'comptime ::salsa::Db,
    linktime: &'comptime Linktime,
) -> Option<VmirRegion<Linktime::LinketImpl>> {
    let (root_hir_eager_expr_idx, mut builder) = VmirBuilder::new(linket, db, linktime)?;
    let root_expr = root_hir_eager_expr_idx.to_vmir(&mut builder);
    let (vmir_expr_arena, vmir_stmt_arena) = builder.finish();
    Some(VmirRegion::new(
        linket,
        root_expr,
        vmir_expr_arena,
        vmir_stmt_arena,
    ))
}

pub fn linket_virtual_vmir_region(db: &::salsa::Db, linket: Linket) -> Option<&VirtualVmirRegion> {
    linket_virtual_vmir_region_aux(db, linket).as_ref()
}

#[salsa::tracked(return_ref)]
pub fn linket_virtual_vmir_region_aux(
    db: &::salsa::Db,
    linket: Linket,
) -> Option<VirtualVmirRegion> {
    let (root_hir_eager_expr_idx, mut builder) = VmirBuilder::new(linket, db, &VirtualLinktime)?;
    let root_expr = root_hir_eager_expr_idx.to_vmir(&mut builder);
    let (vmir_expr_arena, vmir_stmt_arena) = builder.finish();
    Some(VmirRegion::new(
        linket,
        root_expr,
        vmir_expr_arena,
        vmir_stmt_arena,
    ))
}

#[cfg(test)]
fn package_linket_linket_vmir_regions(
    db: &::salsa::Db,
    package: husky_vfs::path::package_path::PackagePath,
) -> Vec<(Linket, Option<&VirtualVmirRegion>)> {
    use husky_linket::linket::package_linkets;

    package_linkets(db, package)
        .iter()
        .map(|&linket| (linket, linket_virtual_vmir_region(db, linket)))
        .collect()
}

#[test]
fn package_linket_vmir_regions_works() {
    DB::ast_rich_test_debug_with_db(
        package_linket_linket_vmir_regions,
        &AstTestConfig::new(
            "package_linket_linket_vmir_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKET,
        ),
    )
}
