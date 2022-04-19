use crate::{
    stmt::{RawBranchKind, RawLoopKind, RawStmtKind},
    transform::utils::*,
    *,
};
use atom::symbol::Symbol;
use text::{TextRange, TextRanged};
use token::{Special, Token, TokenKind};
use vm::BinaryOpr;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_stmt_with_keyword(
        &mut self,
        keyword: StmtKeyword,
        token_group: &[Token],
    ) -> AstResult<RawStmt> {
        let kw_range = token_group[0].text_range();
        Ok(RawStmt {
            range: token_group.into(),
            kind: match keyword {
                StmtKeyword::Let => {
                    self.parse_init_stmt(InitKind::Let, kw_range, &token_group[1..])?
                }
                StmtKeyword::Var => {
                    self.parse_init_stmt(InitKind::Var, kw_range, &token_group[1..])?
                }
                StmtKeyword::If => {
                    expect_at_least!(token_group, kw_range, 3);
                    expect_block_head!(token_group);
                    RawStmtKind::Branch(RawBranchKind::If {
                        condition: self.parse_expr(&token_group[1..(token_group.len() - 1)])?,
                    })
                }
                StmtKeyword::Elif => todo!(),
                StmtKeyword::Else => {
                    expect!(token_group.len() == 2, "expect one tokens after", kw_range);
                    expect!(
                        token_group[1].kind == TokenKind::Special(Special::Colon),
                        "expect `:`",
                        token_group[1].range
                    );
                    RawStmtKind::Branch(RawBranchKind::Else)
                }
                StmtKeyword::Switch => todo!(),
                StmtKeyword::Match => todo!(),
                StmtKeyword::Case => todo!(),
                StmtKeyword::DeFault => todo!(),
                StmtKeyword::For => self.parse_for_loop(token_group)?,
                StmtKeyword::ForExt => self.parse_forext_loop(token_group)?,
                StmtKeyword::While => self.parse_while_loop(token_group)?,
                StmtKeyword::Do => self.parse_do_while_loop(token_group)?,
                StmtKeyword::Break => todo!(),
                StmtKeyword::Return => {
                    expect!(token_group.len() >= 2, "expect some tokens after", kw_range);
                    RawStmtKind::Return(self.parse_expr(&token_group[1..])?)
                }
                StmtKeyword::Assert => {
                    expect!(token_group.len() >= 2, "expect some tokens after", kw_range);
                    RawStmtKind::Assert(self.parse_expr(&token_group[1..])?)
                }
            },
        })
    }

    pub(super) fn parse_stmt_without_keyword(
        &mut self,
        token_group: &[Token],
    ) -> AstResult<RawStmt> {
        Ok(match self.env() {
            AstContext::Package(_) => todo!(),
            AstContext::Module(_) => todo!(),
            AstContext::DatasetConfig
            | AstContext::Main
            | AstContext::Morphism
            | AstContext::Func => {
                if token_group.len() > 2 && token_group[1].kind == Special::Assign.into() {
                    // declarative initialization
                    let varname = identify!(token_group[0]);
                    self.symbols
                        .push(Symbol::var(varname, token_group[0].row()));
                    RawStmt {
                        range: token_group.into(),
                        kind: RawStmtKind::Init {
                            init_kind: InitKind::Decl,
                            varname,
                            initial_value: self.parse_expr(&token_group[2..])?,
                        },
                    }
                } else {
                    // declarative return
                    RawStmt {
                        range: token_group.into(),
                        kind: RawStmtKind::Return(self.parse_expr(token_group)?),
                    }
                }
            }
            AstContext::Proc => RawStmt {
                range: token_group.into(),
                kind: RawStmtKind::Exec(self.parse_expr(token_group)?),
            },
            AstContext::Test => todo!(),
            AstContext::Struct | AstContext::Enum => panic!(),
            AstContext::Record => todo!(),
            AstContext::Props => todo!(),
        })
        // Ok(Stmt::Exec(expr.unwrap()).into())
    }
    fn parse_init_stmt(
        &mut self,
        kind: InitKind,
        kw_range: TextRange,
        tokens: &[Token],
    ) -> AstResult<RawStmtKind> {
        match kind {
            InitKind::Let | InitKind::Var => match self.env() {
                AstContext::Proc | AstContext::Test => (),
                _ => err!(
                    format!(
                        "`{}` statement requires env to be `proc` or `test`, but got `{}` instead",
                        kind,
                        self.env()
                    ),
                    kw_range
                )?,
            },
            InitKind::Decl => todo!(),
        }
        expect_at_least!(tokens, kw_range, 3);
        let varname = identify!(&tokens[0]);
        self.symbols.push(Symbol::var(varname, tokens[0].row()));
        expect_kind!(tokens[1], Special::Assign);
        let initial_value = self.parse_expr(&tokens[2..])?;
        Ok(RawStmtKind::Init {
            init_kind: kind,
            varname,
            initial_value,
        })
    }

    fn parse_for_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(token_group);
        let expr = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        let expr = &self.arena[expr];
        Ok(match expr.kind {
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Prefix(_) | Opr::Suffix(_) | Opr::List(_) => todo!(),
                Opr::Binary(binary) => match binary {
                    BinaryOpr::Assign(_) => todo!(),
                    BinaryOpr::Pure(pure_binary) => {
                        let lopd_idx = opds.start;
                        let ropd_idx = opds.end - 1;
                        let lopd = &self.arena[lopd_idx];
                        let ropd = &self.arena[ropd_idx];
                        if let RawExprVariant::Unrecognized(frame_var) = lopd.kind {
                            RawLoopKind::for_loop_with_default_initial(
                                frame_var,
                                pure_binary,
                                opds.end - 1,
                                self.file,
                                expr.range(),
                            )?
                            .into()
                        } else if let RawExprVariant::Unrecognized(frame_var) = ropd.kind {
                            RawLoopKind::for_loop_with_default_final(
                                opds.start,
                                pure_binary,
                                frame_var,
                                self.file,
                                expr.range(),
                            )?
                            .into()
                        } else {
                            let final_comparison = pure_binary;
                            match lopd.kind {
                                RawExprVariant::Opn { opr, ref opds } => {
                                    let llopd_idx = opds.start;
                                    let lropd_idx = opds.end - 1;
                                    let lropd = &self.arena[lropd_idx];
                                    let initial_comparison = match opr {
                                        Opr::Binary(binary) => match binary {
                                            BinaryOpr::Pure(pure_binary_opr) => pure_binary_opr,
                                            BinaryOpr::Assign(_) => todo!(),
                                        },
                                        _ => todo!(),
                                    };
                                    let frame_var = if let RawExprVariant::Unrecognized(frame_var) =
                                        lropd.kind
                                    {
                                        frame_var
                                    } else {
                                        err!("expect unrecognized", expr.range())?
                                    };
                                    RawLoopKind::for_loop(
                                        llopd_idx,
                                        initial_comparison,
                                        frame_var,
                                        final_comparison,
                                        ropd_idx,
                                        self.file,
                                        expr.range(),
                                    )?
                                    .into()
                                }
                                _ => todo!(),
                            }
                            // LoopRawStmt::for_loop()?.into()
                        }
                    }
                },
            },
            _ => todo!(),
        })
    }

    fn parse_forext_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(token_group);
        let expr_idx = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        let expr = &self.arena[expr_idx];
        Ok(match expr.kind {
            RawExprVariant::Opn {
                opr: Opr::Binary(BinaryOpr::Pure(comparison)),
                ref opds,
            } => {
                let lopd_idx = opds.start;
                let ropd_idx = opds.end - 1;
                let lopd = &self.arena[lopd_idx];
                let frame_var = match lopd.kind {
                    RawExprVariant::Variable { varname, .. } => varname,
                    _ => todo!(),
                };
                RawLoopKind::forext_loop(frame_var, comparison, ropd_idx)?.into()
            }
            _ => todo!(),
        })
    }

    fn parse_while_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(token_group);
        let expr_idx = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        Ok(RawLoopKind::while_loop(expr_idx).into())
    }

    fn parse_do_while_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(token_group);
        match token_group[1].kind {
            TokenKind::Keyword(Keyword::Stmt(StmtKeyword::While)) => (),
            _ => {
                todo!()
            }
        }
        let expr_idx = self.parse_expr(&token_group[2..(token_group.len() - 1)])?;
        Ok(RawLoopKind::do_while_loop(expr_idx).into())
    }
}
