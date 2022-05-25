mod impl_match;

use crate::{stmt::*, *};
use atom::symbol::{Symbol, SymbolKind};
use text::{TextRange, TextRanged};
use token::*;
use vm::*;
use word::RoutineKeyword;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_stmt_with_keyword(
        &mut self,
        keyword: StmtKeyword,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<RawStmt> {
        let kw_range = token_group[0].text_range();
        self.abs_semantic_tokens
            .push(AbsSemanticToken::new(SemanticTokenKind::Keyword, kw_range));
        Ok(RawStmt {
            range: token_group.text_range(),
            variant: match keyword {
                StmtKeyword::Let => {
                    self.parse_init_stmt(InitKind::Let, kw_range, &token_group[1..])?
                }
                StmtKeyword::Var => {
                    self.parse_init_stmt(InitKind::Var, kw_range, &token_group[1..])?
                }
                StmtKeyword::If => {
                    expect_block_head!(token_group);
                    expect_at_least!(token_group, kw_range, 3);
                    RawStmtVariant::ConditionBranch {
                        condition_branch_kind: RawConditionBranchKind::If {
                            condition: self.parse_expr(&token_group[1..(token_group.len() - 1)])?,
                        },
                    }
                }
                StmtKeyword::Elif => {
                    expect_block_head!(token_group);
                    expect_at_least!(token_group, kw_range, 3);
                    RawStmtVariant::ConditionBranch {
                        condition_branch_kind: RawConditionBranchKind::Elif {
                            condition: self.parse_expr(&token_group[1..(token_group.len() - 1)])?,
                        },
                    }
                }
                StmtKeyword::Else => {
                    must_be!(token_group.len() == 2, "expect one tokens after", kw_range);
                    must_be!(
                        token_group[1].kind == TokenKind::Special(Special::Colon),
                        "expect `:` at the end",
                        token_group[0].text_range()
                    );
                    RawStmtVariant::ConditionBranch {
                        condition_branch_kind: RawConditionBranchKind::Else,
                    }
                }
                StmtKeyword::Match => {
                    enter_block(self);
                    match self.context() {
                        AstContext::Main | AstContext::Lazy => {
                            self.context.set(AstContext::LazyMatch)
                        }
                        AstContext::DatasetConfig | AstContext::Routine(RoutineKeyword::Func) => {
                            self.context.set(AstContext::FuncMatch)
                        }
                        AstContext::Routine(RoutineKeyword::Proc) => {
                            self.context.set(AstContext::ProcMatch)
                        }
                        _ => todo!("can't put match here"),
                    }
                    self.parse_match(token_group)?
                }
                StmtKeyword::Case => {
                    enter_block(self);
                    match self.context() {
                        AstContext::FuncMatch => {
                            self.context.set(AstContext::Routine(RoutineKeyword::Func))
                        }
                        AstContext::ProcMatch => {
                            self.context.set(AstContext::Routine(RoutineKeyword::Proc))
                        }
                        AstContext::LazyMatch => self.context.set(AstContext::Lazy),
                        _ => {
                            return err!(
                                "can't put case outside a match context",
                                token_group.text_range()
                            )
                        }
                    }
                    self.parse_case(token_group)?
                }
                StmtKeyword::DeFault => {
                    enter_block(self);
                    match self.context() {
                        AstContext::FuncMatch => {
                            self.context.set(AstContext::Routine(RoutineKeyword::Func))
                        }
                        AstContext::ProcMatch => {
                            self.context.set(AstContext::Routine(RoutineKeyword::Proc))
                        }
                        AstContext::LazyMatch => self.context.set(AstContext::Lazy),
                        _ => {
                            return err!(
                                "can't put case outside a match context",
                                token_group.text_range()
                            )
                        }
                    }
                    expect_head!(token_group);
                    must_be!(token_group.len() == 2, "expect some tokens after", kw_range);
                    RawStmtVariant::PatternBranch {
                        pattern_branch_variant: RawPatternBranchVariant::Default,
                    }
                }
                StmtKeyword::For => self.parse_for_loop(token_group)?,
                StmtKeyword::ForExt => self.parse_forext_loop(token_group)?,
                StmtKeyword::While => self.parse_while_loop(token_group)?,
                StmtKeyword::Do => self.parse_do_while_loop(token_group)?,
                StmtKeyword::Break => {
                    if token_group.len() != 1 {
                        todo!()
                    }
                    RawStmtVariant::Break
                }
                StmtKeyword::Return => {
                    must_be!(token_group.len() >= 2, "expect some tokens after", kw_range);
                    RawStmtVariant::Return(self.parse_expr(&token_group[1..])?)
                }
                StmtKeyword::Assert => {
                    must_be!(token_group.len() >= 2, "expect some tokens after", kw_range);
                    RawStmtVariant::Assert(self.parse_expr(&token_group[1..])?)
                }
            },
        })
    }

    pub(super) fn parse_stmt_without_keyword(
        &mut self,
        token_group: &[Token],
    ) -> AstResult<RawStmt> {
        Ok(match self.context() {
            AstContext::Package(_) => todo!(),
            AstContext::Module(_) => todo!(),
            AstContext::DatasetConfig
            | AstContext::Main
            | AstContext::Lazy
            | AstContext::Routine(RoutineKeyword::Func)
            | AstContext::Visual => {
                if token_group.len() > 2 && token_group[1].kind == Special::Assign.into() {
                    // declarative initialization
                    let varname =
                        identify_token!(self, token_group[0], SemanticTokenKind::Variable);
                    self.symbols.push(Symbol::variable(varname));
                    RawStmt {
                        range: token_group.text_range(),
                        variant: RawStmtVariant::Init {
                            init_kind: InitKind::Decl,
                            varname,
                            initial_value: self.parse_expr(&token_group[2..])?,
                        },
                    }
                } else {
                    // declarative return
                    if self.context() == AstContext::Visual {
                        // return xml expr
                        RawStmt {
                            range: token_group.text_range(),
                            variant: RawStmtVariant::ReturnXml(self.parse_xml_expr(token_group)?),
                        }
                    } else {
                        // return eager expr
                        RawStmt {
                            range: token_group.text_range(),
                            variant: RawStmtVariant::Return(self.parse_expr(token_group)?),
                        }
                    }
                }
            }
            AstContext::Routine(RoutineKeyword::Proc) => {
                let (expr_tokens, discard) = match token_group.last().unwrap().kind {
                    TokenKind::Special(Special::Semicolon) => {
                        (&token_group[..(token_group.len() - 1)], true)
                    }
                    _ => (token_group, false),
                };
                RawStmt {
                    range: token_group.text_range(),
                    variant: RawStmtVariant::Exec {
                        expr: self.parse_expr(expr_tokens)?,
                        discard,
                    },
                }
            }
            AstContext::Struct | AstContext::Enum(_) => panic!(),
            AstContext::Record => todo!(),
            AstContext::Props => todo!(),
            AstContext::FuncMatch => err!(format!("expect case stmt"), token_group.text_range())?,
            AstContext::ProcMatch => todo!(),
            AstContext::LazyMatch => todo!(),
        })
        // Ok(Stmt::Exec(expr.unwrap()).into())
    }
    fn parse_init_stmt(
        &mut self,
        kind: InitKind,
        kw_range: TextRange,
        tokens: &[Token],
    ) -> AstResult<RawStmtVariant> {
        match kind {
            InitKind::Let | InitKind::Var => match self.context() {
                AstContext::Routine(RoutineKeyword::Proc) => (),
                _ => err!(
                    format!(
                        "`{}` statement requires env to be `proc` or `test`, but got `{}` instead",
                        kind,
                        self.context()
                    ),
                    kw_range
                )?,
            },
            InitKind::Decl => todo!(),
        }
        expect_at_least!(tokens, kw_range, 3);
        let varname = identify_token!(self, &tokens[0], SemanticTokenKind::Variable);
        self.symbols.push(Symbol::variable(varname));
        expect_token_kind!(tokens[1], Special::Assign);
        let initial_value = self.parse_expr(&tokens[2..])?;
        Ok(RawStmtVariant::Init {
            init_kind: kind,
            varname,
            initial_value,
        })
    }

    fn parse_for_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        let expr = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        let expr = &self.arena[expr];
        match expr.variant {
            RawExprVariant::Opn { ref opr, ref opds } => match opr {
                Opr::Prefix(_) | Opr::Suffix(_) | Opr::List(_) => todo!(),
                Opr::Binary(binary) => match binary {
                    BinaryOpr::Assign(_) => todo!(),
                    BinaryOpr::Pure(pure_binary) => {
                        let lopd_idx = opds.start;
                        let ropd_idx = opds.end - 1;
                        let lopd = &self.arena[lopd_idx];
                        let ropd = &self.arena[ropd_idx];
                        let (frame_var, kind) = if let RawExprVariant::Unrecognized(frame_var) =
                            lopd.variant
                        {
                            let frame_var = RangedCustomIdentifier {
                                ident: frame_var,
                                range: lopd.range,
                            };
                            (
                                frame_var,
                                RawLoopKind::for_loop_with_default_initial(
                                    frame_var,
                                    *pure_binary,
                                    opds.end - 1,
                                    expr.range(),
                                )?
                                .into(),
                            )
                        } else if let RawExprVariant::Unrecognized(frame_var) = ropd.variant {
                            let frame_var = RangedCustomIdentifier {
                                ident: frame_var,
                                range: ropd.range,
                            };
                            (
                                frame_var,
                                RawLoopKind::for_loop_with_default_final(
                                    opds.start,
                                    *pure_binary,
                                    frame_var,
                                    expr.range(),
                                )?
                                .into(),
                            )
                        } else {
                            let final_comparison = pure_binary;
                            match lopd.variant {
                                RawExprVariant::Opn { ref opr, ref opds } => {
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
                                        lropd.variant
                                    {
                                        RangedCustomIdentifier {
                                            ident: frame_var,
                                            range: lropd.range,
                                        }
                                    } else {
                                        err!("expect unrecognized", expr.range())?
                                    };
                                    (
                                        frame_var,
                                        RawLoopKind::for_loop(
                                            llopd_idx,
                                            *initial_comparison,
                                            frame_var,
                                            *final_comparison,
                                            ropd_idx,
                                        )?
                                        .into(),
                                    )
                                }
                                _ => todo!(),
                            }
                        };
                        self.insert_abs_semantic_token(AbsSemanticToken::new(
                            SemanticTokenKind::FrameVariable,
                            frame_var.range,
                        ));
                        self.symbols.push(Symbol {
                            ident: frame_var.ident,
                            kind: SymbolKind::FrameVariable {
                                init_range: frame_var.range,
                            },
                        });
                        Ok(kind)
                    }
                },
            },
            _ => todo!(),
        }
    }

    fn parse_forext_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        let expr_idx = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        let expr = &self.arena[expr_idx];
        Ok(match expr.variant {
            RawExprVariant::Opn {
                opr: Opr::Binary(BinaryOpr::Pure(comparison)),
                ref opds,
            } => {
                let lopd_idx = opds.start;
                let ropd_idx = opds.end - 1;
                let lopd = &self.arena[lopd_idx];
                let frame_var = RangedCustomIdentifier {
                    ident: match lopd.variant {
                        RawExprVariant::Variable { varname, .. } => varname,
                        _ => todo!(),
                    },
                    range: lopd.range,
                };
                RawLoopKind::forext_loop(frame_var, comparison, ropd_idx)?.into()
            }
            _ => todo!(),
        })
    }

    fn parse_while_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        let expr_idx = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        Ok(RawLoopKind::while_loop(expr_idx).into())
    }

    fn parse_do_while_loop(&mut self, token_group: &[Token]) -> AstResult<RawStmtVariant> {
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
