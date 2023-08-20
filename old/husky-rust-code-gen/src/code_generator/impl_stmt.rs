mod impl_condition_flow;
mod impl_loop;
mod impl_match_pattern;

use fold::Indent;

use husky_eager_semantics::{Boundary, EagerExpr, FuncStmt, LoopVariant, ProcStmt};
use husky_ethereal_term::EtherealTerm;

use husky_coword::RootBuiltinIdent;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_func_stmts(&mut self, stmts: &[HirEagerStmtIdx]) {
        for stmt in stmts {
            self.gen_func_stmt(stmt);
        }
    }

    pub(super) fn gen_proc_stmts(&mut self, stmts: &[HirEagerStmtIdx]) {
        for stmt in stmts {
            self.gen_proc_stmt(stmt);
        }
    }

    fn gen_func_stmt(&mut self, _stmt: &FuncStmt) {
        todo!()
        // self.indent(stmt.indent);
        // match stmt.variant {
        //     FuncStmtVariant::Init {
        //         varname,
        //         ref initial_value,
        //     } => {
        //         self.write("let ");
        //         self.write(&varname.ident);
        //         self.write(" = ");
        //         self.gen_binding(initial_value);
        //         self.gen_expr(stmt.indent, initial_value);
        //         self.write(";");
        //     }
        //     FuncStmtVariant::Assert { ref condition } => {
        //         self.write("assert!(");
        //         self.gen_expr(stmt.indent, condition);
        //         self.write(");");
        //     }
        //     FuncStmtVariant::Require {
        //         ref condition,
        //         return_context,
        //     } => match return_context.kind {
        //         RawReturnContextKind::Normal => {
        //             self.write("normal_require!(");
        //             self.gen_expr(stmt.indent, condition);
        //             self.write(");");
        //         }
        //         RawReturnContextKind::Feature => {
        //             self.write(format!(r#"feature_require!(__ctx, __feature, "#));
        //             self.gen_expr(stmt.indent, condition);
        //             self.write(");");
        //         }
        //         RawReturnContextKind::MemoField => todo!(),
        //     },
        //     FuncStmtVariant::Return {
        //         ref result,
        //         return_context,
        //     } => match result.variant {
        //         EagerExprVariant::Opn {
        //             opn_variant:
        //                 EagerOpnVariant::Suffix {
        //                     opr: EagerSuffixOpr::Unveil,
        //                     ..
        //                 },
        //             ref opds,
        //         } => {
        //             self.write("if let Some(result) = ");
        //             self.gen_expr(stmt.indent, &opds[0]);
        //             self.write(" {");
        //             self.newline_indented(stmt.indent + 4);
        //             match return_context.kind {
        //                 RawReturnContextKind::Normal => {
        //                     if return_context.return_ty().is_option() {
        //                         self.write("return Some(result);");
        //                     } else {
        //                         self.write("return result;");
        //                     }
        //                 }
        //                 RawReturnContextKind::Feature => todo!(),
        //                 RawReturnContextKind::MemoField => todo!(),
        //             }
        //             self.newline_indented(stmt.indent);
        //             self.write("}")
        //         }
        //         _ => {
        //             self.write("return ");
        //             match return_context.kind {
        //                 RawReturnContextKind::Normal => {
        //                     self.gen_binding(result);
        //                     // ad hoc
        //                     if return_context.return_ty().is_option()
        //                         && !result.intrinsic_ty().is_option()
        //                     {
        //                         self.write("Some(");
        //                         self.gen_expr(stmt.indent, result);
        //                         self.write(")")
        //                     } else {
        //                         self.gen_expr(stmt.indent, result)
        //                     }
        //                 }
        //                 RawReturnContextKind::Feature => {
        //                     self.gen_feature_return(stmt.indent, result, return_context.return_ty())
        //                 }
        //                 RawReturnContextKind::MemoField => self.gen_lazy_field_return(
        //                     stmt.indent,
        //                     result,
        //                     return_context.return_ty(),
        //                 ),
        //             }
        //             self.write(";")
        //         }
        //     },
        //     FuncStmtVariant::ConditionFlow { ref branches } => {
        //         self.gen_func_condition_flow(stmt.indent, branches)
        //     }
        //     FuncStmtVariant::Match {
        //         ref match_expr,
        //         ref branches,
        //     } => self.gen_func_match_pattern(match_expr, stmt.indent, branches),
        // }
        // self.newline();
    }

    fn gen_proc_stmt(&mut self, _stmt: &HirEagerStmt) {
        todo!()
        // self.indent(stmt.indent);
        // match stmt.variant {
        //     HirEagerStmt::Init {
        //         varname,
        //         ref initial_value,
        //         init_kind,
        //     } => {
        //         self.write(match init_kind {
        //             InitKind::Let => "let ",
        //             InitKind::Val => "let mut ",
        //             InitKind::Decl => "let ",
        //         });
        //         self.write(&varname.ident);
        //         self.write(" = ");
        //         self.gen_binding(initial_value);
        //         self.gen_expr(stmt.indent, initial_value);
        //         self.write(";");
        //     }
        //     HirEagerStmt::Assert { ref condition } => {
        //         self.write("assert!(");
        //         self.gen_expr(stmt.indent, condition);
        //         self.write(");");
        //     }
        //     HirEagerStmt::Execute { ref expr } => {
        //         self.gen_expr(stmt.indent, expr);
        //         self.write(";");
        //     }
        //     HirEagerStmt::Return {
        //         ref result,
        //         return_context,
        //     } => match result.variant {
        //         EagerExprVariant::Opn {
        //             opn_variant:
        //                 EagerOpnVariant::Suffix {
        //                     opr: EagerSuffixOpr::Unveil,
        //                     ..
        //                 },
        //             ref opds,
        //         } => {
        //             self.write("if let Some(result) = ");
        //             self.gen_expr(stmt.indent, &opds[0]);
        //             self.write(" {");
        //             self.newline_indented(stmt.indent + 4);
        //             self.write("return result;");
        //             self.newline_indented(stmt.indent);
        //             self.write("}")
        //         }
        //         _ => match return_context.kind {
        //             RawReturnContextKind::Normal => {
        //                 self.write("return ");
        //                 self.gen_binding(result);
        //                 self.gen_expr(stmt.indent, result);
        //                 self.write(";")
        //             }
        //             RawReturnContextKind::Feature => {
        //                 self.gen_feature_return(stmt.indent, result, return_context.return_ty())
        //             }
        //             RawReturnContextKind::MemoField => self.gen_lazy_field_return(
        //                 stmt.indent,
        //                 result,
        //                 return_context.opt_return_ty.unwrap().route,
        //             ),
        //         },
        //     },
        //     HirEagerStmt::ConditionFlow { ref branches } => {
        //         self.gen_proc_condition_flow(stmt.indent, branches)
        //     }
        //     HirEagerStmt::Loop {
        //         ref loop_variant,
        //         ref stmts,
        //     } => self.gen_loop_stmt(loop_variant, stmt.indent, stmts),
        //     HirEagerStmt::Break => {
        //         self.write("break;");
        //     }
        //     HirEagerStmt::Match {
        //         ref match_expr,
        //         ref branches,
        //     } => self.gen_proc_match_pattern(match_expr, stmt.indent, branches),
        // }
    }

    fn gen_range_start(&mut self, indent: Indent, boundary: &Boundary) {
        if let Some(bound) = &boundary.opt_bound {
            match boundary.kind {
                BoundaryKind::UpperOpen => todo!(),
                BoundaryKind::UpperClosed => todo!(),
                BoundaryKind::LowerOpen => {
                    self.write("(");
                    self.gen_expr(indent, bound);
                    self.write(" + 1");
                    self.write(")");
                }
                BoundaryKind::LowerClosed => self.gen_expr(indent, bound),
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

    fn gen_range_end(&mut self, indent: Indent, boundary: &Boundary) {
        if let Some(bound) = &boundary.opt_bound {
            match boundary.kind {
                BoundaryKind::UpperOpen => self.gen_expr(indent, bound),
                BoundaryKind::UpperClosed => {
                    self.write("(");
                    self.gen_expr(indent, bound);
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

    fn gen_condition(&mut self, indent: Indent, condition: HirEagerExprIdx) {
        match condition.intrinsic_ty() {
            EtherealTerm::Root(builtin_ident) => match builtin_ident {
                RootBuiltinIdent::Void => todo!(),
                RootBuiltinIdent::I32
                | RootBuiltinIdent::I64
                | RootBuiltinIdent::F32
                | RootBuiltinIdent::F64
                | RootBuiltinIdent::B32
                | RootBuiltinIdent::B64 => {
                    self.gen_expr(indent, condition);
                    self.write(" != 0");
                }
                RootBuiltinIdent::Bool => self.gen_expr(indent, condition),
                RootBuiltinIdent::True
                | RootBuiltinIdent::False
                | RootBuiltinIdent::Vec
                | RootBuiltinIdent::Tuple
                | RootBuiltinIdent::Debug
                | RootBuiltinIdent::Std
                | RootBuiltinIdent::Core
                | RootBuiltinIdent::Mor
                | RootBuiltinIdent::ThickFp
                | RootBuiltinIdent::Fn
                | RootBuiltinIdent::FnMut
                | RootBuiltinIdent::FnOnce
                | RootBuiltinIdent::Array
                | RootBuiltinIdent::DatasetType
                | RootBuiltinIdent::TypeType
                | RootBuiltinIdent::Trait => panic!(),
                RootBuiltinIdent::Domains => todo!(),
                RootBuiltinIdent::CloneTrait => todo!(),
                RootBuiltinIdent::CopyTrait => todo!(),
                RootBuiltinIdent::PartialEqTrait => todo!(),
                RootBuiltinIdent::EqTrait => todo!(),
                RootBuiltinIdent::Module => todo!(),
                RootBuiltinIdent::Ref => todo!(),
                RootBuiltinIdent::RefMut => todo!(),
                RootBuiltinIdent::Option => todo!(),
                RootBuiltinIdent::VisualType => todo!(),
            },
            EtherealTerm::Custom(_) => panic!(),
        }
    }
}
