pub mod r#let;
#[cfg(test)]
mod tests;

use self::r#let::VdSemLetClauseDispatch;
use super::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_token::idx::{LxMathTokenIdx, LxRoseTokenIdx};
use visored_syn_expr::clause::{VdSynClauseData, VdSynClauseIdx, VdSynClauseIdxRange};

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemClauseData {
    Verb,
    Let {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSemExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
        dispatch: VdSemLetClauseDispatch,
    },
    Assume {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSemExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Goal {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSemExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Have {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSemExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Show {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSemExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Todo(LxRoseTokenIdx),
}

pub struct VdSemClauseEntry {
    data: VdSemClauseData,
    syn_clause: VdSynClauseIdx,
}

pub type VdSemClauseArena = Arena<VdSemClauseEntry>;
pub type VdSemClauseArenaRef<'a> = ArenaRef<'a, VdSemClauseEntry>;
pub type VdSemClauseIdx = ArenaIdx<VdSemClauseEntry>;
pub type VdSemClauseIdxRange = ArenaIdxRange<VdSemClauseEntry>;
pub type VdSemClauseMap<T> = ArenaMap<VdSemClauseEntry, T>;

impl VdSemClauseEntry {
    pub fn data(&self) -> &VdSemClauseData {
        &self.data
    }

    pub fn syn_clause(&self) -> VdSynClauseIdx {
        self.syn_clause
    }
}

impl ToVdSem<VdSemClauseIdxRange> for VdSynClauseIdxRange {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemClauseIdxRange {
        let mut clauses: Vec<VdSemClauseEntry> = vec![];
        for clause in self {
            let data = builder.build_clause_data(clause);
            clauses.push(VdSemClauseEntry {
                data,
                syn_clause: clause,
            });
        }
        builder.alloc_clauses(clauses)
    }
}

impl<'a> VdSemExprBuilder<'a> {
    fn build_clause_data(&mut self, clause: VdSynClauseIdx) -> VdSemClauseData {
        match *self.syn_clause_arena()[clause].data() {
            VdSynClauseData::Let {
                left_math_delimiter_token_idx,
                formula,
                right_math_delimiter_token_idx,
            } => {
                let resolution = &self.syn_let_clause_resolutions()[clause];
                self.build_let_clause(
                    left_math_delimiter_token_idx,
                    formula,
                    right_math_delimiter_token_idx,
                    resolution,
                )
            }
            VdSynClauseData::Assume {
                left_math_delimiter_token_idx,
                formula,
                right_math_delimiter_token_idx,
            } => VdSemClauseData::Assume {
                left_math_delimiter_token_idx,
                formula: (formula, self.ty_menu().prop).to_vd_sem(self),
                right_math_delimiter_token_idx,
            },
            VdSynClauseData::Goal {
                left_math_delimiter_token_idx,
                formula,
                right_math_delimiter_token_idx,
            } => VdSemClauseData::Goal {
                left_math_delimiter_token_idx,
                formula: (formula, self.ty_menu().prop).to_vd_sem(self),
                right_math_delimiter_token_idx,
            },
            VdSynClauseData::Have {
                left_math_delimiter_token_idx,
                formula,
                right_math_delimiter_token_idx,
            } => VdSemClauseData::Have {
                left_math_delimiter_token_idx,
                formula: (formula, self.ty_menu().prop).to_vd_sem(self),
                right_math_delimiter_token_idx,
            },
            VdSynClauseData::Show {
                left_math_delimiter_token_idx,
                formula,
                right_math_delimiter_token_idx,
            } => VdSemClauseData::Show {
                left_math_delimiter_token_idx,
                formula: (formula, self.ty_menu().prop).to_vd_sem(self),
                right_math_delimiter_token_idx,
            },
        }
    }
}

pub enum VdSemClauseChild {
    Expr(VdSemExprIdx),
}

impl VdSemClauseData {
    pub fn children(&self) -> Vec<VdSemClauseChild> {
        match *self {
            VdSemClauseData::Verb => todo!(),
            VdSemClauseData::Let {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                right_math_delimiter_token_idx: right_dollar_token_idx,
                formula,
                ..
            } => vec![VdSemClauseChild::Expr(formula)],
            VdSemClauseData::Goal {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                right_math_delimiter_token_idx: right_dollar_token_idx,
                formula,
                ..
            } => vec![VdSemClauseChild::Expr(formula)],
            VdSemClauseData::Assume {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => vec![VdSemClauseChild::Expr(formula)],
            VdSemClauseData::Have {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => vec![VdSemClauseChild::Expr(formula)],
            VdSemClauseData::Show {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => vec![VdSemClauseChild::Expr(formula)],
            VdSemClauseData::Todo(..) => vec![],
        }
    }
}
