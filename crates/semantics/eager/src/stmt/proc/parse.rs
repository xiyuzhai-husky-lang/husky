use std::iter::Peekable;

use super::{parser::EagerStmtParser, *};
use crate::*;

type IterType<'a> = fold::FoldIter<'a, AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>;

impl<'a> EagerStmtParser<'a> {
    fn parse_boundary(&mut self, boundary: RawBoundary) -> SemanticResult<Boundary> {
        let bound = if let Some(bound) = boundary.opt_bound {
            Some(self.parse_eager_expr(bound)?)
        } else {
            None
        };
        Ok(Boundary {
            opt_bound: bound,
            kind: boundary.kind,
        })
    }

    pub(super) fn parse_impr_stmts(
        &mut self,
        iter: IterType,
    ) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            let instruction_id = InstructionId::default();
            stmts.push(Arc::new(match item.value.as_ref()?.kind {
                AstKind::TypeDefnHead { .. } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::RoutineDefnHead { .. } => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::Use { .. } => todo!(),
                AstKind::Stmt(ref stmt) => ProcStmt {
                    file: self.file,
                    range: stmt.range,
                    indent: item.indent,
                    variant: self.parse_impr_stmt(stmt, item.children, &mut iter)?,
                    instruction_id,
                },
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: ref variant_kind,
                } => todo!(),
                AstKind::FieldDefnHead { .. } => todo!(),
                AstKind::TypeMethodDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { .. } => todo!(),
            }))
        }
        Ok(Arc::new(stmts))
    }

    fn parse_impr_stmt(
        &mut self,
        stmt: &RawStmt,
        children: Option<IterType>,
        iter: &mut Peekable<IterType>,
    ) -> SemanticResult<ProcStmtVariant> {
        Ok(match stmt.kind {
            RawStmtKind::Loop(loop_kind) => self.parse_loop_stmt(loop_kind, not_none!(children))?,
            RawStmtKind::Branch(branch_kind) => {
                let mut branches = vec![];
                match branch_kind {
                    RawBranchKind::If { condition } => branches.push(Arc::new(ImprBranch {
                        kind: ImprBranchKind::If {
                            condition: self.parse_eager_expr(condition)?,
                        },
                        stmts: self.parse_impr_stmts(not_none!(children))?,
                    })),
                    RawBranchKind::Elif { condition } => todo!(),
                    RawBranchKind::Else => todo!(),
                }
                while let Some(item) = iter.peek() {
                    let item = match item.value.as_ref()?.kind {
                        AstKind::Stmt(RawStmt {
                            kind: RawStmtKind::Branch(_),
                            ..
                        }) => iter.next().unwrap(),
                        _ => break,
                    };
                    match item.value.as_ref()?.kind {
                        AstKind::Stmt(RawStmt {
                            kind: RawStmtKind::Branch(branch_stmt),
                            ..
                        }) => match branch_stmt {
                            RawBranchKind::If { condition } => break,
                            RawBranchKind::Elif { condition } => {
                                if branches.len() == 0 {
                                    todo!()
                                }
                                todo!()
                            }
                            RawBranchKind::Else => {
                                branches.push(Arc::new(ImprBranch {
                                    kind: ImprBranchKind::Else,
                                    stmts: self.parse_impr_stmts(not_none!(item.children))?,
                                }));
                                break;
                            }
                        },
                        _ => break,
                    }
                }
                ProcStmtVariant::BranchGroup {
                    kind: ImprBranchGroupKind::If,
                    branches,
                }
            }
            RawStmtKind::Exec(expr) => {
                let expr = self.parse_eager_expr(expr)?;
                if expr.ty != EntityRoutePtr::Root(RootIdentifier::Void) {
                    err!(format!(
                        "expect executed expression to be of type void, but got {:?} instead",
                        expr.ty
                    ))
                }
                ProcStmtVariant::Execute { expr }
            }
            RawStmtKind::Init {
                varname,
                initial_value,
                init_kind,
            } => {
                let initial_value = self.parse_eager_expr(initial_value)?;
                let qual = Qual::from_init(init_kind);
                let varidx = self.def_variable(varname, initial_value.ty, qual)?;
                ProcStmtVariant::Init {
                    varname,
                    initial_value,
                    init_kind,
                    varidx,
                }
            }
            RawStmtKind::Return(result) => ProcStmtVariant::Return {
                result: self.parse_eager_expr(result)?,
            },
            RawStmtKind::Assert(condition) => ProcStmtVariant::Assert {
                condition: self.parse_eager_expr(condition)?,
            },
        })
    }

    fn parse_loop_stmt(
        &mut self,
        loop_kind: RawLoopKind,
        children: IterType,
    ) -> SemanticResult<ProcStmtVariant> {
        Ok(match loop_kind {
            RawLoopKind::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => {
                self.def_variable(
                    frame_var,
                    EntityRoutePtr::Root(RootIdentifier::I32),
                    Qual::frame_var(),
                )?;
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::For {
                        frame_var,
                        initial_boundary: self.parse_boundary(initial_boundary)?,
                        final_boundary: self.parse_boundary(final_boundary)?,
                        step,
                    },
                    stmts: self.parse_impr_stmts(children)?,
                }
            }
            RawLoopKind::ForExt {
                frame_var,
                final_boundary,
                step,
            } => {
                msg_once!("todo: change frame var qual in forext");
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::ForExt {
                        frame_var,
                        frame_varidx: self.varidx(frame_var.ident),
                        final_boundary: self.parse_boundary(final_boundary)?,
                        step,
                    },
                    stmts: self.parse_impr_stmts(children)?,
                }
            }
            RawLoopKind::While { condition } => {
                let condition = self.parse_eager_expr(condition)?;
                match condition.ty {
                    EntityRoutePtr::Root(RootIdentifier::Bool)
                    | EntityRoutePtr::Root(RootIdentifier::I32)
                    | EntityRoutePtr::Root(RootIdentifier::F32)
                    | EntityRoutePtr::Root(RootIdentifier::B32)
                    | EntityRoutePtr::Root(RootIdentifier::B64) => (),
                    _ => todo!(),
                }
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::While { condition },
                    stmts: self.parse_impr_stmts(children)?,
                }
            }
            RawLoopKind::DoWhile { condition } => {
                let condition = self.parse_eager_expr(condition)?;
                match condition.ty {
                    EntityRoutePtr::Root(RootIdentifier::Bool)
                    | EntityRoutePtr::Root(RootIdentifier::I32)
                    | EntityRoutePtr::Root(RootIdentifier::F32)
                    | EntityRoutePtr::Root(RootIdentifier::B32)
                    | EntityRoutePtr::Root(RootIdentifier::B64) => (),
                    _ => todo!(),
                }
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::DoWhile { condition },
                    stmts: self.parse_impr_stmts(children)?,
                }
            }
        })
    }
}
