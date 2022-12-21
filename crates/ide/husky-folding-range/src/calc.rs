use crate::*;
use husky_ast::{Ast, AstIdx, AstRangeSheet, AstSheet};

use lsp_types::FoldingRangeKind;

pub(crate) fn calc_folding_ranges(
    ast_sheet: &AstSheet,
    ast_range_sheet: &AstRangeSheet,
) -> Vec<FoldingRange> {
    FoldingRangeCalculator {
        ast_range_sheet,
        ast_sheet,
    }
    .calc_all()
}

struct FoldingRangeCalculator<'a> {
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstRangeSheet,
}

impl<'a> FoldingRangeCalculator<'a> {
    fn calc_all(self) -> Vec<FoldingRange> {
        self.ast_sheet
            .indexed_asts()
            .filter_map(|(idx, ast)| self.calc_ast(idx, ast))
            .collect()
    }

    fn calc_ast(&self, ast_idx: AstIdx, ast: &Ast) -> Option<FoldingRange> {
        let (text_range, kind) = match ast {
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Comment { .. }
            | Ast::Decor { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::ModuleItemVariant { .. } => None,
            Ast::Stmt { body, .. }
            | Ast::Defn { body, .. }
            | Ast::Impl { body, .. }
            | Ast::Main { body, .. }
            | Ast::Config { body, .. } => body
                .last()
                .map(|_| (self.ast_range_sheet[ast_idx], FoldingRangeKind::Region)),
        }?;
        Some(FoldingRange {
            start_line: text_range.start.i(),
            start_character: Some(text_range.start.j()),
            end_line: text_range.end.i(),
            end_character: Some(text_range.end.j()),
            kind: Some(kind),
        })
    }
}
