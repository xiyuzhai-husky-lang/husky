pub mod builder;
pub mod item;
pub mod module;

use crate::{
    division::{VdSynDivisionArenaRef, VdSynDivisionMap},
    expr::VdSynExprIdx,
    stmt::{VdSynStmtArenaRef, VdSynStmtIdxRange, VdSynStmtMap},
};
use builder::VdSynExprEntityTreeBuilder;
use either::*;
use latex_vfs::path::LxFilePath;
use visored_item_path::module::{VdModulePath, VdModulePathRegistry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdSynExprEntityTreeNode {
    module_path: VdModulePath,
    children: Vec<VdModulePath>,
}

pub(crate) fn build_entity_tree_in_expr_or_stmts(
    db: &::salsa::Db,
    file_path: LxFilePath,
    stmt_arena: VdSynStmtArenaRef,
    division_arena: VdSynDivisionArenaRef,
    expr_or_stmts: Either<VdSynExprIdx, VdSynStmtIdxRange>,
) -> (VdSynStmtMap<VdModulePath>, VdSynDivisionMap<VdModulePath>) {
    let mut builder = VdSynExprEntityTreeBuilder::new(db, stmt_arena, division_arena);
    let root_node = match expr_or_stmts {
        Left(expr) => VdSynExprEntityTreeNode {
            module_path: VdModulePath::new_root(db, file_path),
            children: vec![],
        },
        Right(stmts) => builder.build_root_stmts(file_path, stmts),
    };
    builder.finish()
}
