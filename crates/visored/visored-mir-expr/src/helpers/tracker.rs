use crate::elaborator::IsVdMirTacticElaborator;
use crate::{
    builder::VdMirExprBuilder,
    expr::{VdMirExprArena, VdMirExprIdx},
    stmt::{VdMirStmtArena, VdMirStmtIdxRange},
    *,
};
use either::*;
use eterned::db::EternerDb;
use expr::{application::VdMirFunc, VdMirExprData};
use helpers::show::display_tree::VdMirExprDisplayTreeBuilder;
use hint::VdMirHintArena;
use husky_tree_utils::display::DisplayTree;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxFormulaInput, LxPageInput},
    mode::LxMode,
};
use latex_token::storage::LxTokenStorage;
use region::{VdMirExprRegionDataMut, VdMirExprRegionDataRef};
use source_map::VdMirSourceMap;
use symbol::local_defn::storage::VdMirSymbolLocalDefnStorage;
use tactic::VdMirTacticArena;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};
use visored_entity_path::module::VdModulePath;
use visored_sem_expr::{
    helpers::tracker::{IsVdSemExprInput, VdSemExprTracker},
    range::{
        VdSemBlockTokenIdxRangeMap, VdSemClauseTokenIdxRangeMap, VdSemDivisionTokenIdxRangeMap,
        VdSemExprTokenIdxRangeMap, VdSemPhraseTokenIdxRangeMap, VdSemSentenceTokenIdxRangeMap,
    },
};
use visored_syn_expr::vibe::VdSynExprVibe;

pub struct VdMirExprTracker<'a, Input: IsVdMirExprInput<'a>, Elaborator: IsVdMirTacticElaborator> {
    pub input: Input,
    pub root_module_path: VdModulePath,
    pub expr_arena: VdMirExprArena,
    pub stmt_arena: VdMirStmtArena,
    pub hint_arena: VdMirHintArena,
    pub tactic_arena: VdMirTacticArena,
    pub symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    pub source_map: VdMirSourceMap,
    pub sem_expr_range_map: VdSemExprTokenIdxRangeMap,
    pub sem_phrase_range_map: VdSemPhraseTokenIdxRangeMap,
    pub sem_clause_range_map: VdSemClauseTokenIdxRangeMap,
    pub sem_sentence_range_map: VdSemSentenceTokenIdxRangeMap,
    pub sem_stmt_range_map: VdSemBlockTokenIdxRangeMap,
    pub sem_division_range_map: VdSemDivisionTokenIdxRangeMap,
    pub token_storage: LxTokenStorage,
    pub output: Input::VdMirExprOutput,
    pub elaborator: Elaborator,
}

pub trait IsVdMirExprInput<'a>: IsVdSemExprInput<'a> {
    type VdMirExprOutput: IsVdMirExprOutput + FromToVdMir<Self::VdSemExprOutput>;
}

pub trait IsVdMirExprOutput: std::fmt::Debug + Copy {
    fn show(self, builder: &VdMirExprDisplayTreeBuilder) -> String;

    fn eval_all_tactics_within_self(
        self,
        region_data: VdMirExprRegionDataRef,
        elaborator: &mut impl IsVdMirTacticElaborator,
    );
}

pub trait FromToVdMir<S> {
    fn from_to_vd_mir(output: S, builder: &mut VdMirExprBuilder) -> Self;
}

impl<'a, S, T: IsVdMirExprOutput> FromToVdMir<S> for T
where
    S: ToVdMir<T>,
{
    fn from_to_vd_mir(s: S, builder: &mut VdMirExprBuilder) -> Self {
        s.to_vd_mir(builder)
    }
}

impl<'a, Input, Elaborator> VdMirExprTracker<'a, Input, Elaborator>
where
    Input: IsVdMirExprInput<'a>,
    Elaborator: IsVdMirTacticElaborator,
{
    pub fn new(
        input: Input,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        models: &VdModels,
        vibe: VdSynExprVibe,
        db: &EternerDb,
        gen_elaborator: impl FnOnce(VdMirExprRegionDataRef) -> Elaborator,
    ) -> Self {
        let VdSemExprTracker {
            root_module_path,
            input,
            annotations,
            default_resolution_table,
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            expr_arena: sem_expr_arena,
            phrase_arena: sem_phrase_arena,
            clause_arena: sem_clause_arena,
            sentence_arena: sem_sentence_arena,
            stmt_arena: sem_stmt_arena,
            division_arena: sem_division_arena,
            expr_range_map: sem_expr_range_map,
            phrase_range_map: sem_phrase_range_map,
            clause_range_map: sem_clause_range_map,
            sentence_range_map: sem_sentence_range_map,
            stmt_range_map: sem_stmt_range_map,
            division_range_map: sem_division_range_map,
            symbol_local_defn_storage: sem_symbol_local_defn_storage,
            output,
        } = VdSemExprTracker::new(
            input,
            token_annotations,
            space_annotations,
            models,
            vibe,
            db,
        );
        let mut builder = VdMirExprBuilder::new(
            sem_expr_arena.as_arena_ref(),
            sem_phrase_arena.as_arena_ref(),
            sem_clause_arena.as_arena_ref(),
            sem_sentence_arena.as_arena_ref(),
            sem_stmt_arena.as_arena_ref(),
            sem_division_arena.as_arena_ref(),
            &sem_symbol_local_defn_storage,
        );
        let output: Input::VdMirExprOutput = FromToVdMir::from_to_vd_mir(output, &mut builder);
        let (
            mut expr_arena,
            mut stmt_arena,
            mut hint_arena,
            mut tactic_arena,
            symbol_local_defn_storage,
            source_map,
        ) = builder.finish();
        let mut elaborator = gen_elaborator(VdMirExprRegionDataRef {
            expr_arena: expr_arena.as_arena_ref(),
            stmt_arena: stmt_arena.as_arena_ref(),
            hint_arena: hint_arena.as_arena_ref(),
            symbol_local_defn_storage: &symbol_local_defn_storage,
        });
        output.eval_all_tactics_within_self(
            VdMirExprRegionDataRef {
                expr_arena: expr_arena.as_arena_ref(),
                stmt_arena: stmt_arena.as_arena_ref(),
                hint_arena: hint_arena.as_arena_ref(),
                symbol_local_defn_storage: &symbol_local_defn_storage,
            },
            &mut elaborator,
        );
        elaborator.extract(VdMirExprRegionDataMut {
            expr_arena: &mut expr_arena,
            stmt_arena: &mut stmt_arena,
            hint_arena: &mut hint_arena,
            tactic_arena: &mut tactic_arena,
            symbol_local_defn_storage: &symbol_local_defn_storage,
        });
        Self {
            input,
            root_module_path,
            expr_arena,
            stmt_arena,
            hint_arena,
            tactic_arena,
            symbol_local_defn_storage,
            source_map,
            sem_expr_range_map,
            sem_phrase_range_map,
            sem_clause_range_map,
            sem_sentence_range_map,
            sem_stmt_range_map,
            sem_division_range_map,
            token_storage,
            output,
            elaborator,
        }
    }

    pub(crate) fn show_display_tree(&self, db: &EternerDb) -> String {
        let builder = self.display_tree_builder(db);
        self.output.show(&builder)
    }

    fn display_tree_builder<'b>(&'b self, db: &'b EternerDb) -> VdMirExprDisplayTreeBuilder<'b> {
        VdMirExprDisplayTreeBuilder::new(
            db,
            self.expr_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
        )
    }
}

impl<'a> IsVdMirExprInput<'a> for LxDocumentInput<'a> {
    type VdMirExprOutput = VdMirStmtIdxRange;
}

impl<'a> IsVdMirExprInput<'a> for LxDocumentBodyInput<'a> {
    type VdMirExprOutput = VdMirStmtIdxRange;
}

impl<'a> IsVdMirExprInput<'a> for LxPageInput<'a> {
    type VdMirExprOutput = VdMirStmtIdxRange;
}

impl<'a> IsVdMirExprInput<'a> for LxFormulaInput<'a> {
    type VdMirExprOutput = VdMirExprIdx;
}

impl IsVdMirExprOutput for VdMirStmtIdxRange {
    fn show(self, builder: &VdMirExprDisplayTreeBuilder) -> String {
        DisplayTree::show_trees(&builder.render_stmts(self), &Default::default())
    }

    fn eval_all_tactics_within_self(
        self,
        region_data: VdMirExprRegionDataRef,
        elaborator: &mut impl IsVdMirTacticElaborator,
    ) {
        elaborator.elaborate_stmts(self, region_data);
    }
}

impl IsVdMirExprOutput for VdMirExprIdx {
    fn show(self, builder: &VdMirExprDisplayTreeBuilder) -> String {
        builder.render_expr(self).show(&Default::default())
    }

    fn eval_all_tactics_within_self(
        self,
        region_data: VdMirExprRegionDataRef,
        elaborator: &mut impl IsVdMirTacticElaborator,
    ) {
    }
}
