use super::*;
use crate::{
    builder::VdLeanTranspilationBuilder, dictionary::VdLeanDictionary,
    scheme::IsVdLeanTranspilationScheme, VdTranspileToLean,
};
use either::*;
use eterned::db::EternerDb;
use husky_tree_utils::display::DisplayTree;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxFormulaInput, LxPageInput},
    mode::LxMode,
};
use lean_mir_expr::{
    expr::{LnMirExprArena, LnMirExprIdx},
    helpers::{
        fmt::{LnMirExprFormatter, LnMirExprFormatterConfig},
        show::display_tree::LnMirExprDisplayTreeBuilder,
    },
    item_defn::{LnItemDefnArena, LnItemDefnComment, LnItemDefnIdxRange, LnItemDefnOrderedMap},
    stmt::{LnMirStmtArena, LnMirStmtIdxRange},
    tactic::LnMirTacticArena,
};
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};
use visored_mir_expr::{
    expr::VdMirExprIdx,
    helpers::tracker::{IsVdMirExprInput, VdMirExprTracker},
    stmt::VdMirStmtIdxRange,
};

pub struct VdLeanTranspilationTracker<'a, Scheme, Input: IsVdLeanTranspilationInput<'a, Scheme>>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    expr_arena: LnMirExprArena,
    stmt_arena: LnMirStmtArena,
    tactic_arena: LnMirTacticArena,
    defn_arena: LnItemDefnArena,
    defn_comments: LnItemDefnOrderedMap<LnItemDefnComment>,
    output: Input::VdLeanTranspilationOutput,
}

pub trait IsVdLeanTranspilationInput<'a, Scheme>: IsVdMirExprInput<'a>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    type VdLeanTranspilationOutput: IsVdLeanTranspilationOutput
        + FromVdTranspileToLean<Scheme, Self::VdMirExprOutput>;
}

pub trait IsVdLeanTranspilationOutput: std::fmt::Debug + Copy {
    fn show_display_tree(self, builder: &LnMirExprDisplayTreeBuilder) -> String;
    fn show_fmt(self, formatter: &mut LnMirExprFormatter);
}

pub trait FromVdTranspileToLean<Scheme, S>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    fn from_transpile_to_lean(s: S, builder: &mut VdLeanTranspilationBuilder<Scheme>) -> Self;
}

impl<Scheme, S, T> FromVdTranspileToLean<Scheme, S> for T
where
    Scheme: IsVdLeanTranspilationScheme,
    S: VdTranspileToLean<Scheme, T>,
{
    fn from_transpile_to_lean(s: S, builder: &mut VdLeanTranspilationBuilder<Scheme>) -> Self {
        s.to_lean(builder)
    }
}

impl<'a, Scheme, Input: IsVdLeanTranspilationInput<'a, Scheme>>
    VdLeanTranspilationTracker<'a, Scheme, Input>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    pub fn new(
        input: Input,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        models: &'a VdModels,
        db: &'a EternerDb,
        scheme: &'a Scheme,
    ) -> Self {
        let content = input.content();
        let VdMirExprTracker {
            root_module_path,
            expr_arena: vd_mir_expr_arena,
            stmt_arena: vd_mir_stmt_arena,
            symbol_local_defn_storage: vd_mir_symbol_local_defn_storage,
            source_map: vd_mir_source_map,
            sem_expr_range_map,
            sem_phrase_range_map,
            sem_clause_range_map,
            sem_sentence_range_map,
            sem_stmt_range_map,
            sem_division_range_map,
            token_storage,
            output,
        } = VdMirExprTracker::new(input, &[], &[], models, db);
        let dictionary = &VdLeanDictionary::new_standard(db);
        let mut builder = VdLeanTranspilationBuilder::new(
            db,
            scheme,
            content,
            vd_mir_expr_arena.as_arena_ref(),
            vd_mir_stmt_arena.as_arena_ref(),
            &vd_mir_symbol_local_defn_storage,
            &vd_mir_source_map,
            dictionary,
            root_module_path,
            &sem_expr_range_map,
            &sem_phrase_range_map,
            &sem_clause_range_map,
            &sem_sentence_range_map,
            &sem_stmt_range_map,
            &sem_division_range_map,
            &token_storage,
        );
        let output = FromVdTranspileToLean::from_transpile_to_lean(output, &mut builder);
        let (expr_arena, stmt_arena, tactic_arena, defn_arena, defn_comments) = builder.finish();
        Self {
            expr_arena,
            stmt_arena,
            tactic_arena,
            defn_arena,
            defn_comments,
            output,
        }
    }

    pub fn show_display_tree(&self, db: &EternerDb) -> String {
        let builder = LnMirExprDisplayTreeBuilder::new(
            db,
            self.expr_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.tactic_arena.as_arena_ref(),
            self.defn_arena.as_arena_ref(),
        );
        self.output.show_display_tree(&builder)
    }

    pub fn show_fmt(&self, db: &EternerDb) -> String {
        let fmt_config = Default::default();
        let mut formatter = self.formatter(&fmt_config, db);
        self.output.show_fmt(&mut formatter);
        formatter.finish()
    }

    fn formatter<'b>(
        &'b self,
        config: &'b LnMirExprFormatterConfig,
        db: &'b EternerDb,
    ) -> LnMirExprFormatter<'b> {
        LnMirExprFormatter::new(
            db,
            self.expr_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.tactic_arena.as_arena_ref(),
            self.defn_arena.as_arena_ref(),
            &self.defn_comments,
            config,
        )
    }
}

impl<'a, Scheme> IsVdLeanTranspilationInput<'a, Scheme> for LxDocumentInput<'a>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    type VdLeanTranspilationOutput = LnItemDefnIdxRange;
}

impl<'a, Scheme> IsVdLeanTranspilationInput<'a, Scheme> for LxDocumentBodyInput<'a>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    type VdLeanTranspilationOutput = LnItemDefnIdxRange;
}

impl<'a, Scheme> IsVdLeanTranspilationInput<'a, Scheme> for LxPageInput<'a>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    type VdLeanTranspilationOutput = LnItemDefnIdxRange;
}

impl<'a, Scheme> IsVdLeanTranspilationInput<'a, Scheme> for LxFormulaInput<'a>
where
    Scheme: IsVdLeanTranspilationScheme,
{
    type VdLeanTranspilationOutput = LnMirExprIdx;
}

impl<'a> IsVdLeanTranspilationOutput for LnItemDefnIdxRange {
    fn show_display_tree(self, builder: &LnMirExprDisplayTreeBuilder) -> String {
        DisplayTree::show_trees(&builder.render_defns(self), &Default::default())
    }

    fn show_fmt(self, formatter: &mut LnMirExprFormatter) {
        formatter.format_defns(self);
    }
}

impl<'a> IsVdLeanTranspilationOutput for LnMirExprIdx {
    fn show_display_tree(self, builder: &LnMirExprDisplayTreeBuilder) -> String {
        builder.render_expr(self).show(&Default::default())
    }

    fn show_fmt(self, formatter: &mut LnMirExprFormatter) {
        formatter.format_expr_ext(self);
    }
}
