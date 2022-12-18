use husky_token::TokenSheet;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct AstRangeSheet {
    text_ranges: Vec<TextRange>,
}

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_range_sheet(db: &dyn AstDb, module: EntityPath) -> VfsResult<AstRangeSheet> {
    let token_sheet = db.token_sheet(module).as_ref()?;
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    Ok(AstRangeSheet {
        text_ranges: AstRangeCalculator {
            token_sheet,
            ast_sheet,
            text_ranges: Default::default(),
        }
        .calc_all(),
    })
}

#[test]
fn ast_range_sheet_works() {
    use tests::*;
    DB::expect_test_modules("ast_range_sheet", AstDb::ast_range_sheet);
}

impl std::ops::Index<AstIdx> for AstRangeSheet {
    type Output = TextRange;

    fn index(&self, index: AstIdx) -> &Self::Output {
        &self.text_ranges[index.raw()]
    }
}

struct AstRangeCalculator<'a> {
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    text_ranges: Vec<TextRange>,
}

impl<'a> AstRangeCalculator<'a> {
    fn calc_all(mut self) -> Vec<TextRange> {
        for ast in self.ast_sheet.arena.data() {
            self.text_ranges.push(self.calc_ast_range(ast))
        }
        self.text_ranges
    }

    fn calc_ast_range(&self, ast: &Ast) -> TextRange {
        match ast {
            Ast::Err(token_group, _)
            | Ast::Use { token_group, .. }
            | Ast::Comment(token_group)
            | Ast::Decor(token_group) => self.token_sheet[*token_group].text_range(),
            Ast::Stmt {
                token_group, body, ..
            }
            | Ast::Defn {
                token_group, body, ..
            }
            | Ast::Impl {
                token_group, body, ..
            }
            | Ast::Main { token_group, body }
            | Ast::Config { token_group, body } => {
                let start = self.token_sheet[*token_group].first().unwrap().text_start();
                let end = match body.last() {
                    Some(last) => self.text_ranges[last.raw()].text_end(),
                    None => self.token_sheet[*token_group].last().unwrap().text_end(),
                };
                (start..end).into()
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => {
                let start = self.text_ranges[if_stmt.raw()].start;
                let end = match else_stmt {
                    Some(else_stmt) => self.text_ranges[else_stmt.raw()].end,
                    None => {
                        if let Some(last) = elif_stmts.last() {
                            self.text_ranges[last.raw()].end
                        } else {
                            self.text_ranges[if_stmt.raw()].end
                        }
                    }
                };
                (start..end).into()
            }
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => {
                let start = self.text_ranges[pattern_stmt.raw()].start;
                let end = {
                    if let Some(last) = case_stmts.last() {
                        self.text_ranges[last.raw()].end
                    } else {
                        self.text_ranges[pattern_stmt.raw()].end
                    }
                };
                (start..end).into()
            }
        }
    }
}
