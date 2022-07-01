mod impl_condition_flow;
mod impl_loop;
mod impl_match_pattern;

use entity_route::EntityRoutePtr;
use semantics_eager::{
    Boundary, EagerExpr, FuncStmt, FuncStmtVariant, LoopVariant, ProcStmt, ProcStmtVariant,
};
use vm::{BoundaryKind, InitKind};
use word::RootIdentifier;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_func_stmts(&mut self, stmts: &[Arc<FuncStmt>]) {
        for stmt in stmts {
            self.gen_func_stmt(stmt);
        }
    }

    pub(super) fn gen_proc_stmts(&mut self, stmts: &[Arc<ProcStmt>]) {
        for stmt in stmts {
            self.gen_proc_stmt(stmt);
        }
    }

    fn gen_func_stmt(&mut self, stmt: &FuncStmt) {
        self.indent(stmt.indent);
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.write("let ");
                self.write(&varname.ident);
                self.write(" = ");
                self.gen_binding(initial_value);
                self.gen_expr(initial_value);
                self.write(";");
            }
            FuncStmtVariant::Assert { ref condition } => {
                self.write("assert!(");
                self.gen_expr(condition);
                self.write(");");
            }
            FuncStmtVariant::Return { ref result } => {
                self.write("return ");
                self.gen_expr(result)
            }
            FuncStmtVariant::ConditionFlow { ref branches } => {
                self.gen_func_condition_flow(stmt.indent, branches)
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => self.gen_func_match_pattern(match_expr, stmt.indent, branches),
        }
        self.newline();
    }

    fn gen_proc_stmt(&mut self, stmt: &ProcStmt) {
        self.indent(stmt.indent);
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => {
                self.write(match init_kind {
                    InitKind::Let => "let ",
                    InitKind::Var => "let mut ",
                    InitKind::Decl => "let ",
                });
                self.write(&varname.ident);
                self.write(" = ");
                self.gen_binding(initial_value);
                self.gen_expr(initial_value);
                self.write(";");
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.write("assert!(");
                self.gen_expr(condition);
                self.write(");");
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.gen_expr(expr);
                self.write(";");
            }
            ProcStmtVariant::Return { ref result } => {
                self.write("return ");
                self.gen_binding(result);
                self.gen_expr(result);
            }
            ProcStmtVariant::ConditionFlow { ref branches } => {
                self.gen_proc_condition_flow(stmt.indent, branches)
            }
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => self.gen_loop_stmt(loop_variant, stmt.indent, stmts),
            ProcStmtVariant::Break => {
                self.write("break;");
            }
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => self.gen_proc_match_pattern(match_expr, stmt.indent, branches),
        }
        self.newline();
    }

    fn gen_range_start(&mut self, boundary: &Boundary) {
        if let Some(bound) = &boundary.opt_bound {
            match boundary.kind {
                BoundaryKind::UpperOpen => todo!(),
                BoundaryKind::UpperClosed => todo!(),
                BoundaryKind::LowerOpen => {
                    self.write("(");
                    self.gen_expr(bound);
                    self.write(" + 1");
                    self.write(")");
                }
                BoundaryKind::LowerClosed => self.gen_expr(bound),
            }
        } else {
            match boundary.kind {
                BoundaryKind::UpperOpen => todo!(),
                BoundaryKind::UpperClosed => todo!(),
                BoundaryKind::LowerOpen => todo!(),
                BoundaryKind::LowerClosed => self.write("0"),
            }
        }
    }

    fn gen_range_end(&mut self, boundary: &Boundary) {
        if let Some(bound) = &boundary.opt_bound {
            match boundary.kind {
                BoundaryKind::UpperOpen => self.gen_expr(bound),
                BoundaryKind::UpperClosed => {
                    self.write("(");
                    self.gen_expr(bound);
                    self.write(" + 1)");
                }
                BoundaryKind::LowerOpen => todo!(),
                BoundaryKind::LowerClosed => todo!(),
            }
        } else {
            match boundary.kind {
                BoundaryKind::UpperOpen => todo!(),
                BoundaryKind::UpperClosed => todo!(),
                BoundaryKind::LowerOpen => todo!(),
                BoundaryKind::LowerClosed => todo!(),
            }
        }
    }

    fn gen_condition(&mut self, condition: &EagerExpr) {
        match condition.ty() {
            EntityRoutePtr::Root(builtin_ident) => match builtin_ident {
                RootIdentifier::Void => todo!(),
                RootIdentifier::I32
                | RootIdentifier::F32
                | RootIdentifier::B32
                | RootIdentifier::B64 => {
                    self.gen_expr(condition);
                    self.write(" != 0");
                }
                RootIdentifier::Bool => self.gen_expr(condition),
                RootIdentifier::True
                | RootIdentifier::False
                | RootIdentifier::Vec
                | RootIdentifier::Tuple
                | RootIdentifier::Debug
                | RootIdentifier::Std
                | RootIdentifier::Core
                | RootIdentifier::Mor
                | RootIdentifier::Fp
                | RootIdentifier::Fn
                | RootIdentifier::FnMut
                | RootIdentifier::FnOnce
                | RootIdentifier::Array
                | RootIdentifier::DatasetType
                | RootIdentifier::TypeType
                | RootIdentifier::TraitType => panic!(),
                RootIdentifier::Domains => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::VisualType => todo!(),
            },
            EntityRoutePtr::Custom(_) => panic!(),
            EntityRoutePtr::ThisType => todo!(),
        }
    }
}
