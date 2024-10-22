use crate::{expr::VdHirExprArena, stmt::VdHirStmtArena};
use visored_sem_expr::{
    clause::VdSemClauseArenaRef, expr::VdSemExprArenaRef, phrase::VdSemPhraseArenaRef,
    region::VdSemExprRegionData, sentence::VdSemSentenceArenaRef,
};

pub struct VdHirExprBuilder<'db> {
    db: &'db ::salsa::Db,
    vd_sem_expr_arena: VdSemExprArenaRef<'db>,
    vd_sem_phrase_arena: VdSemPhraseArenaRef<'db>,
    vd_sem_clause_arena: VdSemClauseArenaRef<'db>,
    vd_sem_sentence_arena: VdSemSentenceArenaRef<'db>,
    expr_arena: VdHirExprArena,
    stmt_arena: VdHirStmtArena,
}

impl<'db> VdHirExprBuilder<'db> {
    pub fn new(db: &'db ::salsa::Db, vd_sem_expr_region_data: &'db VdSemExprRegionData) -> Self {
        Self {
            db,
            vd_sem_expr_arena: vd_sem_expr_region_data.expr_arena(),
            vd_sem_phrase_arena: vd_sem_expr_region_data.phrase_arena(),
            vd_sem_clause_arena: vd_sem_expr_region_data.clause_arena(),
            vd_sem_sentence_arena: vd_sem_expr_region_data.sentence_arena(),
            expr_arena: VdHirExprArena::default(),
            stmt_arena: VdHirStmtArena::default(),
        }
    }
}
