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

    pub(super) fn parse_proc_stmts(
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
                    variant: self.parse_proc_stmt(stmt, item.opt_children, &mut iter)?,
                    instruction_id,
                },
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: ref variant_kind,
                } => todo!(),
                AstKind::FieldDefnHead { .. } => todo!(),
                AstKind::TypeMethodDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { .. } => todo!(),
                AstKind::Submodule { ident, source_file } => todo!(),
            }))
        }
        Ok(Arc::new(stmts))
    }

    fn parse_proc_stmt(
        &mut self,
        stmt: &RawStmt,
        children: Option<IterType>,
        iter: &mut Peekable<IterType>,
    ) -> SemanticResult<ProcStmtVariant> {
        Ok(match stmt.variant {
            RawStmtVariant::Loop(loop_kind) => {
                self.parse_loop_stmt(loop_kind, not_none!(children))?
            }
            RawStmtVariant::Branch(ref branch_kind) => {
                let mut branches = vec![];
                match branch_kind {
                    RawBranchVariant::If { condition } => branches.push(Arc::new(ProcBranch {
                        variant: ProcBranchVariant::If {
                            condition: self.parse_eager_expr(*condition)?,
                        },
                        stmts: self.parse_proc_stmts(not_none!(children))?,
                        range: stmt.range,
                        file: self.file,
                    })),
                    RawBranchVariant::Elif { condition } => todo!(),
                    RawBranchVariant::Else => todo!(),
                    RawBranchVariant::Case { pattern } => todo!(),
                    RawBranchVariant::Default => todo!(),
                }
                while let Some(item) = iter.peek() {
                    let item = match item.value.as_ref()?.kind {
                        AstKind::Stmt(RawStmt {
                            variant: RawStmtVariant::Branch(ref branch_variant),
                            ..
                        }) => match branch_variant {
                            RawBranchVariant::If { .. } => break,
                            RawBranchVariant::Elif { .. } | RawBranchVariant::Else => {
                                iter.next().unwrap()
                            }
                            RawBranchVariant::Case { pattern } => todo!(),
                            RawBranchVariant::Default => todo!(),
                        },
                        _ => break,
                    };
                    match item.value.as_ref()?.kind {
                        AstKind::Stmt(RawStmt {
                            variant: RawStmtVariant::Branch(ref branch_variant),
                            ..
                        }) => match branch_variant {
                            RawBranchVariant::If { .. } => panic!(),
                            RawBranchVariant::Elif { condition } => {
                                if branches.len() == 0 {
                                    todo!()
                                }
                                todo!()
                            }
                            RawBranchVariant::Else => {
                                branches.push(Arc::new(ProcBranch {
                                    variant: ProcBranchVariant::Else,
                                    stmts: self.parse_proc_stmts(not_none!(item.opt_children))?,
                                    range: stmt.range,
                                    file: self.file,
                                }));
                                break;
                            }
                            RawBranchVariant::Case { pattern } => todo!(),
                            RawBranchVariant::Default => todo!(),
                        },
                        _ => break,
                    }
                }
                ProcStmtVariant::BranchGroup {
                    kind: ProcBranchGroupKind::If,
                    branches,
                }
            }
            RawStmtVariant::Exec(expr) => {
                let expr = self.parse_eager_expr(expr)?;
                if expr.ty != EntityRoutePtr::Root(RootIdentifier::Void) {
                    err!(format!(
                        "expect executed expression to be of type void, but got {:?} instead",
                        expr.ty
                    ))
                }
                ProcStmtVariant::Execute { expr }
            }
            RawStmtVariant::Init {
                varname,
                initial_value,
                init_kind,
            } => {
                let initial_value = self.parse_eager_expr(initial_value)?;
                ProcStmtVariant::Init {
                    varname,
                    initial_value,
                    init_kind,
                }
            }
            RawStmtVariant::Return(result) => ProcStmtVariant::Return {
                result: self.parse_eager_expr(result)?,
            },
            RawStmtVariant::Assert(condition) => ProcStmtVariant::Assert {
                condition: self.parse_eager_expr(condition)?,
            },
            RawStmtVariant::Break => ProcStmtVariant::Break,
            RawStmtVariant::Match { .. } => todo!(),
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
            } => ProcStmtVariant::Loop {
                loop_variant: LoopVariant::For {
                    frame_var,
                    initial_boundary: self.parse_boundary(initial_boundary)?,
                    final_boundary: self.parse_boundary(final_boundary)?,
                    step,
                },
                stmts: self.parse_proc_stmts(children)?,
            },
            RawLoopKind::ForExt {
                frame_var,
                final_boundary,
                step,
            } => {
                emsg_once!("todo: change frame var qual in forext");
                ProcStmtVariant::Loop {
                    loop_variant: LoopVariant::ForExt {
                        frame_var,
                        final_boundary: self.parse_boundary(final_boundary)?,
                        step,
                    },
                    stmts: self.parse_proc_stmts(children)?,
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
                    stmts: self.parse_proc_stmts(children)?,
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
                    stmts: self.parse_proc_stmts(children)?,
                }
            }
        })
    }
}
