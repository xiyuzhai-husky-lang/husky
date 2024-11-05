use crate::{
    clause::{VdSynClauseArena, VdSynClauseArenaRef, VdSynClauseIdx, VdSynClauseMap},
    expr::{
        VdSynExprArena, VdSynExprArenaRef, VdSynExprData, VdSynExprIdx, VdSynExprMap,
        VdSynLeftDelimiter, VdSynPrefixOpr, VdSynRightDelimiter, VdSynSeparator,
    },
    phrase::{VdSynPhraseArena, VdSynPhraseArenaRef, VdSynPhraseIdx, VdSynPhraseMap},
    sentence::{VdSynSentenceArena, VdSynSentenceArenaRef, VdSynSentenceIdx, VdSynSentenceMap},
    stmt::{VdSynStmtArena, VdSynStmtArenaRef, VdSynStmtIdx, VdSynStmtMap},
};
use either::*;
use latex_token::idx::LxTokenIdxRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynExprTokenIdxRange {
    Standard(LxTokenIdxRange),
}
impl VdSynExprTokenIdxRange {
    fn join(self, other: VdSynExprTokenIdxRange) -> Self {
        match (self, other) {
            (VdSynExprTokenIdxRange::Standard(slf), VdSynExprTokenIdxRange::Standard(other)) => {
                VdSynExprTokenIdxRange::Standard(slf.join(other))
            }
        }
    }
}

pub type VdSynPhraseTokenIdxRange = LxTokenIdxRange;
pub type VdSynClauseTokenIdxRange = LxTokenIdxRange;
pub type VdSynSentenceTokenIdxRange = LxTokenIdxRange;
pub type VdSynStmtTokenIdxRange = LxTokenIdxRange;

pub type VdSynExprTokenIdxRangeMap = VdSynExprMap<VdSynExprTokenIdxRange>;
pub type VdSynPhraseTokenIdxRangeMap = VdSynPhraseMap<VdSynPhraseTokenIdxRange>;
pub type VdSynClauseTokenIdxRangeMap = VdSynClauseMap<VdSynClauseTokenIdxRange>;
pub type VdSynSentenceTokenIdxRangeMap = VdSynSentenceMap<VdSynSentenceTokenIdxRange>;
pub type VdSynStmtTokenIdxRangeMap = VdSynStmtMap<VdSynStmtTokenIdxRange>;

pub fn calc_expr_range_map(
    db: &::salsa::Db,
    expr_arena: &VdSynExprArena,
    phrase_arena: &VdSynPhraseArena,
    clause_arena: &VdSynClauseArena,
    sentence_arena: &VdSynSentenceArena,
    stmt_arena: &VdSynStmtArena,
) -> (
    VdSynExprTokenIdxRangeMap,
    VdSynPhraseTokenIdxRangeMap,
    VdSynClauseTokenIdxRangeMap,
    VdSynSentenceTokenIdxRangeMap,
    VdSynStmtTokenIdxRangeMap,
) {
    let mut calculator = VdSynExprRangeCalculator::new(
        db,
        expr_arena,
        phrase_arena,
        clause_arena,
        sentence_arena,
        stmt_arena,
    );
    calculator.infer_all_ranges();
    calculator.finish()
}

struct VdSynExprRangeCalculator<'db> {
    db: &'db ::salsa::Db,
    expr_arena: VdSynExprArenaRef<'db>,
    phrase_arena: VdSynPhraseArenaRef<'db>,
    clause_arena: VdSynClauseArenaRef<'db>,
    sentence_arena: VdSynSentenceArenaRef<'db>,
    stmt_arena: VdSynStmtArenaRef<'db>,
    expr_range_map: VdSynExprTokenIdxRangeMap,
    phrase_range_map: VdSynPhraseTokenIdxRangeMap,
    clause_range_map: VdSynClauseTokenIdxRangeMap,
    sentence_range_map: VdSynSentenceTokenIdxRangeMap,
    stmt_range_map: VdSynStmtTokenIdxRangeMap,
}

impl<'db> VdSynExprRangeCalculator<'db> {
    fn new(
        db: &'db ::salsa::Db,
        expr_arena: &'db VdSynExprArena,
        phrase_arena: &'db VdSynPhraseArena,
        clause_arena: &'db VdSynClauseArena,
        sentence_arena: &'db VdSynSentenceArena,
        stmt_arena: &'db VdSynStmtArena,
    ) -> Self {
        Self {
            db,
            expr_arena: expr_arena.as_arena_ref(),
            phrase_arena: phrase_arena.as_arena_ref(),
            clause_arena: clause_arena.as_arena_ref(),
            sentence_arena: sentence_arena.as_arena_ref(),
            stmt_arena: stmt_arena.as_arena_ref(),
            expr_range_map: VdSynExprTokenIdxRangeMap::new(expr_arena),
            phrase_range_map: VdSynPhraseTokenIdxRangeMap::new(phrase_arena),
            clause_range_map: VdSynClauseTokenIdxRangeMap::new(clause_arena),
            sentence_range_map: VdSynSentenceTokenIdxRangeMap::new(sentence_arena),
            stmt_range_map: VdSynStmtTokenIdxRangeMap::new(stmt_arena),
        }
    }
}

impl<'db> VdSynExprRangeCalculator<'db> {
    fn infer_all_ranges(&mut self) {
        for expr in self.expr_arena.index_iter() {
            self.infer_expr(expr);
        }
        for phrase in self.phrase_arena.index_iter() {
            self.infer_phrase(phrase);
        }
        for clause in self.clause_arena.index_iter() {
            self.infer_clause(clause);
        }
        for sentence in self.sentence_arena.index_iter() {
            self.infer_sentence(sentence);
        }
        for stmt in self.stmt_arena.index_iter() {
            self.infer_stmt(stmt);
        }
    }

    fn infer_expr(&mut self, expr: VdSynExprIdx) {
        if self.expr_range_map.has(expr) {
            return;
        }
        let range = self.calc_expr(expr);
        self.expr_range_map.insert(expr, range);
    }

    fn calc_expr(&mut self, expr: VdSynExprIdx) -> VdSynExprTokenIdxRange {
        let expr_arena = self.expr_arena;
        match expr_arena[expr] {
            VdSynExprData::Literal {
                token_idx_range, ..
            } => VdSynExprTokenIdxRange::Standard(token_idx_range),
            VdSynExprData::Letter {
                token_idx_range, ..
            } => VdSynExprTokenIdxRange::Standard(token_idx_range),
            VdSynExprData::Notation => todo!(),
            VdSynExprData::BaseOpr { opr } => todo!(),
            VdSynExprData::Binary { lopd, ropd, .. } => {
                let lopd_range = self.get_expr(lopd);
                let ropd_range = self.get_expr(ropd);
                lopd_range.join(ropd_range)
            }
            VdSynExprData::Prefix { opr, opd } => {
                let opd_range = self.get_expr(opd);
                let opr_range = match opr {
                    VdSynPrefixOpr::Base(lx_token_idx_range, _) => {
                        VdSynExprTokenIdxRange::Standard(lx_token_idx_range)
                    }
                    VdSynPrefixOpr::Composite(expr, _) => self.get_expr(expr),
                };
                opr_range.join(opd_range)
            }
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::Attach { base, ref scripts } => {
                let mut range = self.get_expr(base);
                for &(_, script) in scripts {
                    range = range.join(self.get_expr(script));
                }
                range
            }
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(ref e) => VdSynExprTokenIdxRange::Standard(e.token_idx_range()),
            VdSynExprData::SeparatedList { ref fragments, .. } => {
                // use the first and the last fragment's range
                let mut t = |fragment: Either<VdSynExprIdx, VdSynSeparator>| match fragment {
                    Left(expr) | Right(VdSynSeparator::Composite(expr, _)) => self.get_expr(expr),
                    Right(VdSynSeparator::Base(token_idx_range, _)) => {
                        VdSynExprTokenIdxRange::Standard(token_idx_range)
                    }
                };
                let first = *fragments.first().expect("fragments are always non-empty");
                let last = *fragments.last().expect("fragments are always non-empty");
                let first_range = t(first);
                let last_range = t(last);
                first_range.join(last_range)
            }
            VdSynExprData::LxDelimited {
                left_delimiter_token_idx,
                right_delimiter_token_idx,
                ..
            } => VdSynExprTokenIdxRange::Standard(LxTokenIdxRange::new_closed(
                *left_delimiter_token_idx,
                *right_delimiter_token_idx,
            )),
            VdSynExprData::Delimited {
                left_delimiter,

                right_delimiter,
                ..
            } => {
                let left_delimiter_range = match left_delimiter {
                    VdSynLeftDelimiter::Base(token_idx_range, _) => {
                        VdSynExprTokenIdxRange::Standard(token_idx_range)
                    }
                    VdSynLeftDelimiter::Composite(expr, _) => self.get_expr(expr),
                };
                let right_delimiter_range = match right_delimiter {
                    VdSynRightDelimiter::Base(token_idx_range, _) => {
                        VdSynExprTokenIdxRange::Standard(token_idx_range)
                    }
                    VdSynRightDelimiter::Composite(expr, _) => self.get_expr(expr),
                };
                left_delimiter_range.join(right_delimiter_range)
            }
            VdSynExprData::Fraction {
                command_token_idx,
                denominator_rcurl_token_idx,
                ..
            } => VdSynExprTokenIdxRange::Standard(LxTokenIdxRange::new_closed(
                *command_token_idx,
                *denominator_rcurl_token_idx,
            )),
            VdSynExprData::Sqrt {
                command_token_idx,
                radicand_rcurl_token_idx,
                ..
            } => VdSynExprTokenIdxRange::Standard(LxTokenIdxRange::new_closed(
                *command_token_idx,
                *radicand_rcurl_token_idx,
            )),
        }
    }

    fn get_expr(&mut self, expr: VdSynExprIdx) -> VdSynExprTokenIdxRange {
        self.infer_expr(expr);
        self.expr_range_map[expr]
    }

    fn infer_phrase(&mut self, phrase: VdSynPhraseIdx) {
        todo!()
    }

    fn calc_phrase(&mut self, phrase: VdSynPhraseIdx) -> VdSynPhraseTokenIdxRange {
        todo!()
    }

    fn get_phrase(&mut self, phrase: VdSynPhraseIdx) -> VdSynPhraseTokenIdxRange {
        self.infer_phrase(phrase);
        self.phrase_range_map[phrase]
    }

    fn infer_clause(&mut self, clause: VdSynClauseIdx) {
        todo!()
    }

    fn calc_clause(&mut self, clause: VdSynClauseIdx) -> VdSynClauseTokenIdxRange {
        todo!()
    }

    fn get_clause(&mut self, clause: VdSynClauseIdx) -> VdSynClauseTokenIdxRange {
        self.infer_clause(clause);
        self.clause_range_map[clause]
    }

    fn infer_sentence(&mut self, sentence: VdSynSentenceIdx) {
        todo!()
    }

    fn calc_sentence(&mut self, sentence: VdSynSentenceIdx) -> VdSynSentenceTokenIdxRange {
        todo!()
    }

    fn get_sentence(&mut self, sentence: VdSynSentenceIdx) -> VdSynSentenceTokenIdxRange {
        self.infer_sentence(sentence);
        self.sentence_range_map[sentence]
    }

    fn infer_stmt(&mut self, stmt: VdSynStmtIdx) {
        todo!()
    }

    fn calc_stmt(&mut self, stmt: VdSynStmtIdx) -> VdSynStmtTokenIdxRange {
        todo!()
    }

    fn get_stmt(&mut self, stmt: VdSynStmtIdx) -> VdSynStmtTokenIdxRange {
        self.infer_stmt(stmt);
        self.stmt_range_map[stmt]
    }

    fn finish(
        self,
    ) -> (
        VdSynExprTokenIdxRangeMap,
        VdSynPhraseTokenIdxRangeMap,
        VdSynClauseTokenIdxRangeMap,
        VdSynSentenceTokenIdxRangeMap,
        VdSynStmtTokenIdxRangeMap,
    ) {
        (
            self.expr_range_map,
            self.phrase_range_map,
            self.clause_range_map,
            self.sentence_range_map,
            self.stmt_range_map,
        )
    }
}
