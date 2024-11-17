use crate::{
    ast::{
        lisp::LxLispAstIdxRange,
        math::{LxMathAstIdx, LxMathAstIdxRange},
        parse_latex_input_into_asts,
        root::LxRootAstIdxRange,
        rose::LxRoseAstIdxRange,
        LxAstArena, LxAstIdxRange,
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
        IsLxInput, LxDocumentBodyTrackerInput, LxDocumentTrackerInput, LxFormulaTrackerInput,
        LxLispTrackerInput,
    },
    mode::LxMode,
};
use latex_token::storage::LxTokenStorage;

#[derive(Debug)]
pub struct LxAstTracker<'a, Input: IsLxAstInput<'a>> {
    pub command_signature_table: LxCommandSignatureTable,
    pub input: Input,
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
    pub output: Input::LxAstOutput,
}

pub trait IsLxAstInput<'a>: IsLxInput<'a> {
    type LxAstOutput: std::fmt::Debug;

    fn parse(parser: LxAstParser) -> Self::LxAstOutput;
}

impl<'a, Input: IsLxAstInput<'a>> LxAstTracker<'a, Input> {
    pub fn new(input: Input, db: &salsa::Db) -> Self {
        let mut ast_arena = LxAstArena::default();
        let mut token_storage = LxTokenStorage::default();
        let command_signature_table = LxCommandSignatureTable::new_default(db);
        let environment_signature_table = LxEnvironmentSignatureTable::new_default(db);
        let mut parser = LxAstParser::new(
            db,
            &command_signature_table,
            &environment_signature_table,
            input.input(),
            input.root_mode(),
            &mut token_storage,
            &mut ast_arena,
        );
        let output = Input::parse(parser);
        let ast_token_idx_range_map = calc_ast_token_idx_range_map(db, &ast_arena);
        Self {
            command_signature_table,
            input,
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            output,
        }
    }
}

impl<'a, Input: IsLxAstInput<'a>> LxAstTracker<'a, Input> {
    pub fn display_tree_builder<'b>(&'b self, db: &'b salsa::Db) -> LxAstDisplayTreeBuilder<'b> {
        LxAstDisplayTreeBuilder::new(
            db,
            self.input.input(),
            &self.token_storage,
            self.ast_arena.as_arena_ref(),
            &self.ast_token_idx_range_map,
        )
    }
}

impl<'a> IsLxAstInput<'a> for LxDocumentTrackerInput<'a> {
    type LxAstOutput = LxRootAstIdxRange;

    fn parse(mut parser: LxAstParser) -> Self::LxAstOutput {
        parser.parse_root_asts()
    }
}

impl<'a> LxAstTracker<'a, LxDocumentTrackerInput<'a>> {
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

impl<'a> IsLxAstInput<'a> for LxDocumentBodyTrackerInput<'a> {
    type LxAstOutput = LxRoseAstIdxRange;

    fn parse(mut parser: LxAstParser) -> Self::LxAstOutput {
        parser.parse_rose_asts()
    }
}

impl<'a> LxAstTracker<'a, LxDocumentBodyTrackerInput<'a>> {
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

impl<'a> IsLxAstInput<'a> for LxFormulaTrackerInput<'a> {
    type LxAstOutput = LxMathAstIdxRange;

    fn parse(mut parser: LxAstParser) -> Self::LxAstOutput {
        parser.parse_math_asts()
    }
}

impl<'a> LxAstTracker<'a, LxFormulaTrackerInput<'a>> {
    pub fn show(&self, db: &salsa::Db) -> String {
        let display_tree_builder = self.display_tree_builder(db);
        format!(
            "{}",
            DisplayTree::show_trees(
                &display_tree_builder.render_math_asts(self.output),
                &Default::default(),
            )
        )
    }
}

impl<'a> IsLxAstInput<'a> for LxLispTrackerInput<'a> {
    type LxAstOutput = LxLispAstIdxRange;

    fn parse(mut parser: LxAstParser) -> Self::LxAstOutput {
        parser.parse_lisp_asts()
    }
}

impl<'a> LxAstTracker<'a, LxLispTrackerInput<'a>> {
    pub fn show(&self, db: &salsa::Db) -> String {
        let display_tree_builder = self.display_tree_builder(db);
        format!(
            "{}",
            DisplayTree::show_trees(
                &display_tree_builder.render_lisp_asts(self.output),
                &Default::default(),
            )
        )
    }
}
