use crate::{
    expr::{VmirExprArena, VmirExprIdx},
    stmt::VmirStmtArena,
    *,
};
use husky_linkage::linkage::Linkage;

#[salsa::tracked]
pub struct VmirRegion {
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
        root_expr,
        vmir_expr_arena,
        vmir_stmt_arena,
    ))
}
