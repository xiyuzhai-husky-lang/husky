pub mod condition;
pub mod if_else_stmt;
mod let_stmt;
pub mod loop_stmt;
pub mod match_stmt;

use self::condition::*;
use self::if_else_stmt::*;
use self::loop_stmt::*;
use self::match_stmt::*;

use husky_regional_token::{
    AssertRegionalToken, BreakRegionalToken, DoRegionalToken, EolRegionalToken,
    EolSemicolonRegionalToken, EolWithRegionalToken, EqRegionalToken, ForextRegionalToken,
    LetRegionalToken, MatchRegionalToken, RequireRegionalToken, ReturnRegionalToken,
    StmtForRegionalToken, WhileRegionalToken,
};
use husky_token_data::TokenDataResult;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

use crate::{obelisks::let_variable::LetVariableObelisk, *};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SemaStmtData {
    Let {
        let_token: LetRegionalToken,
        let_pattern_sema_obelisk: LetVariableObelisk,
        contract: TermContract,
        eq_token: EqRegionalToken,
        initial_value_sema_expr_idx: SemaExprIdx,
        coersion_outcome: Option<ExpectCoersionOutcome>,
    },
    Return {
        return_token: ReturnRegionalToken,
        result: SemaExprIdx,
        coersion_outcome: Option<ExpectCoersionOutcome>,
    },
    Require {
        require_token: RequireRegionalToken,
        condition: SemaCondition,
    },
    Assert {
        assert_token: AssertRegionalToken,
        condition: SemaCondition,
    },
    Break {
        break_token: BreakRegionalToken,
    },
    Eval {
        sema_expr_idx: SemaExprIdx,
        outcome: Option<ExpectationOutcome>,
        // todo: change this to EolOrEolSemicolonToken
        eol_semicolon: TokenDataResult<Option<EolSemicolonRegionalToken>>,
    },
    ForBetween {
        for_token: StmtForRegionalToken,
        particulars: SemaForBetweenParticulars,
        for_loop_var_symbol_idx: CurrentSynSymbolIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    ForIn {
        for_token: StmtForRegionalToken,
        condition: SemaExprIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    Forext {
        forext_token: ForextRegionalToken,
        particulars: SemaForextParticulars,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    While {
        while_token: WhileRegionalToken,
        condition: SemaCondition,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    DoWhile {
        do_token: DoRegionalToken,
        while_token: WhileRegionalToken,
        condition: SemaCondition,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    IfElse {
        if_branch: SemaIfBranch,
        elif_branches: Vec<SemaElifBranch>,
        else_branch: Option<SemaElseBranch>,
    },
    Match {
        match_token: MatchRegionalToken,
        match_target: SemaExprIdx,
        eol_with_token: EolWithRegionalToken,
        case_branches: Vec<SemaCaseBranch>,
    },
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaStmtEntry {
    data_result: SemaExprDataResult<SemaStmtData>,
    ty_result: SemaExprTypeResult<FlyTerm>,
}

#[salsa::debug_with_db]
#[derive(Debug, Default)]
pub(crate) struct SemaStmtBatch {
    entries: SmallVec<[SemaStmtEntry; 8]>,
}

impl SemaStmtBatch {
    pub(crate) fn add(
        &mut self,
        (data_result, ty_result): (
            SemaExprDataResult<SemaStmtData>,
            SemaExprTypeResult<FlyTerm>,
        ),
    ) {
        self.entries.push(SemaStmtEntry {
            data_result,
            ty_result,
        })
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SemaStmtArena(Arena<SemaStmtEntry>);

impl SemaStmtArena {
    pub(crate) fn alloc_batch(&mut self, batch: SemaStmtBatch) -> SemaStmtIdxRange {
        SemaStmtIdxRange(self.0.alloc_batch(batch.entries))
    }

    pub fn arena_ref<'a>(&'a self) -> SemaStmtArenaRef<'a> {
        SemaStmtArenaRef(self.0.to_ref())
    }
}

#[derive(Clone, Copy)]
pub struct SemaStmtArenaRef<'a>(ArenaRef<'a, SemaStmtEntry>);

impl<'a> SemaStmtArenaRef<'a> {
    pub fn len(self) -> usize {
        self.0.len()
    }
}

impl<'a> std::ops::Index<SemaStmtIdx> for SemaStmtArenaRef<'a> {
    type Output = SemaStmtEntry;

    fn index(&self, index: SemaStmtIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemaStmtIdx(ArenaIdx<SemaStmtEntry>);

impl SemaStmtIdx {
    pub fn data<'a>(self, arena_ref: SemaStmtArenaRef<'a>) -> &'a SemaStmtData {
        arena_ref
            .0
            .index(self.0)
            .data_result
            .as_ref()
            .expect("no error")
    }

    pub fn index(self) -> usize {
        self.0.index()
    }
}

impl std::ops::Sub<usize> for SemaStmtIdx {
    type Output = SemaStmtIdx;

    fn sub(self, rhs: usize) -> Self::Output {
        SemaStmtIdx(self.0 - rhs)
    }
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemaStmtIdxRange(ArenaIdxRange<SemaStmtEntry>);

impl SemaStmtIdxRange {
    pub fn iter(self) -> impl Iterator<Item = SemaStmtIdx> {
        self.0.into_iter().map(SemaStmtIdx)
    }

    pub fn start(self) -> SemaStmtIdx {
        SemaStmtIdx(self.0.start())
    }

    pub fn end(self) -> SemaStmtIdx {
        SemaStmtIdx(self.0.end())
    }
}

impl IntoIterator for SemaStmtIdxRange {
    type Item = SemaStmtIdx;

    type IntoIter = std::iter::Map<
        <ArenaIdxRange<stmt::SemaStmtEntry> as IntoIterator>::IntoIter,
        fn(ArenaIdx<SemaStmtEntry>) -> SemaStmtIdx,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(SemaStmtIdx)
    }
}

impl IntoIterator for &SemaStmtIdxRange {
    type Item = SemaStmtIdx;

    type IntoIter = std::iter::Map<
        <ArenaIdxRange<stmt::SemaStmtEntry> as IntoIterator>::IntoIter,
        fn(ArenaIdx<SemaStmtEntry>) -> SemaStmtIdx,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(SemaStmtIdx)
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaStmtMap<V>(ArenaMap<SemaStmtEntry, V>);

impl<V> SemaStmtMap<V> {
    pub fn new(sema_stmt_arena: SemaStmtArenaRef<'_>) -> SemaStmtMap<V> {
        Self(ArenaMap::new2(sema_stmt_arena.0))
    }

    pub fn insert_new(&mut self, stmt_idx: SemaStmtIdx, v: V) {
        self.0.insert_new(stmt_idx.0, v)
    }

    pub fn get(&self, sema_stmt_idx: SemaStmtIdx) -> Option<&V> {
        self.0.get(sema_stmt_idx.0)
    }
}

impl<V> std::ops::Index<SemaStmtIdx> for SemaStmtMap<V> {
    type Output = V;

    fn index(&self, index: SemaStmtIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_branch<Expectation: ExpectFlyTerm>(
        &mut self,
        stmts: SynStmtIdxRange,
        merger: &mut BranchTypeMerger<Expectation>,
    ) -> SemaStmtIdxRange {
        let (stmts, stmts_ty) =
            self.build_sema_stmts_with_its_ty_returned(stmts, merger.expr_expectation().clone());
        merger.add(self, stmts_ty);
        stmts
    }

    pub(crate) fn build_sema_stmts(
        &mut self,
        stmts: SynStmtIdxRange,
        block_ty_expectation: impl ExpectFlyTerm,
    ) -> SemaStmtIdxRange {
        let mut batch = SemaStmtBatch::default();
        for stmt in stmts.start()..(stmts.end() - 1) {
            batch.add(self.infer_new_nonlast_stmt(stmt));
        }
        batch.add(self.build_sema_stmt(stmts.end() - 1, block_ty_expectation));
        self.alloc_stmt_batch(batch)
    }

    pub(crate) fn build_sema_stmts_with_its_ty_returned(
        &mut self,
        stmts: SynStmtIdxRange,
        block_ty_expectation: impl ExpectFlyTerm,
    ) -> (SemaStmtIdxRange, Option<FlyTerm>) {
        let mut batch = SemaStmtBatch::default();
        for stmt in stmts.start()..(stmts.end() - 1) {
            batch.add(self.infer_new_nonlast_stmt(stmt));
        }
        let (data_result, ty_result) = self.build_sema_stmt(stmts.end() - 1, block_ty_expectation);
        let ty = ty_result.as_ref().ok().copied();
        batch.add((data_result, ty_result));
        (self.alloc_stmt_batch(batch), ty)
    }

    fn infer_new_nonlast_stmt(
        &mut self,
        stmt_idx: SynStmtIdx,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        let expect_unit = self.expect_unit();
        self.build_sema_stmt(stmt_idx, expect_unit)
    }

    fn build_sema_stmt(
        &mut self,
        stmt_idx: SynStmtIdx,
        stmt_ty_expectation: impl ExpectFlyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FlyTerm>,
    ) {
        match self.syn_expr_region_data()[stmt_idx] {
            SynStmtData::Let {
                let_token,
                ref let_variables_pattern,
                ref assign_token,
                initial_value,
            } => self.calc_let_stmt(
                let_token,
                let_variables_pattern,
                assign_token,
                initial_value,
            ),
            SynStmtData::Return {
                return_token,
                result,
            } => {
                let (result, coersion_outcome) = match self.return_ty() {
                    Some(return_ty) => self.build_sema_expr_with_outcome(
                        result,
                        ExpectCoersion::new_move(return_ty.into()),
                    ),
                    None => (self.build_sema_expr(result, ExpectAnyDerived), None),
                };
                (
                    Ok(SemaStmtData::Return {
                        return_token,
                        result,
                        coersion_outcome,
                    }),
                    Ok(self.term_menu().never().into()),
                )
            }
            SynStmtData::Require {
                require_token,
                condition,
            } => {
                let condition = self.build_sema_condition(condition);
                (
                    Ok(SemaStmtData::Require {
                        require_token,
                        condition,
                    }),
                    Ok(self.term_menu().unit_ty_ontology().into()),
                )
            }
            SynStmtData::Assert {
                assert_token,
                condition,
            } => {
                let condition = self.build_sema_condition(condition);
                (
                    Ok(SemaStmtData::Assert {
                        assert_token,
                        condition,
                    }),
                    Ok(self.term_menu().unit_ty_ontology().into()),
                )
            }
            SynStmtData::Break { break_token } => (
                Ok(SemaStmtData::Break { break_token }),
                Ok(self.term_menu().never().into()),
            ),
            SynStmtData::Eval {
                expr_idx,
                eol_semicolon,
            } => {
                let (sema_expr_idx, ty, outcome) = match eol_semicolon {
                    Ok(None) => {
                        self.build_sema_expr_with_ty_and_outcome(expr_idx, stmt_ty_expectation)
                    }
                    Ok(Some(_)) => {
                        let (sema_expr_idx, expr_ty, outcome) =
                            self.build_sema_expr_with_ty_and_outcome(expr_idx, ExpectAnyOriginal);
                        let ty_result = match expr_ty {
                            Some(ty) => match ty.base_resolved(self) {
                                FlyTermBase::Eth(ty) if ty == self.term_menu().never() => {
                                    Some(self.term_menu().never().into())
                                }
                                _ => Some(self.term_menu().unit_ty_ontology().into()),
                            },
                            None => None,
                        };
                        (sema_expr_idx, ty_result, outcome)
                    }
                    Err(_) => self.build_sema_expr_with_ty_and_outcome(expr_idx, ExpectAnyDerived),
                };
                (
                    Ok(SemaStmtData::Eval {
                        sema_expr_idx,
                        eol_semicolon,
                        outcome,
                    }),
                    ty.ok_or(DerivedSemaExprTypeError::EvalExprTypeNotInferred.into()),
                )
            }
            SynStmtData::ForBetween {
                for_token,
                ref particulars,
                for_loop_var_symbol_idx,
                ref block,
                ref eol_colon,
            } => {
                let expr_expectation = self.expect_unit();
                let Ok(particulars) =
                    self.build_sema_for_between_particulars(particulars, for_loop_var_symbol_idx)
                else {
                    todo!()
                };
                let block = self.build_sema_stmts(*block, expr_expectation);
                let &Ok(eol_colon) = eol_colon else { todo!() };
                (
                    Ok(SemaStmtData::ForBetween {
                        for_token,
                        particulars,
                        for_loop_var_symbol_idx,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu().unit_ty_ontology().into()),
                )
            }
            SynStmtData::ForIn {
                ref condition,
                block,
                ..
            } => todo!(),
            SynStmtData::ForExt {
                forext_token,
                ref particulars,
                block,
                ref eol_colon,
            } => {
                let (forext_loop_var_sema_expr_idx, forext_loop_var_ty) = self
                    .build_sema_expr_with_ty(
                        particulars.forext_loop_var_expr_idx,
                        ExpectAnyOriginal,
                    );
                let Some(forext_loop_var_ty) = forext_loop_var_ty else {
                    todo!()
                };
                let particulars = self.build_sema_forext_particulars(
                    particulars,
                    forext_loop_var_sema_expr_idx,
                    forext_loop_var_ty,
                );
                let block_ty_expectation = self.expect_unit();
                let block = self.build_sema_stmts(block, block_ty_expectation);
                let &Ok(eol_colon) = eol_colon else { todo!() };
                (
                    Ok(SemaStmtData::Forext {
                        forext_token,
                        particulars,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu().unit_ty_ontology().into()),
                )
            }
            SynStmtData::While {
                while_token,
                ref condition,
                block,
                ref eol_colon,
            } => {
                let condition = match condition {
                    Ok(condition) => self.build_sema_condition(*condition),
                    Err(_) => todo!(),
                };
                let &Ok(eol_colon) = eol_colon else { todo!() };
                let expect_unit = self.expect_unit();
                let block = self.build_sema_stmts(block, expect_unit);
                (
                    Ok(SemaStmtData::While {
                        while_token,
                        condition,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu().unit_ty_ontology().into()),
                )
            }
            SynStmtData::DoWhile {
                do_token,
                while_token,
                ref condition,
                block,
                ref eol_colon,
            } => {
                let condition = match condition {
                    Ok(condition) => self.build_sema_condition(*condition),
                    Err(_) => todo!(),
                };
                let &Ok(eol_colon) = eol_colon else { todo!() };
                let expect_unit = self.expect_unit();
                let block = self.build_sema_stmts(block, expect_unit);
                (
                    Ok(SemaStmtData::DoWhile {
                        do_token,
                        while_token,
                        condition,
                        eol_colon,
                        block,
                    }),
                    Ok(self.term_menu().unit_ty_ontology().into()),
                )
            }
            SynStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => self.build_if_else_stmt(
                if_branch,
                elif_branches,
                else_branch.as_ref(),
                stmt_ty_expectation,
            ),
            SynStmtData::Match {
                match_token,
                ref match_expr,
                ref eol_with_token,
                ref case_branches,
            } => self.build_match_stmt(
                match_token,
                match_expr,
                eol_with_token,
                case_branches,
                stmt_ty_expectation,
            ),
        }
    }

    pub(crate) fn build_sema_condition(&mut self, syn_expr_idx: SynExprIdx) -> SemaCondition {
        let (sema_expr_idx, outcome) =
            self.build_sema_expr_with_outcome(syn_expr_idx, ExpectConditionType);
        match *sema_expr_idx.data(self.sema_expr_arena().arena_ref()) {
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => SemaCondition::Be {
                src,
                be_regional_token_idx,
                target,
            },
            _ => SemaCondition::Other {
                sema_expr_idx,
                conversion: match outcome {
                    Some(ExpectConditionTypeOutcome { conversion }) => conversion,
                    None => todo!(),
                },
            },
        }
    }
}
