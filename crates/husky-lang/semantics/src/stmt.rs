use ast::*;
use scope::ScopeId;
use syntax_types::{BinaryOpr, Opr};
use text::TextRange;
use word::{BuiltinIdentifier, CustomIdentifier};

use crate::SemanticResult;

use crate::error::err;
use crate::expr::{BinaryOpnKind, Opn};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyStmt {
    Init {
        varname: CustomIdentifier,
        initial_value: Expr,
    },
    Assert {
        condition: Expr,
    },
    Return {
        result: Expr,
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
        let mut parser = LazyStmtParser::new(self.as_lazy_stmt_query_group(), arena);
        parser.parse(iter)
    }
}

pub struct LazyStmtParser<'a> {
    db: &'a dyn LazyStmtQueryGroup,
    arena: &'a RawExprArena,
    variables: Vec<Variable>,
}

pub struct Variable {
    ident: CustomIdentifier,
    ty: ScopeId,
}

impl<'a> LazyStmtParser<'a> {
    fn new(db: &'a dyn LazyStmtQueryGroup, arena: &'a RawExprArena) -> Self {
        Self {
            db,
            arena,
            variables: Vec::new(),
        }
    }

    fn parse(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResult<Vec<LazyStmt>> {
        iter.map(|item| {
            Ok(match item.value.as_ref()? {
                Ast::TypeDef { .. } => todo!(),
                Ast::MainDef => todo!(),
                Ast::DatasetConfig => todo!(),
                Ast::FuncDef { .. } => todo!(),
                Ast::PatternDef => todo!(),
                Ast::Use { .. } => todo!(),
                Ast::MembDef { .. } => todo!(),
                Ast::Stmt(stmt) => match stmt {
                    RawStmt::Loop(_) => todo!(),
                    RawStmt::Branch(_) => todo!(),
                    RawStmt::Exec(_) => todo!(),
                    RawStmt::Init {
                        varname,
                        initial_value,
                        ..
                    } => {
                        let varname = *varname;
                        let initial_value = self.parse_expr(&self.arena[initial_value])?;
                        self.def_variable(varname, initial_value.ty);
                        LazyStmt::Init {
                            varname,
                            initial_value,
                        }
                    }
                    RawStmt::Return(result) => LazyStmt::Return {
                        result: self.parse_expr(&self.arena[result])?,
                    },
                    RawStmt::Assert(condition) => LazyStmt::Assert {
                        condition: self.parse_expr(&self.arena[condition])?,
                    },
                },
            })
        })
        .collect()
    }

    fn def_variable(&mut self, varname: CustomIdentifier, ty: ScopeId) {
        self.variables.push(Variable { ident: varname, ty });
    }

    fn vartype(&self, varname: CustomIdentifier) -> ScopeId {
        self.variables
            .iter()
            .find_map(|variable| {
                if variable.ident == varname {
                    Some(variable.ty)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn parse_expr(&mut self, raw_expr: &RawExpr) -> SemanticResult<Expr> {
        let (ty, kind): (ScopeId, _) = match &raw_expr.kind {
            RawExprKind::Variable(ident) => (self.vartype(*ident), ExprKind::Variable(*ident)),
            RawExprKind::Scope(id) => (
                todo!(),
                ExprKind::Scope {
                    id: *id,
                    compiled: None,
                },
            ),
            RawExprKind::Literal(value) => (value.ty(), ExprKind::Literal(*value)),
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

    fn parse_opn(&mut self, opr: &Opr, opds: &RawExprRange) -> SemanticResult<(ScopeId, ExprKind)> {
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
        &mut self,
        opr: &BinaryOpr,
        opds: &RawExprRange,
    ) -> SemanticResult<(ScopeId, ExprKind)> {
        let opds = self.arena[opds]
            .iter()
            .map(|raw| self.parse_expr(raw))
            .collect::<SemanticResult<Vec<_>>>()?;
        Ok(match opr {
            BinaryOpr::Less => todo!(),
            BinaryOpr::Leq => todo!(),
            BinaryOpr::Greater => todo!(),
            BinaryOpr::Geq => todo!(),
            BinaryOpr::Neq => todo!(),
            BinaryOpr::Eq => {
                if opds[0].ty != opds[1].ty {
                    err!()
                }
                let opn = match opds[0].ty {
                    ScopeId::Builtin(ident) => {
                        let kind = match ident {
                            BuiltinIdentifier::Void => todo!(),
                            BuiltinIdentifier::I32 => BinaryOpnKind::EqI32,
                            BuiltinIdentifier::F32 => BinaryOpnKind::EqF32,
                            BuiltinIdentifier::Bool => BinaryOpnKind::EqBool,
                            _ => panic!(),
                        };
                        Opn::Binary {
                            opr: BinaryOpr::Eq,
                            this: opds[0].ty,
                            kind,
                        }
                    }
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
            BinaryOpr::LShift => todo!(),
            BinaryOpr::RShift => todo!(),
            BinaryOpr::Add => todo!(),
            BinaryOpr::Sub => todo!(),
            BinaryOpr::Mult => todo!(),
            BinaryOpr::Div => todo!(),
            BinaryOpr::Power => todo!(),
            BinaryOpr::And => todo!(),
            BinaryOpr::BitAnd => todo!(),
            BinaryOpr::Or => todo!(),
            BinaryOpr::BitXor => todo!(),
            BinaryOpr::BitOr => todo!(),
            BinaryOpr::Modulo => todo!(),
        })
    }
}
