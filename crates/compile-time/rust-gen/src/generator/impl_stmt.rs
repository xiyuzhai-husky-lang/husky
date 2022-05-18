use entity_route::EntityRoutePtr;
use semantics_eager::{
    Boundary, EagerExpr, FuncStmt, FuncStmtVariant, LoopVariant, ProcStmt, ProcStmtVariant,
};
use vm::{BoundaryKind, InitKind};
use word::RootIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_func_stmts(&mut self, stmts: &[Arc<FuncStmt>], indent: fold::Indent) {
        let indent_prev = self.indent;
        self.indent = indent;
        for stmt in stmts {
            self.gen_func_stmt(stmt);
        }
        self.indent = indent_prev;
    }

    pub(super) fn gen_proc_stmts(&mut self, stmts: &[Arc<ProcStmt>], indent: fold::Indent) {
        let indent_prev = self.indent;
        self.indent = indent;
        for stmt in stmts {
            self.gen_proc_stmt(stmt);
        }
        self.indent = indent_prev;
    }

    fn gen_func_stmt(&mut self, stmt: &FuncStmt) {
        self.write_indent();
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.write("let ");
                self.write(&varname.ident);
                self.write(" = ");
                self.gen_expr(initial_value);
                self.write(";\n");
            }
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => self.gen_expr(result),
            FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
            FuncStmtVariant::Match { ref branches } => todo!(),
        }
        self.write_newline();
    }

    fn gen_proc_stmt(&mut self, stmt: &ProcStmt) {
        self.write_indent();
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => {
                self.write(match init_kind {
                    InitKind::Let => "let ",
                    InitKind::Var => "let mut ",
                    InitKind::Decl => todo!(),
                });
                self.write(&varname.ident);
                self.write(" = ");
                self.gen_expr(initial_value);
                self.write(";\n");
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.write("assert!(");
                self.gen_expr(condition);
                self.write(");\n");
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.gen_expr(expr);
                self.write(";\n");
            }
            ProcStmtVariant::Return { ref result } => {
                self.gen_expr(result);
                self.write_newline();
            }
            ProcStmtVariant::ConditionFlow { ref branches } => todo!(),
            ProcStmtVariant::Loop {
                loop_variant: ref loop_kind,
                ref stmts,
            } => match loop_kind {
                LoopVariant::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    step,
                } => {
                    if step.0 == 1 {
                        self.write("for ");
                        self.write(&frame_var.ident);
                        self.write(" in ");
                        self.gen_range_start(initial_boundary);
                        self.write("..");
                        self.gen_range_end(final_boundary);
                    } else if step.0 == -1 {
                        self.write("for ");
                        self.write(&frame_var.ident);
                        self.write(" in ");
                        self.gen_range_start(final_boundary);
                        self.write("..");
                        self.gen_range_end(initial_boundary);
                    } else {
                        todo!()
                    }
                    self.write(" {\n");
                    self.gen_proc_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("}\n")
                }
                LoopVariant::ForExt {
                    frame_var,
                    final_boundary,
                    step,
                } => {
                    self.write("while ");
                    self.write(&frame_var.ident);
                    self.write(match final_boundary.kind {
                        BoundaryKind::UpperOpen => " < ",
                        BoundaryKind::UpperClosed => todo!(),
                        BoundaryKind::LowerOpen => " > ",
                        BoundaryKind::LowerClosed => todo!(),
                    });
                    if let Some(bound) = &final_boundary.opt_bound {
                        self.gen_expr(bound)
                    } else {
                        self.write("0")
                    }
                    self.write(" {\n");
                    self.gen_proc_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("    ");
                    self.write(&frame_var.ident);
                    if step.0 > 0 {
                        self.write(" += ");
                        self.write(&step.0.to_string());
                    } else if step.0 < 0 {
                        self.write(" -= ");
                        self.write(&(-step.0).to_string());
                    } else {
                        panic!()
                    }
                    self.write(";\n");
                    self.write_indent();
                    self.write("}\n")
                }
                LoopVariant::While { condition } => {
                    self.write("while ");
                    self.gen_condition(condition);
                    self.write(" {\n");
                    self.gen_proc_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("}\n")
                }
                LoopVariant::DoWhile { condition } => {
                    self.write("loop {\n");
                    self.gen_proc_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("    if !(");
                    self.gen_condition(condition);
                    self.write(") {\n");
                    self.write_indent();
                    self.write("        break;\n");
                    self.write_indent();
                    self.write("    }\n");
                    self.write_indent();
                    self.write("}\n")
                }
            },
            ProcStmtVariant::Break => todo!(),
            ProcStmtVariant::Match { ref branches } => todo!(),
        }
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
                BoundaryKind::UpperClosed => todo!(),
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
        match condition.ty {
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
                | RootIdentifier::Fp
                | RootIdentifier::Fn
                | RootIdentifier::FnMut
                | RootIdentifier::FnOnce
                | RootIdentifier::Array
                | RootIdentifier::DatasetType
                | RootIdentifier::TypeType => panic!(),
                RootIdentifier::Datasets => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::ModuleType => todo!(),
            },
            EntityRoutePtr::Custom(_) => panic!(),
            EntityRoutePtr::ThisType => todo!(),
        }
    }
}
