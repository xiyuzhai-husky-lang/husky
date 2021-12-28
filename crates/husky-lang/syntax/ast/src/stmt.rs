use word::CustomIdentifier;

use crate::{expr::ExprIdx, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    Loop(Loop),
    Branch(BranchStmt),
    Exec {
        expr: ExprIdx,
    },
    Init {
        kind: InitKind,
        varname: CustomIdentifier,
        initial_value: ExprIdx,
    },
    Return(ExprIdx),
    Assert(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Loop {
    For {
        left_bound: ExprIdx,
        right_bound: ExprIdx,
        is_left_shifted: bool,
        is_right_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
    },
    ForExt {
        bound: ExprIdx,
        is_shifted: bool,
        is_incremental: bool,
        fvar_ident: CustomIdentifier,
    },
    While {
        condition: ExprIdx,
    },
    DoWhile {
        condition: ExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InitKind {
    Let,
    Var,
    Functional,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BranchStmt {
    If { condition: ExprIdx },
    Elif { condition: ExprIdx },
    Else,
}

impl Stmt {
    pub(crate) fn parse(
        arena: &ExprArena,
        attr: &atom::StmtAttr,
        expr: Option<ExprIdx>,
    ) -> AstResult<Ast> {
        macro_rules! identify {
            ($expr:expr) => {{
                identify($expr, src!())
            }};
        }

        fn identify(expr: &Expr, src: DevSource) -> AstResult<CustomIdentifier> {
            match &expr.kind {
                ExprKind::Variable(Identifier::Custom(ident)) => Ok(*ident),
                _ => ast_err!(expr.range, "expect `<custom_identifier>`", src),
            }
        }

        if let Some((keyword, range)) = &attr.keyword {
            match keyword {
                StmtKeyword::Let => {
                    let expr = &arena[expr.ok_or(ast_error!(
                        range,
                        "expect `let <var> = <expr>`, but get `let` instead.",
                        src!()
                    ))?];
                    match &expr.kind {
                        ExprKind::Opn {
                            opr: Opr::Binary(BinaryOpr::Assign),
                            opds,
                        } => {
                            let varname = identify!(&arena[&opds.start])?;
                            let initial_value = opds.end - 1;
                            Ok(Stmt::Init {
                                kind: InitKind::Let,
                                varname,
                                initial_value,
                            }
                            .into())
                        }
                        _ => ast_err!(range, "expect: let <var> = <expr>.", src!())?,
                    }
                }
                StmtKeyword::Var => todo!(),
                StmtKeyword::If => todo!(),
                StmtKeyword::Elif => todo!(),
                StmtKeyword::Else => todo!(),
                StmtKeyword::Switch => todo!(),
                StmtKeyword::Match => todo!(),
                StmtKeyword::Case => todo!(),
                StmtKeyword::DeFault => todo!(),
                StmtKeyword::For => todo!(),
                StmtKeyword::Ext => todo!(),
                StmtKeyword::ForExt => todo!(),
                StmtKeyword::While => todo!(),
                StmtKeyword::Do => todo!(),
                StmtKeyword::Break => todo!(),
                StmtKeyword::Return => {
                    Ok(Stmt::Return(expr.ok_or(ast_error!(range, "expect expr"))?).into())
                }
                StmtKeyword::Assert => {
                    Ok(Stmt::Assert(expr.ok_or(ast_error!(range, "expect expr"))?).into())
                }
            }
        } else {
            todo!()
        }
    }
}
