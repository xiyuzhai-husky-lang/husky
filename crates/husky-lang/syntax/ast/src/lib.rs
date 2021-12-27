mod error;
mod expr;
mod gen;
mod query;
mod stmt;

use common::*;

pub use expr::*;
pub use gen::AstGenResult;
pub use query::{AstQuery, AstQueryStorage};

use scope::ScopeId;

use stmt::Stmt;

use atom::{types::*, BinaryOpr};
use word::{CustomIdentifier, Identifier, StmtKeyword};

use crate::{
    error::{ast_err, ast_error, AstError, AstResult},
    stmt::InitKind,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ast {
    TypeDef {
        ident: CustomIdentifier,
        kind: TypeKind,
        args: Vec<GenericPlaceholder>,
    },
    MainDef,
    FuncDef {
        ident: CustomIdentifier,
        kind: FuncKind,
        args: Vec<GenericPlaceholder>,
        inputs: Vec<LiasonedType>,
        output: LiasonedType,
    },
    PatternDef,
    Use {
        ident: CustomIdentifier,
        scope: ScopeId,
    },
    MembDef {
        ident: CustomIdentifier,
        kind: MembKind,
    },
    Stmt(Stmt),
}

impl From<Stmt> for Ast {
    fn from(stmt: Stmt) -> Self {
        Self::Stmt(stmt)
    }
}

impl Ast {
    pub(crate) fn stmt(
        arena: &ExprArena,
        attr: &atom::StmtAttr,
        expr: Option<ExprIdx>,
    ) -> AstResult<Self> {
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
                            let initial_value = opds.end;
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
                StmtKeyword::Return => todo!(),
            }
        } else {
            todo!()
        }
    }
}
