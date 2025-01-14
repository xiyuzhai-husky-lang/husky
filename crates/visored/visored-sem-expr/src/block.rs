#[cfg(test)]
mod tests;

use crate::sentence::VdSemSentenceIdxRange;
use crate::*;
use husky_coword::BaseCoword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_environment::signature::LxEnvironmentSignature;
use latex_token::idx::LxRoseTokenIdx;
use sentence::VdSemSentenceIdx;
use visored_entity_path::module::VdModulePath;
use visored_global_resolution::resolution::environment::VdEnvironmentGlobalResolution;
use visored_syn_expr::block::{VdSynBlockData, VdSynBlockIdx};

pub struct VdSemBlockEntry {
    data: VdSemBlockData,
    module_path: VdModulePath,
}

pub enum VdSemBlockData {
    Paragraph(VdSemSentenceIdxRange),
    Environment {
        begin_command_token_idx: LxRoseTokenIdx,
        environment_signature: LxEnvironmentSignature,
        resolution: VdEnvironmentGlobalResolution,
        blocks: VdSemBlockIdxRange,
        end_rcurl_token_idx: LxRoseTokenIdx,
    },
}

pub type VdSemBlockArena = Arena<VdSemBlockEntry>;
pub type VdSemBlockArenaRef<'a> = ArenaRef<'a, VdSemBlockEntry>;
pub type VdSemBlockIdx = ArenaIdx<VdSemBlockEntry>;
pub type VdSemBlockIdxRange = ArenaIdxRange<VdSemBlockEntry>;
pub type VdSemBlockMap<T> = ArenaMap<VdSemBlockEntry, T>;
pub type VdSemBlockOrderedMap<T> = ArenaOrderedMap<VdSemBlockEntry, T>;

impl VdSemBlockEntry {
    pub fn new(data: VdSemBlockData, module_path: VdModulePath) -> Self {
        Self { data, module_path }
    }
}

impl VdSemBlockEntry {
    pub fn data(&self) -> &VdSemBlockData {
        &self.data
    }

    pub fn module_path(&self) -> VdModulePath {
        self.module_path
    }
}

impl ToVdSem<VdSemBlockIdxRange> for VdSynBlockIdxRange {
    // there is no need to cache because stmts will be created in one go
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemBlockIdxRange {
        let mut stmts: Vec<VdSemBlockEntry> = vec![];
        for stmt in self {
            let module_path = builder.stmt_entity_tree_node_map()[stmt].module_path();
            stmts.push(VdSemBlockEntry::new(builder.build_stmt(stmt), module_path));
        }
        builder.alloc_stmts(stmts)
    }
}

impl<'a> VdSemExprBuilder<'a> {
    pub(crate) fn build_stmt(&mut self, stmt: VdSynBlockIdx) -> VdSemBlockData {
        match self.syn_stmt_arena()[stmt] {
            VdSynBlockData::Paragraph(sentences) => {
                VdSemBlockData::Paragraph(sentences.to_vd_sem(self))
            }
            VdSynBlockData::Environment {
                environment_signature,
                resolution,
                stmts,
                begin_command_token_idx,
                end_rcurl_token_idx,
            } => VdSemBlockData::Environment {
                environment_signature,
                resolution,
                blocks: stmts.to_vd_sem(self),
                begin_command_token_idx,
                end_rcurl_token_idx,
            },
        }
    }
}

pub enum VdSemBlockChild {
    Sentence(VdSemSentenceIdx),
    Stmt(VdSemBlockIdx),
}

impl VdSemBlockData {
    pub fn children(&self) -> Vec<VdSemBlockChild> {
        match *self {
            VdSemBlockData::Paragraph(sentences) => sentences
                .into_iter()
                .map(|s| VdSemBlockChild::Sentence(s))
                .collect(),
            VdSemBlockData::Environment {
                environment_signature,
                resolution,
                blocks: stmts,
                begin_command_token_idx,
                end_rcurl_token_idx,
            } => stmts
                .into_iter()
                .map(|s| VdSemBlockChild::Stmt(s))
                .collect(),
        }
    }
}
