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
    pub expr_range_map: VdSynExprRangeMap,
    pub phrase_arena: VdSynPhraseArena,
    pub phrase_range_map: VdSynPhraseRangeMap,
    pub clause_arena: VdSynClauseArena,
    pub clause_range_map: VdSynClauseRangeMap,
    pub sentence_arena: VdSynSentenceArena,
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
        let mut builder = VdSynExprBuilder::new(db, &token_storage, &ast_arena, &annotations);
        let result = asts.to_vd_syn(&mut builder);
        let (
            expr_arena,
            expr_range_map,
            phrase_arena,
            phrase_range_map,
            clause_arena,
            clause_range_map,
            sentence_arena,
            sentence_range_map,
        ) = builder.finish();
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
            expr_range_map,
            phrase_arena,
            phrase_range_map,
            clause_arena,
            clause_range_map,
            sentence_arena,
            sentence_range_map,
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
            &self.expr_range_map,
            self.phrase_arena.as_arena_ref(),
            &self.phrase_range_map,
            self.clause_arena.as_arena_ref(),
            &self.clause_range_map,
            self.sentence_arena.as_arena_ref(),
            &self.sentence_range_map,
        );
        match self.result {
            Left(expr) => builder.render_expr(expr).show(&Default::default()),
            Right(_) => todo!(),
        }
    }
}
