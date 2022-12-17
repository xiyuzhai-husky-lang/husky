use crate::*;
use husky_ast::{Ast, AstIdx, AstIdxRange, AstRangeSheet, AstSheet};
use husky_token::{TokenGroupIdx, TokenSheet};
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
    fn calc_all(mut self) -> Vec<FoldingRange> {
        self.ast_sheet
            .indexed_asts()
            .filter_map(|(idx, ast)| self.calc_ast(idx, ast))
            .collect()
    }

    fn calc_ast(&self, idx: AstIdx, ast: &Ast) -> Option<FoldingRange> {
        let (text_range, kind) = match ast {
            Ast::Err(_, _)
            | Ast::Use { .. }
            | Ast::Comment(_)
            | Ast::Decor(_)
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. } => None,
            Ast::Stmt {
                token_group, body, ..
            }
            | Ast::Defn {
                token_group, body, ..
            }
            | Ast::Impl { token_group, body }
            | Ast::Main { token_group, body }
            | Ast::Config { token_group, body } => body
                .last()
                .map(|_| (self.ast_range_sheet[idx], FoldingRangeKind::Region)),
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
