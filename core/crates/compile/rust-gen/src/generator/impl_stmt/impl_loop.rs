use super::*;
use semantics_eager::ProcStmt;
use std::sync::Arc;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_loop_stmt(
        &mut self,
        loop_variant: &LoopVariant,
        body_stmts: &[Arc<ProcStmt>],
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
                self.gen_proc_stmts(body_stmts, self.indent + 4);
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
                self.gen_proc_stmts(body_stmts, self.indent + 4);
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
                self.gen_proc_stmts(body_stmts, self.indent + 4);
                self.write_indent();
                self.write("}\n")
            }
            LoopVariant::DoWhile { condition } => {
                self.write("loop {\n");
                self.gen_proc_stmts(body_stmts, self.indent + 4);
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
        }
    }
}
