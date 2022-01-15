use ast::*;
use scope::ScopeId;
use syntax_types::{BinaryOpr, Opr};
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use crate::SemanticResult;

use crate::error::err;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyStmt {
    Init {
        varname: CustomIdentifier,
        initial_value: Expr,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrictStmt {}

pub trait LazyStmtQueryGroup {
    fn as_lazy_stmt_query_group(&self) -> &dyn LazyStmtQueryGroup;

    fn parse_lazy_stmts(
        &self,
        arena: &RawExprArena,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResult<Vec<LazyStmt>> {
        let parser = LazyStmtParser::new(self.as_lazy_stmt_query_group(), arena);
        parser.parse(iter)
    }
}

pub struct LazyStmtParser<'a> {
    db: &'a dyn LazyStmtQueryGroup,
    arena: &'a RawExprArena,
}

impl<'a> LazyStmtParser<'a> {
    fn new(db: &'a dyn LazyStmtQueryGroup, arena: &'a RawExprArena) -> Self {
        Self { db, arena }
    }
    fn parse(
        &self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResult<Vec<LazyStmt>> {
        iter.map(|item| {
            Ok(match item.value.as_ref()? {
                Ast::TypeDef {
                    ident,
                    kind,
                    generics,
                } => todo!(),
                Ast::MainDef => todo!(),
                Ast::DatasetConfig => todo!(),
                Ast::FuncDef { kind, decl } => todo!(),
                Ast::PatternDef => todo!(),
                Ast::Use { ident, scope } => todo!(),
                Ast::MembDef { ident, kind } => todo!(),
                Ast::Stmt(stmt) => match stmt {
                    RawStmt::Loop(_) => todo!(),
                    RawStmt::Branch(_) => todo!(),
                    RawStmt::Exec(_) => todo!(),
                    RawStmt::Init {
                        varname,
                        initial_value,
                        ..
                    } => LazyStmt::Init {
                        varname: *varname,
                        initial_value: self.parse_expr(&self.arena[initial_value])?,
                    },
                    RawStmt::Return(_) => todo!(),
                    RawStmt::Assert(_) => todo!(),
                },
            })
        })
        .collect()
    }

    fn parse_expr(&self, raw_expr: &RawExpr) -> SemanticResult<Expr> {
        let (ty, kind): (ScopeId, _) = match &raw_expr.kind {
            RawExprKind::Variable(ident) => (todo!(), ExprKind::Variable(*ident)),
            RawExprKind::Scope(id) => (
                todo!(),
                ExprKind::Scope {
                    id: *id,
                    compiled: None,
                },
            ),
            RawExprKind::Literal(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, opds } => self.parse_opn(opr, opds)?,
            RawExprKind::Lambda(_, _) => todo!(),
        };
        Ok(Expr {
            range: raw_expr.range.clone(),
            ty,
            kind,
        })
    }

    fn parse_opn(&self, opr: &Opr, opds: &RawExprRange) -> SemanticResult<(ScopeId, ExprKind)> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(_) => todo!(),
            Opr::List(opr) => match opr {
                syntax_types::ListOpr::TupleInit => todo!(),
                syntax_types::ListOpr::NewVec => todo!(),
                syntax_types::ListOpr::NewDict => todo!(),
                syntax_types::ListOpr::Call => todo!(),
                syntax_types::ListOpr::Index => todo!(),
                syntax_types::ListOpr::ModuloIndex => todo!(),
                syntax_types::ListOpr::StructInit => todo!(),
            },
        }
    }

    fn parse_binary_opr(
        &self,
        opr: &BinaryOpr,
        opds: &RawExprRange,
    ) -> SemanticResult<(ScopeId, ExprKind)> {
        let opds = self.arena[opds]
            .iter()
            .map(|raw| self.parse_expr(raw))
            .collect::<SemanticResult<Vec<_>>>()?;
        Ok(match opr {
            syntax_types::BinaryOpr::Less => todo!(),
            syntax_types::BinaryOpr::Leq => todo!(),
            syntax_types::BinaryOpr::Greater => todo!(),
            syntax_types::BinaryOpr::Geq => todo!(),
            syntax_types::BinaryOpr::Neq => todo!(),
            syntax_types::BinaryOpr::Eq => {
                if opds[0].ty != opds[1].ty {
                    err!()
                }
                let opn = match opds[0].ty {
                    ScopeId::Builtin(ident) => match ident {
                        BuiltinIdentifier::Void => todo!(),
                        BuiltinIdentifier::I32 => todo!(),
                        BuiltinIdentifier::F32 => todo!(),
                        BuiltinIdentifier::Bool => todo!(),
                        _ => panic!(),
                    },
                    ScopeId::Custom(_) => todo!(),
                };
                (
                    BuiltinIdentifier::Bool.into(),
                    ExprKind::Opn {
                        opds,
                        compiled: None,
                        opn,
                    },
                )
            }
            syntax_types::BinaryOpr::LShift => todo!(),
            syntax_types::BinaryOpr::RShift => todo!(),
            syntax_types::BinaryOpr::Add => todo!(),
            syntax_types::BinaryOpr::Sub => todo!(),
            syntax_types::BinaryOpr::Mult => todo!(),
            syntax_types::BinaryOpr::Div => todo!(),
            syntax_types::BinaryOpr::Power => todo!(),
            syntax_types::BinaryOpr::And => todo!(),
            syntax_types::BinaryOpr::BitAnd => todo!(),
            syntax_types::BinaryOpr::Or => todo!(),
            syntax_types::BinaryOpr::BitXor => todo!(),
            syntax_types::BinaryOpr::BitOr => todo!(),
            syntax_types::BinaryOpr::Modulo => todo!(),
        })
    }
}
