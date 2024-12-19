pub mod builder;
pub mod item;
pub mod module;

use crate::{
    block::{VdSynBlockArenaRef, VdSynBlockIdxRange, VdSynBlockMap},
    division::{VdSynDivisionArenaRef, VdSynDivisionMap},
    expr::VdSynExprIdx,
    helpers::tracker::IsVdSynOutput,
};
use builder::VdSynExprEntityTreeBuilder;
use either::*;
use eterned::db::EternerDb;
use latex_vfs::path::LxFilePath;
use visored_entity_path::module::{VdModulePath, VdModulePathRegistry};
use visored_global_resolution::default_table::VdDefaultGlobalResolutionTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdSynExprEntityTreeNode {
    module_path: VdModulePath,
    children: Vec<VdModulePath>,
}

impl VdSynExprEntityTreeNode {
    pub fn new(module_path: VdModulePath, children: Vec<VdModulePath>) -> Self {
        Self {
            module_path,
            children,
        }
    }
}

pub(crate) fn build_entity_tree_with(
    db: &EternerDb,
    default_global_resolution_table: &VdDefaultGlobalResolutionTable,
    file_path: LxFilePath,
    stmt_arena: VdSynBlockArenaRef,
    division_arena: VdSynDivisionArenaRef,
    output: impl IsVdSynOutput,
) -> (
    VdSynExprEntityTreeNode,
    VdSynBlockMap<VdSynExprEntityTreeNode>,
    VdSynDivisionMap<VdSynExprEntityTreeNode>,
) {
    let mut builder = VdSynExprEntityTreeBuilder::new(
        db,
        default_global_resolution_table,
        file_path,
        stmt_arena,
        division_arena,
    );
    let root_node = output.build_entity_tree_root_node(&mut builder);
    let (stmt_entity_tree_node_map, division_entity_tree_node_map) = builder.finish();
    (
        root_node,
        stmt_entity_tree_node_map,
        division_entity_tree_node_map,
    )
}
