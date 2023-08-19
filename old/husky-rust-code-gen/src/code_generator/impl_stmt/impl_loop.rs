use super::*;
use husky_eager_semantics::ProcStmt;
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_loop_stmt(
        &mut self,
        loop_variant: &LoopVariant,
        indent: u8,
        body_stmts: &[HirEagerStmtIdx],
    ) {
        match loop_variant {
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
                    self.gen_range_start(indent, initial_boundary);
                    self.write("..");
                    self.gen_range_end(indent, final_boundary);
                } else if step.0 == -1 {
                    self.write("for ");
                    self.write(&frame_var.ident);
                    self.write(" in ");
                    self.gen_range_start(indent, final_boundary);
                    self.write("..");
                    self.gen_range_end(indent, initial_boundary);
                } else {
                    todo!()
                }
                self.write(" {\n");
                self.gen_proc_stmts(body_stmts);
                self.indent(indent);
                self.write("}")
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
                    BoundaryKind::UpperClosed => " <= ",
                    BoundaryKind::LowerOpen => " > ",
                    BoundaryKind::LowerClosed => " >= ",
                });
                if let Some(bound) = &final_boundary.opt_bound {
                    self.gen_expr(indent, bound)
                } else {
                    self.write("0")
                }
                self.write(" {\n");
                self.gen_proc_stmts(body_stmts);
                self.indent(indent + 4);
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
                self.indent(indent);
                self.write("}")
            }
            LoopVariant::While { condition } => {
                self.write("while ");
                self.gen_condition(indent, condition);
                self.write(" {\n");
                self.gen_proc_stmts(body_stmts);
                self.indent(indent);
                self.write("}")
            }
            LoopVariant::DoWhile { condition } => {
                self.write("loop {\n");
                self.gen_proc_stmts(body_stmts);
                self.indent(indent);
                self.write("    if !(");
                self.gen_condition(indent, condition);
                self.write(") {\n");
                self.indent(indent);
                self.write("        break;\n");
                self.indent(indent);
                self.write("    }\n");
                self.indent(indent);
                self.write("}")
            }
        }
    }
}
