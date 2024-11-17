use crate::{
    ast::{
        math::LxMathAstIdx, parse_latex_input_into_asts, root::LxRootAstIdxRange,
        rose::LxRoseAstIdxRange, LxAstArena, LxAstIdxRange,
    },
    helpers::show::display_tree::LxAstDisplayTreeBuilder,
    parser::LxAstParser,
    range::{calc_ast_token_idx_range_map, LxAstTokenIdxRangeMap},
};
use husky_tree_utils::display::DisplayTree;
use latex_command::signature::table::LxCommandSignatureTable;
use latex_environment::signature::table::LxEnvironmentSignatureTable;
use latex_prelude::{
    helper::tracker::{
        IsLxTrackerInput, LxDocumentBodyTrackerInput, LxDocumentTrackerInput, LxFormulaTrackerInput,
    },
    mode::LxMode,
};
use latex_token::storage::LxTokenStorage;

#[derive(Debug)]
pub struct LxAstTracker<Input: IsLxAstTrackerInput> {
    pub command_signature_table: LxCommandSignatureTable,
    pub input: String,
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
    pub output: Input::Output,
}

pub trait IsLxAstTrackerInput: IsLxTrackerInput {
    type Output;
}

impl<Input: IsLxAstTrackerInput> LxAstTracker<Input> {
    pub fn new_aux(
        input: &str,
        root_mode: LxMode,
        db: &salsa::Db,
        f: impl FnOnce(LxAstParser) -> Input::Output,
    ) -> Self {
        let mut ast_arena = LxAstArena::default();
        let mut token_storage = LxTokenStorage::default();
        let command_signature_table = LxCommandSignatureTable::new_default(db);
        let environment_signature_table = LxEnvironmentSignatureTable::new_default(db);
        let ast_token_idx_range_map = calc_ast_token_idx_range_map(db, &ast_arena);
        let mut parser = LxAstParser::new(
            db,
            &command_signature_table,
            &environment_signature_table,
            input,
            root_mode,
            &mut token_storage,
            &mut ast_arena,
        );
        let output = f(parser);
        Self {
            command_signature_table,
            input: input.to_string(),
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            output,
        }
    }
}

impl<'a, Input: IsLxAstTrackerInput> LxAstTracker<Input> {
    pub fn display_tree_builder(&'a self, db: &'a ::salsa::Db) -> LxAstDisplayTreeBuilder<'a> {
        LxAstDisplayTreeBuilder::new(
            db,
            &self.input,
            &self.token_storage,
            self.ast_arena.as_arena_ref(),
            &self.ast_token_idx_range_map,
        )
    }
}

impl IsLxAstTrackerInput for LxDocumentTrackerInput {
    type Output = LxRootAstIdxRange;
}

impl LxAstTracker<LxDocumentTrackerInput> {
    pub fn show(&self, db: &salsa::Db) -> String {
        let display_tree_builder = self.display_tree_builder(db);
        format!(
            "{}",
            DisplayTree::show_trees(
                &display_tree_builder.render_root_asts(self.output),
                &Default::default(),
            )
        )
    }
}

impl IsLxAstTrackerInput for LxDocumentBodyTrackerInput {
    type Output = LxRoseAstIdxRange;
}

impl LxAstTracker<LxDocumentBodyTrackerInput> {
    pub fn show(&self, db: &salsa::Db) -> String {
        let display_tree_builder = self.display_tree_builder(db);
        format!(
            "{}",
            DisplayTree::show_trees(
                &display_tree_builder.render_rose_asts(self.output),
                &Default::default(),
            )
        )
    }
}

impl IsLxAstTrackerInput for LxFormulaTrackerInput {
    type Output = LxMathAstIdx;
}

impl LxAstTracker<LxFormulaTrackerInput> {
    pub fn show(&self, db: &salsa::Db) -> String {
        let display_tree_builder = self.display_tree_builder(db);
        format!(
            "{}",
            display_tree_builder
                .render_math_ast(self.output)
                .show(&Default::default())
        )
    }
}
