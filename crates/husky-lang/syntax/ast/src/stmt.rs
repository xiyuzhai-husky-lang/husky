use text::TextRange;
use token::{Token, TokenKind};
use word::CustomIdentifier;

use crate::{atom::symbol_proxy::SymbolProxy, expr::ExprIdx, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    Loop(LoopStmt),
    Branch(BranchStmt),
    Exec(ExprIdx),
    Init {
        kind: InitKind,
        varname: CustomIdentifier,
        initial_value: ExprIdx,
    },
    Return(ExprIdx),
    Assert(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoopStmt {
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
pub enum BranchStmt {
    If { condition: ExprIdx },
    Elif { condition: ExprIdx },
    Else,
}

impl Stmt {
    pub(crate) fn parse(
        arena: &ExprArena,
        env: hir::Env,
        scope_proxy: SymbolProxy,
        keyword: Option<(StmtKeyword, TextRange)>,
        tokens: &[Token],
    ) -> AstResult<Ast> {
        macro_rules! identify {
            ($token:expr) => {{
                match &$token.kind {
                    TokenKind::Identifier(Identifier::Custom(ident)) => Ok(*ident),
                    _ => ast_err!($token.range, "expect `<custom_identifier>`"),
                }
            }};
        }

        macro_rules! expect {
            ($cond:expr, $range:expr, $msg:expr) => {
                ast_err!($range, $msg)?
            };
        }

        if let Some((keyword, range)) = keyword {
            match keyword {
                StmtKeyword::Let => {
                    expect!(tokens.len() >= 3, range, "expect at least 3 tokens");
                    let varname = identify!(&tokens[0])?;
                    expect!(
                        tokens[1].kind == TokenKind::Special(Special::Be),
                        range,
                        "expect `=`"
                    );
                    &tokens[2..];
                    todo!()
                    // let expr = &arena[expr.ok_or(ast_error!(
                    //     range,
                    //     "expect `let <varname> = <expr>`, but get `let` instead.",
                    //     src!()
                    // ))?];
                    // match &expr.kind {
                    //     ExprKind::Opn {
                    //         opr: Opr::Binary(BinaryOpr::Assign),
                    //         opds,
                    //     } => {
                    //         let varname = identify!(&arena[&opds.start])?;
                    //         let initial_value = opds.end - 1;
                    //         Ok(Stmt::Init {
                    //             kind: InitKind::Let,
                    //             varname,
                    //             initial_value,
                    //         }
                    //         .into())
                    //     }
                    //     _ => ast_err!(range, "expect: let <varname> = <expr>.", src!())?,
                    // }
                }
                StmtKeyword::Var => todo!(),
                StmtKeyword::If => {
                    todo!()
                    //     Ok(Stmt::Branch(BranchStmt::If {
                    //     condition: expr.ok_or(ast_error!(
                    //         range,
                    //         "expect `if <expr>:`, but get `if:` instead.",
                    //         src!()
                    //     ))?,
                    // })
                    // .into())
                }
                StmtKeyword::Elif => todo!(),
                StmtKeyword::Else => {
                    todo!()
                    // if expr != None {
                    //     ast_err!(
                    //         range,
                    //         "expect `else:`, but get `else <expr>:` instead.",
                    //         src!()
                    //     )?;
                    // }
                    // Ok(Stmt::Branch(BranchStmt::Else).into())
                }
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
                    todo!()
                    // Ok(Stmt::Return(expr.ok_or(ast_error!(range, "expect expr"))?).into())
                }
                StmtKeyword::Assert => {
                    todo!()
                    // Ok(Stmt::Assert(expr.ok_or(ast_error!(range, "expect expr"))?).into())
                }
            }
        } else {
            todo!()
            // Ok(Stmt::Exec(expr.unwrap()).into())
        }
    }
}
