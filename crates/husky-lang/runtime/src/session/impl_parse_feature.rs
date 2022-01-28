use common::*;
use semantics::{BinaryOpnKind, DeclStmt, Expr, ExprKind, Opn};
use word::{CustomIdentifier, ImplicitIdentifier};

use crate::*;
use vm::StackValue;

use super::*;

impl<'sess> Session<'sess> {
    pub(super) fn parse_body(&mut self, stmts: &[DeclStmt]) -> FeatureId {
        let mut stack = FeatureStack::new(self);
        stack.parse_stmts(None, stmts)
    }
}

struct FeatureStack<'a, 'sess: 'a> {
    vars: Vec<FeatureVariable>,
    sess: &'a mut Session<'sess>,
}

struct FeatureVariable {
    varname: CustomIdentifier,
    value: FeatureId,
}

impl<'a, 'sess: 'a> FeatureStack<'a, 'sess> {
    fn new(sess: &'a mut Session<'sess>) -> Self {
        Self { sess, vars: vec![] }
    }

    fn parse_stmts(&mut self, prev: Option<FeatureId>, stmts: &[DeclStmt]) -> FeatureId {
        if stmts.len() == 0 {
            todo!()
        }
        let stmt = &stmts[0];
        let next = &stmts[1..];
        match stmt.kind {
            semantics::DeclStmtKind::Init {
                varname,
                ref initial_value,
            } => {
                self.def_var(varname, initial_value);
                self.parse_stmts(prev, next)
            }
            semantics::DeclStmtKind::Assert { ref condition } => {
                let condition = self.parse_expr(condition);
                let condition = self.cache(condition);
                let assert = self.sess.intern_feature(Feature::Assert { condition });
                let new_prev = self.merge(prev, assert);
                self.parse_stmts(Some(new_prev), next)
            }
            semantics::DeclStmtKind::Return { ref result } => {
                let result = self.parse_expr(result);
                let result = self.cache(result);
                self.merge(prev, result)
            }
        }
    }

    fn cache(&mut self, feature: FeatureId) -> FeatureId {
        self.sess.intern_feature(Feature::Cached(feature))
    }

    fn merge(&mut self, first: Option<FeatureId>, second: FeatureId) -> FeatureId {
        if let Some(first) = first {
            self.sess.intern_feature(Feature::Do {
                first,
                then: second,
            })
        } else {
            second
        }
    }

    fn parse_expr(&mut self, initial_value: &Expr) -> FeatureId {
        match initial_value.kind {
            ExprKind::Variable(varname) => self.resolve_var(varname),
            ExprKind::Scope { scope, compiled } => match scope.route {
                scope::ScopeRoute::Implicit {
                    main,
                    ident: ImplicitIdentifier::Input,
                } => self.sess.intern_feature(Feature::Input),
                _ => todo!(),
            },
            ExprKind::Literal(value) => self.sess.intern_feature(Feature::Literal(value)),
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn,
                compiled,
                ref opds,
            } => match opn {
                Opn::Binary { opr, this, kind } => {
                    if kind == BinaryOpnKind::Custom {
                        todo!()
                    } else {
                        let lopd = self.parse_expr(&opds[0]);
                        let ropd = self.parse_expr(&opds[1]);
                        self.sess.intern_feature(Feature::PrimitiveBinaryFunc {
                            func: opr.into(),
                            lopd,
                            ropd,
                        })
                    }
                }
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(_) => todo!(),
                Opn::FuncCall { func } => todo!(),
                Opn::PattCall => todo!(),
                Opn::MembVarAccess => todo!(),
                Opn::MembFuncCall(_) => todo!(),
                Opn::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        }
    }

    fn def_var(&mut self, varname: CustomIdentifier, value: &Expr) {
        let value = self.parse_expr(value);
        let value = self.cache(value);
        self.vars.push(FeatureVariable { varname, value })
    }

    fn resolve_var(&self, varname: CustomIdentifier) -> FeatureId {
        self.vars
            .iter()
            .rev()
            .find_map(|var| {
                if var.varname == varname {
                    Some(var.value)
                } else {
                    None
                }
            })
            .unwrap()
    }
}
