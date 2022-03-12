use std::iter::Peekable;

use vm::Contract;

use super::{parser::StmtParser, *};
use crate::{qual::QualTable, *};

type IterType<'a> = fold::FoldIter<'a, AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>;

impl<'a> StmtParser<'a> {
    fn parse_boundary(&mut self, boundary: RawBoundary) -> SemanticResult<Boundary> {
        let bound = if let Some(bound) = boundary.opt_bound {
            Some(self.parse_expr(&self.arena[bound], Contract::Pure)?)
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
    ) -> SemanticResultArc<Vec<Arc<ImprStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            let instruction_id = InstructionId::default();
            stmts.push(Arc::new(match item.value.as_ref()?.kind {
                AstKind::TypeDef { .. } => todo!(),
                AstKind::MainDef => todo!(),
                AstKind::DatasetConfig => todo!(),
                AstKind::RoutineDef { .. } => todo!(),
                AstKind::PatternDef => todo!(),
                AstKind::Use { .. } => todo!(),
                AstKind::MembDef { .. } => todo!(),
                AstKind::Stmt(ref stmt) => ImprStmt {
                    file: self.file,
                    range: stmt.range,
                    indent: item.indent,
                    kind: self.parse_impr_stmt(stmt, item.children, &mut iter)?,
                    instruction_id,
                },
            }))
        }
        Ok(Arc::new(stmts))
    }

    fn parse_impr_stmt(
        &mut self,
        stmt: &RawStmt,
        children: Option<IterType>,
        iter: &mut Peekable<IterType>,
    ) -> SemanticResult<ImprStmtKind> {
        Ok(match stmt.kind {
            RawStmtKind::Loop(loop_kind) => self.parse_loop_stmt(loop_kind, not_none!(children))?,
            RawStmtKind::Branch(branch_kind) => {
                let mut branches = vec![];
                match branch_kind {
                    RawBranchKind::If { condition } => branches.push(Arc::new(ImprBranch {
                        kind: ImprBranchKind::If {
                            condition: self.parse_expr(&self.arena[condition], Contract::Pure)?,
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
                ImprStmtKind::BranchGroup {
                    kind: ImprBranchGroupKind::If,
                    branches,
                }
            }
            RawStmtKind::Exec(expr) => {
                let expr = self.parse_expr(&self.arena[expr], Contract::Pure)?;
                if expr.ty != ScopePtr::Builtin(BuiltinIdentifier::Void) {
                    err!(format!(
                        "expect executed expression to be of type void, but got {:?} instead",
                        expr.ty
                    ))
                }
                ImprStmtKind::Execute { expr }
            }
            RawStmtKind::Init {
                varname,
                initial_value,
                init_kind,
            } => {
                let initial_value = self.parse_expr(&self.arena[initial_value], Contract::Take)?;
                let qual = Qual::from_init(init_kind, &mut self.qual_table);
                let varidx = self.def_variable(varname, initial_value.ty, qual);
                ImprStmtKind::Init {
                    varname,
                    initial_value,
                    init_kind,
                    varidx,
                }
            }
            RawStmtKind::Return(result) => ImprStmtKind::Return {
                result: self.parse_expr(&self.arena[result], Contract::Take)?,
            },
            RawStmtKind::Assert(condition) => ImprStmtKind::Assert {
                condition: self.parse_expr(&self.arena[condition], Contract::Pure)?,
            },
        })
    }

    fn parse_loop_stmt(
        &mut self,
        loop_kind: RawLoopKind,
        children: IterType,
    ) -> SemanticResult<ImprStmtKind> {
        Ok(match loop_kind {
            RawLoopKind::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => ImprStmtKind::Loop {
                loop_kind: LoopKind::For {
                    frame_var,
                    initial_boundary: self.parse_boundary(initial_boundary)?,
                    final_boundary: self.parse_boundary(final_boundary)?,
                    step,
                },
                stmts: self.parse_impr_stmts(children)?,
            },
            RawLoopKind::ForExt {
                bound,
                is_shifted,
                is_incremental,
                fvar_ident,
            } => todo!(),
            RawLoopKind::While { condition } => todo!(),
            RawLoopKind::DoWhile { condition } => todo!(),
        })
    }
}
