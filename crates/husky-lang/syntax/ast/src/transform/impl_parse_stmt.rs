use crate::{
    atom::symbol_proxy::Symbol,
    stmt::{RawBranchKind, RawLoopKind, RawStmtKind},
    transform::utils::*,
    *,
};
use text::{TextRange, TextRanged};
use token::{Special, Token, TokenKind};
use vm::{BinaryOpr, PureBinaryOpr};

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_stmt(
        &mut self,
        keyword: Option<(StmtKeyword, TextRange)>,
        tokens: &[Token],
    ) -> AstResult<RawStmt> {
        Ok(if let Some((keyword, kw_range)) = keyword {
            RawStmt {
                range: tokens.into(),
                kind: match keyword {
                    StmtKeyword::Let => self.parse_init_stmt(InitKind::Let, kw_range, tokens)?,
                    StmtKeyword::Var => self.parse_init_stmt(InitKind::Var, kw_range, tokens)?,
                    StmtKeyword::If => {
                        expect_at_least!(tokens, kw_range, 2);
                        expect_block_head!(tokens);
                        RawStmtKind::Branch(RawBranchKind::If {
                            condition: self.parse_expr(&tokens[0..(tokens.len() - 1)])?,
                        })
                    }
                    StmtKeyword::Elif => todo!(),
                    StmtKeyword::Else => {
                        expect!(tokens.len() == 1, kw_range, "expect one tokens after");
                        expect!(
                            tokens[0].kind == TokenKind::Special(Special::Colon),
                            tokens[0].range,
                            "expect `:`"
                        );
                        RawStmtKind::Branch(RawBranchKind::Else)
                    }
                    StmtKeyword::Switch => todo!(),
                    StmtKeyword::Match => todo!(),
                    StmtKeyword::Case => todo!(),
                    StmtKeyword::DeFault => todo!(),
                    StmtKeyword::For => self.parse_for_loop(tokens)?,
                    StmtKeyword::ForExt => self.parse_forext_loop(tokens)?,
                    StmtKeyword::While => self.parse_while_loop(tokens)?,
                    StmtKeyword::Do => self.parse_do_while_loop(tokens)?,
                    StmtKeyword::Break => todo!(),
                    StmtKeyword::Return => {
                        expect!(tokens.len() > 0, kw_range, "expect some tokens after");
                        RawStmtKind::Return(self.parse_expr(tokens)?)
                    }
                    StmtKeyword::Assert => {
                        expect!(tokens.len() > 0, kw_range, "expect some tokens after");
                        RawStmtKind::Assert(self.parse_expr(tokens)?)
                    }
                },
            }
        } else {
            if tokens.len() > 2 && tokens[1].kind == Special::Assign.into() {
                // declarative initialization
                let varname = identify!(tokens[0]);
                self.symbols
                    .push(Symbol::var(varname, tokens[0].text_range()));
                RawStmt {
                    range: tokens.into(),
                    kind: RawStmtKind::Init {
                        init_kind: InitKind::Decl,
                        varname,
                        initial_value: self.parse_expr(&tokens[2..])?,
                    },
                }
            } else {
                match self.env() {
                    Env::Package => todo!(),
                    Env::Module(_) => todo!(),
                    Env::DatasetConfig | Env::Main | Env::Def | Env::Func => {
                        // declarative return
                        RawStmt {
                            range: tokens.into(),
                            kind: RawStmtKind::Return(self.parse_expr(tokens)?),
                        }
                    }
                    Env::Proc => RawStmt {
                        range: tokens.into(),
                        kind: RawStmtKind::Exec(self.parse_expr(tokens)?),
                    },
                    Env::Test => todo!(),
                }
            }
            // Ok(Stmt::Exec(expr.unwrap()).into())
        })
    }

    fn parse_init_stmt(
        &mut self,
        kind: InitKind,
        kw_range: TextRange,
        tokens: &[Token],
    ) -> AstResult<RawStmtKind> {
        match kind {
            InitKind::Let | InitKind::Var => match self.env() {
                Env::Proc | Env::Test => (),
                _ => err!(
                    kw_range,
                    format!(
                        "`{}` statement requires env to be `proc` or `test`, but got `{}` instead",
                        kind,
                        self.env()
                    )
                )?,
            },
            InitKind::Decl => todo!(),
        }
        expect_at_least!(tokens, kw_range, 3);
        let varname = identify!(&tokens[0]);
        self.symbols
            .push(Symbol::var(varname, tokens[0].range.clone()));
        expect_kind!(tokens[1], Special::Assign);
        let initial_value = self.parse_expr(&tokens[2..])?;
        Ok(RawStmtKind::Init {
            init_kind: kind,
            varname,
            initial_value,
        })
    }

    fn parse_for_loop(&mut self, tokens: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(tokens);
        let expr = self.parse_expr(&tokens[0..(tokens.len() - 1)])?;
        let expr = &self.arena[expr];
        Ok(match expr.kind {
            RawExprKind::Opn { opr, ref opds } => match opr {
                Opr::Prefix(_) | Opr::Suffix(_) | Opr::List(_) => todo!(),
                Opr::Binary(binary) => match binary {
                    BinaryOpr::Assign(_) => todo!(),
                    BinaryOpr::Pure(pure_binary) => {
                        let lopd_idx = opds.start;
                        let ropd_idx = opds.end - 1;
                        let lopd = &self.arena[lopd_idx];
                        let ropd = &self.arena[ropd_idx];
                        if let RawExprKind::Unrecognized(frame_var) = lopd.kind {
                            RawLoopKind::for_loop_with_default_initial(
                                frame_var,
                                pure_binary,
                                opds.end - 1,
                                expr.range(),
                            )?
                            .into()
                        } else if let RawExprKind::Unrecognized(frame_var) = ropd.kind {
                            RawLoopKind::for_loop_with_default_final(
                                opds.start,
                                pure_binary,
                                frame_var,
                                expr.range(),
                            )?
                            .into()
                        } else {
                            let final_comparison = pure_binary;
                            match lopd.kind {
                                RawExprKind::Opn { opr, ref opds } => {
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
                                    let frame_var =
                                        if let RawExprKind::Unrecognized(frame_var) = lropd.kind {
                                            frame_var
                                        } else {
                                            err!(expr.range(), "expect unrecognized")?
                                        };
                                    RawLoopKind::for_loop(
                                        llopd_idx,
                                        initial_comparison,
                                        frame_var,
                                        final_comparison,
                                        ropd_idx,
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

    fn parse_forext_loop(&mut self, tokens: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(tokens);
        let expr_idx = self.parse_expr(&tokens[0..(tokens.len() - 1)])?;
        let expr = &self.arena[expr_idx];
        Ok(match expr.kind {
            RawExprKind::Opn {
                opr: Opr::Binary(BinaryOpr::Pure(comparison)),
                ref opds,
            } => {
                let lopd_idx = opds.start;
                let ropd_idx = opds.end - 1;
                let lopd = &self.arena[lopd_idx];
                let frame_var = match lopd.kind {
                    RawExprKind::Variable(frame_var) => frame_var,
                    _ => todo!(),
                };
                RawLoopKind::forext_loop(frame_var, comparison, ropd_idx)?.into()
            }
            _ => todo!(),
        })
    }

    fn parse_while_loop(&mut self, tokens: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(tokens);
        let expr_idx = self.parse_expr(&tokens[0..(tokens.len() - 1)])?;
        Ok(RawLoopKind::while_loop(expr_idx).into())
    }

    fn parse_do_while_loop(&mut self, tokens: &[Token]) -> AstResult<RawStmtKind> {
        expect_block_head!(tokens);
        match tokens[0].kind {
            TokenKind::Keyword(Keyword::Stmt(StmtKeyword::While)) => (),
            _ => todo!(),
        }
        let expr_idx = self.parse_expr(&tokens[1..(tokens.len() - 1)])?;
        Ok(RawLoopKind::do_while_loop(expr_idx).into())
    }
}
