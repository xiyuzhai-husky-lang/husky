#[cfg(test)]
mod tests;

use std::iter::Peekable;

use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    stmt::{VdSynStmtIdx, VdSynStmtIdxRange},
    *,
};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_ast::ast::{
    rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange},
    LxAstIdxRange,
};
use latex_prelude::division::LxDivisionKind;
use latex_token::idx::LxTokenIdxRange;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynDivisionData {
    Stmts {
        stmts: VdSynStmtIdxRange,
    },
    Divisions {
        kind: LxDivisionKind,
        divisions: VdSynDivisionIdxRange,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynDivisionChild {
    Division(VdSynDivisionIdx),
    Stmt(VdSynStmtIdx),
}

impl VdSynDivisionData {
    pub fn kind(&self) -> LxDivisionKind {
        match *self {
            VdSynDivisionData::Stmts { .. } => LxDivisionKind::Stmts,
            VdSynDivisionData::Divisions { kind, .. } => kind,
        }
    }

    pub fn children(&self) -> Vec<VdSynDivisionChild> {
        match *self {
            VdSynDivisionData::Stmts { stmts } => stmts
                .into_iter()
                .map(|stmt| VdSynDivisionChild::Stmt(stmt))
                .collect(),
            VdSynDivisionData::Divisions { divisions, .. } => divisions
                .into_iter()
                .map(|division| VdSynDivisionChild::Division(division))
                .collect(),
        }
    }
}

pub type VdSynDivisionArena = Arena<VdSynDivisionData>;
pub type VdSynDivisionArenaRef<'a> = ArenaRef<'a, VdSynDivisionData>;
pub type VdSynDivisionMap<T> = ArenaMap<VdSynDivisionData, T>;
pub type VdSynDivisionIdx = ArenaIdx<VdSynDivisionData>;
pub type VdSynDivisionIdxRange = ArenaIdxRange<VdSynDivisionData>;

impl ToVdSyn<VdSynDivisionIdxRange> for (LxTokenIdxRange, LxRoseAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynDivisionIdxRange {
        let (token_range, asts) = self;
        let mut ast_iter = asts.into_iter().peekable();
        let mut divisions = vec![];
        while let Some(division) = builder.build_division(&mut ast_iter) {
            divisions.push(division);
        }
        builder.alloc_divisions(divisions)
    }
}

impl<'a> VdSynExprBuilder<'a> {
    pub(crate) fn build_division(
        &mut self,
        ast_iter: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> Option<VdSynDivisionData> {
        let ast_arena = self.ast_arena();
        let ast = *ast_iter.peek()?;
        Some(match ast_arena[ast] {
            LxRoseAstData::NewDivision {
                command_token_idx,
                lcurl_token_idx,
                title,
                rcurl_token_idx,
            } => todo!(),
            _ => VdSynDivisionData::Stmts {
                stmts: self.parse_stmt_aux(ast_iter),
            },
        })
    }
}
