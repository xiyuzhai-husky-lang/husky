use entity_route::EntityRoutePtr;
use semantics_eager::{
    Boundary, DeclStmt, DeclStmtKind, EagerExpr, ImprStmt, ImprStmtKind, LoopKind,
};
use vm::{BoundaryKind, InitKind};
use word::BuiltinIdentifier;

use super::*;

impl<'a> RustGenerator<'a> {
    pub(super) fn gen_decl_stmts(&mut self, stmts: &[Arc<DeclStmt>], indent: fold::Indent) {
        let indent_prev = self.indent;
        self.indent = indent;
        for stmt in stmts {
            self.gen_decl_stmt(stmt);
        }
        self.indent = indent_prev;
    }

    pub(super) fn gen_impr_stmts(&mut self, stmts: &[Arc<ImprStmt>], indent: fold::Indent) {
        let indent_prev = self.indent;
        self.indent = indent;
        for stmt in stmts {
            self.gen_impr_stmt(stmt);
        }
        self.indent = indent_prev;
    }

    fn gen_decl_stmt(&mut self, stmt: &DeclStmt) {
        self.write_indent();
        match stmt.kind {
            DeclStmtKind::Init {
                varname,
                ref initial_value,
            } => {
                self.write("let ");
                self.write(&varname);
                self.write(" = ");
                self.gen_expr(initial_value);
                self.write(";\n");
            }
            DeclStmtKind::Assert { ref condition } => todo!(),
            DeclStmtKind::Return { ref result } => self.gen_expr(result),
            DeclStmtKind::Branches { kind, ref branches } => todo!(),
        }
        self.write_newline();
    }

    fn gen_impr_stmt(&mut self, stmt: &ImprStmt) {
        self.write_indent();
        match stmt.kind {
            ImprStmtKind::Init {
                varname,
                ref initial_value,
                init_kind,
                varidx,
            } => {
                self.write(match init_kind {
                    InitKind::Let => "let ",
                    InitKind::Var => "let mut ",
                    InitKind::Decl => todo!(),
                });
                self.write(&varname);
                self.write(" = ");
                self.gen_expr(initial_value);
                self.write(";\n");
            }
            ImprStmtKind::Assert { ref condition } => {
                self.write("assert!(");
                self.gen_expr(condition);
                self.write(");\n");
            }
            ImprStmtKind::Execute { ref expr } => {
                self.gen_expr(expr);
                self.write(";\n");
            }
            ImprStmtKind::Return { ref result } => {
                self.gen_expr(result);
                self.write_newline();
            }
            ImprStmtKind::BranchGroup { kind, ref branches } => todo!(),
            ImprStmtKind::Loop {
                ref loop_kind,
                ref stmts,
            } => match loop_kind {
                LoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    step,
                } => {
                    if step.0 == 1 {
                        self.write("for ");
                        self.write(&frame_var);
                        self.write(" in ");
                        self.gen_range_start(initial_boundary);
                        self.write("..");
                        self.gen_range_end(final_boundary);
                    } else if step.0 == -1 {
                        self.write("for ");
                        self.write(&frame_var);
                        self.write(" in ");
                        self.gen_range_start(final_boundary);
                        self.write("..");
                        self.gen_range_end(initial_boundary);
                    } else {
                        todo!()
                    }
                    self.write(" {\n");
                    self.gen_impr_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("}\n")
                }
                LoopKind::ForExt {
                    frame_var,
                    frame_varidx,
                    final_boundary,
                    step,
                } => {
                    self.write("while ");
                    self.write(&frame_var);
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
                    self.gen_impr_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("    ");
                    self.write(&frame_var);
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
                LoopKind::While { condition } => {
                    self.write("while ");
                    self.gen_condition(condition);
                    self.write(" {\n");
                    self.gen_impr_stmts(stmts, self.indent + 4);
                    self.write_indent();
                    self.write("}\n")
                }
                LoopKind::DoWhile { condition } => {
                    self.write("loop {\n");
                    self.gen_impr_stmts(stmts, self.indent + 4);
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
            EntityRoutePtr::Builtin(builtin_ident) => match builtin_ident {
                BuiltinIdentifier::Void => todo!(),
                BuiltinIdentifier::I32
                | BuiltinIdentifier::F32
                | BuiltinIdentifier::B32
                | BuiltinIdentifier::B64 => {
                    self.gen_expr(condition);
                    self.write(" != 0");
                }
                BuiltinIdentifier::Bool => self.gen_expr(condition),
                BuiltinIdentifier::True
                | BuiltinIdentifier::False
                | BuiltinIdentifier::Vec
                | BuiltinIdentifier::Tuple
                | BuiltinIdentifier::Debug
                | BuiltinIdentifier::Std
                | BuiltinIdentifier::Core
                | BuiltinIdentifier::Fp
                | BuiltinIdentifier::Fn
                | BuiltinIdentifier::FnMut
                | BuiltinIdentifier::FnOnce
                | BuiltinIdentifier::Array
                | BuiltinIdentifier::DatasetType
                | BuiltinIdentifier::Type => panic!(),
                BuiltinIdentifier::Datasets => todo!(),
                BuiltinIdentifier::CloneTrait => todo!(),
                BuiltinIdentifier::CopyTrait => todo!(),
                BuiltinIdentifier::PartialEqTrait => todo!(),
                BuiltinIdentifier::EqTrait => todo!(),
            },
            EntityRoutePtr::Custom(_) => panic!(),
        }
    }
}
