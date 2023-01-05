use super::*;
use husky_ast::AstRangeSheet;

pub struct BlockExprParser<'a> {
    expr_parser: ExprParser<'a>,
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstRangeSheet,
}

impl<'a> std::ops::Deref for BlockExprParser<'a> {
    type Target = ExprParser<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expr_parser
    }
}

impl<'a> std::ops::DerefMut for BlockExprParser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr_parser
    }
}

impl<'a> BlockExprParser<'a> {
    pub fn new(
        expr_parser: ExprParser<'a>,
        ast_sheet: &'a AstSheet,
        ast_range_sheet: &'a AstRangeSheet,
    ) -> Self {
        Self {
            expr_parser,
            ast_sheet,
            ast_range_sheet,
        }
    }

    fn parse_stmt(&mut self, ast: &Ast) -> Option<Stmt> {
        match ast {
            Ast::BasicStmt {
                token_group_idx,
                body,
            } => {
                let token_stream = self
                    .token_sheet_data
                    .token_group_token_stream(*token_group_idx, None);
                let mut ctx = self.ctx(token_stream);
                match ctx.parse::<BasicStmtKeywordToken>() {
                    Ok(Some(basic_stmt_keyword_token)) => Some(match basic_stmt_keyword_token {
                        BasicStmtKeywordToken::Let(let_token) => Stmt::Let {
                            let_token,
                            let_variable_pattern: ctx.parse_expected(),
                            assign_token: ctx.parse_expected(),
                            initial_value: ctx
                                .parse_expr(ExprParseEnvironment::None)
                                .ok_or(ExprError::MissingInitialValue),
                        },
                        BasicStmtKeywordToken::Return(return_token) => Stmt::Return {
                            return_token,
                            result: ctx
                                .parse_expr(ExprParseEnvironment::None)
                                .ok_or(ExprError::MissingResult),
                        },
                        BasicStmtKeywordToken::Require(require_token) => Stmt::Require {
                            require_token,
                            condition: ctx
                                .parse_expr(ExprParseEnvironment::None)
                                .ok_or(ExprError::MissingCondition),
                        },
                        BasicStmtKeywordToken::Break(break_token) => Stmt::Break { break_token },
                        BasicStmtKeywordToken::For(_) => todo!(),
                        BasicStmtKeywordToken::Forext(_) => todo!(),
                        BasicStmtKeywordToken::While(while_token) => Stmt::While {
                            while_token,
                            condition: ctx
                                .parse_expr(ExprParseEnvironment::None)
                                .ok_or(ExprError::MissingCondition),
                            eol_colon: ctx.parse_expected(),
                            block: self.parse_block_stmts(body).ok_or(ExprError::MissingBlock),
                        },
                        BasicStmtKeywordToken::Do(_) => todo!(),
                    }),
                    Ok(None) => ctx
                        .parse_expr(ExprParseEnvironment::None)
                        .map(|expr| Stmt::Eval { expr }),
                    Err(_) => todo!(),
                }
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => Some(Stmt::IfElse {}),
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => Some(Stmt::Match {}),
            Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Decor { .. }
            | Ast::Defn { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Impl { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => None,
        }
    }

    pub fn parse_block_stmts(&mut self, body: &AstIdxRange) -> Option<StmtIdxRange> {
        if body.len() == 0 {
            return None;
        }
        let stmts = self.ast_sheet[body]
            .iter()
            .filter_map(|ast| self.parse_stmt(ast))
            .collect();
        Some(self.alloc_stmts(stmts))
    }

    pub fn parse_block_expr(&mut self, body: &AstIdxRange) -> Option<ExprIdx> {
        let stmts = self.parse_block_stmts(body)?;
        Some(self.alloc_expr(Expr::Block { stmts }))
    }

    pub fn finish(self) -> ExprSheet {
        self.expr_parser.finish()
    }
}
