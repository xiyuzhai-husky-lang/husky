use husky_token::{HasTokenIdxRange, TokenDb, TokenIdxRange, TokenSheetData};

use crate::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct AstTokenIdxRangeSheet {
    ast_token_idx_ranges: Vec<TokenIdxRange>,
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_token_idx_range_sheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> AstTokenIdxRangeSheet {
    let token_sheet_data = db.token_sheet_data(module_path);
    let ast_sheet = module_path.ast_sheet(db);
    AstTokenIdxRangeSheet {
        ast_token_idx_ranges: AstTokenIdxRangeCalculator {
            token_sheet_data,
            ast_sheet,
            ast_ranges: Default::default(),
        }
        .calc_all(),
    }
}

#[test]
fn ast_range_sheet_works() {
    DB::token_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_path.ast_token_idx_range_sheet(db),
        &TokenTestConfig::new(
            "ast_range_sheet",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    );
}

impl std::ops::Index<AstIdx> for AstTokenIdxRangeSheet {
    type Output = TokenIdxRange;

    fn index(&self, idx: AstIdx) -> &Self::Output {
        &self.ast_token_idx_ranges[idx.index()]
    }
}

impl AstTokenIdxRangeSheet {
    pub fn asts_token_idx_range(&self, idx_range: AstIdxRange) -> TokenIdxRange {
        self[idx_range.start()].join(self[idx_range.end() - 1])
    }
}

struct AstTokenIdxRangeCalculator<'a> {
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_ranges: Vec<TokenIdxRange>,
}

impl<'a> AstTokenIdxRangeCalculator<'a> {
    fn calc_all(mut self) -> Vec<TokenIdxRange> {
        for ast in self.ast_sheet.data() {
            self.ast_ranges.push(self.calc_ast(ast))
        }
        self.ast_ranges
    }

    fn calc_ast(&self, ast: &Ast) -> TokenIdxRange {
        match ast {
            Ast::Err {
                token_verse_idx, ..
            }
            | Ast::Use {
                token_verse_idx, ..
            }
            | Ast::Sorc {
                token_verse_idx, ..
            }
            | Ast::Attr {
                token_verse_idx, ..
            }
            | Ast::TypeVariant {
                token_verse_idx, ..
            } => self
                .token_sheet_data
                .token_verse_token_idx_range(*token_verse_idx),
            Ast::BasicStmtOrBranch {
                token_verse_idx,
                body,
                ..
            } => self.calc_ast_group(*token_verse_idx, body.map(|body| body.ast_idx_range())),
            Ast::Identifiable {
                token_verse_idx,
                block,
                ..
            } => self.calc_ast_group(*token_verse_idx, block.children()),
            Ast::ImplBlock {
                token_verse_idx,
                items: body,
                ..
            } => self.calc_ast_group(*token_verse_idx, body.map(|body| body.ast_idx_range())),
            Ast::IfElseStmts {
                if_branch: if_stmt,
                elif_branches: elif_stmts,
                else_branch: else_stmt,
            } => {
                let if_stmt_token_idx_range = self.ast_ranges[if_stmt.index()].token_idx_range();
                let start = if_stmt_token_idx_range.start();
                let end = match else_stmt {
                    Some(else_stmt) => self.ast_ranges[else_stmt.index()].end(),
                    None => {
                        if let Some(last) = elif_stmts.last() {
                            self.ast_ranges[last.index()].end()
                        } else {
                            self.ast_ranges[if_stmt.index()].end()
                        }
                    }
                };
                (start, end).into()
            }
            Ast::MatchStmt {
                pattern_stmt,
                case_branches,
                ..
            } => {
                let pattern_stmt_token_idx_range =
                    self.ast_ranges[pattern_stmt.index()].token_idx_range();
                let start = pattern_stmt_token_idx_range.start();
                let end = {
                    if let Some(last) = case_branches.last() {
                        self.ast_ranges[last.index()].end()
                    } else {
                        pattern_stmt_token_idx_range.end()
                    }
                };
                (start, end).into()
            }
        }
    }

    fn calc_ast_group(
        &self,
        token_verse_idx: TokenVerseIdx,
        ast_idx_range: impl Into<Option<AstIdxRange>>,
    ) -> TokenIdxRange {
        let token_verse_token_idx_range = self
            .token_sheet_data
            .token_verse_token_idx_range(token_verse_idx);
        let start = token_verse_token_idx_range.start();
        let ast_idx_range: Option<AstIdxRange> = ast_idx_range.into();
        let end = match ast_idx_range {
            Some(ast_idx_range) => match ast_idx_range.last() {
                Some(last) => self.ast_ranges[last.index()].end(),
                None => token_verse_token_idx_range.end(),
            },
            None => token_verse_token_idx_range.end(),
        };
        (start, end).into()
    }
}
