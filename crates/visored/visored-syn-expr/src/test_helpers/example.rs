use super::*;
use crate::{
    builder::VdSynExprBuilder,
    clause::VdSynClauseArena,
    expr::VdSynExprArena,
    phrase::VdSynPhraseArena,
    range::{VdSynClauseRangeMap, VdSynExprRangeMap, VdSynPhraseRangeMap, VdSynSentenceRangeMap},
    sentence::VdSynSentenceArena,
};
use expr::VdSynExprIdx;
use helpers::show::display_tree::VdSynExprDisplayTreeBuilder;
use latex_ast::{
    ast::{parse_latex_input_into_asts, LxAstArena, LxAstIdxRange},
    range::{calc_ast_token_idx_range_map, LxAstTokenIdxRangeMap},
    test_helpers::example::LxAstExample,
};
use latex_prelude::mode::LxMode;
use latex_token::storage::LxTokenStorage;
use visored_annotation::{
    annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation},
    annotations::VdAnnotations,
};

pub struct VdSynExprExample {
    pub input: String,
    pub root_mode: LxMode,
    pub annotations: VdAnnotations,
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub asts: LxAstIdxRange,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
    pub result: Either<VdSynExprIdx, ()>,
    pub expr_arena: VdSynExprArena,
    pub phrase_arena: VdSynPhraseArena,
    pub clause_arena: VdSynClauseArena,
    pub sentence_arena: VdSynSentenceArena,
    pub expr_range_map: VdSynExprRangeMap,
    pub phrase_range_map: VdSynPhraseRangeMap,
    pub clause_range_map: VdSynClauseRangeMap,
    pub sentence_range_map: VdSynSentenceRangeMap,
}

impl VdSynExprExample {
    pub fn new(
        input: &str,
        root_mode: LxMode,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        db: &salsa::Db,
    ) -> Self {
        let mut ast_arena = LxAstArena::default();
        let mut token_storage = LxTokenStorage::default();
        let asts =
            parse_latex_input_into_asts(db, input, root_mode, &mut token_storage, &mut ast_arena);
        let ast_token_idx_range_map = calc_ast_token_idx_range_map(db, &ast_arena);
        let annotations = VdAnnotations::from_sparse(
            input,
            token_annotations.iter().copied(),
            space_annotations.iter().copied(),
            &token_storage,
        );
        let mut builder = VdSynExprBuilder::new(
            db,
            &token_storage,
            &ast_arena,
            &ast_token_idx_range_map,
            &annotations,
        );
        let result = asts.to_vd_syn(&mut builder);
        let (expr_arena, phrase_arena, clause_arena, sentence_arena) = builder.finish();
        Self {
            input: input.to_string(),
            root_mode,
            annotations,
            token_storage,
            ast_arena,
            asts,
            ast_token_idx_range_map,
            result,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            expr_range_map: todo!(),
            phrase_range_map: todo!(),
            clause_range_map: todo!(),
            sentence_range_map: todo!(),
        }
    }

    pub(crate) fn show_display_tree(&self, db: &salsa::Db) -> String {
        let builder = VdSynExprDisplayTreeBuilder::new(
            db,
            &self.input,
            &self.token_storage,
            self.ast_arena.as_arena_ref(),
            &self.ast_token_idx_range_map,
            self.expr_arena.as_arena_ref(),
            self.phrase_arena.as_arena_ref(),
            self.clause_arena.as_arena_ref(),
            self.sentence_arena.as_arena_ref(),
            &self.expr_range_map,
            &self.phrase_range_map,
            &self.clause_range_map,
            &self.sentence_range_map,
        );
        match self.result {
            Left(expr) => builder.render_expr(expr).show(&Default::default()),
            Right(_) => todo!(),
        }
    }
}
