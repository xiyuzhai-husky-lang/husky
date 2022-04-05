use crate::*;
use ast::*;
use entity_route::{EntityRoutePtr, InputPlaceholder};
use file::FilePtr;
use semantics_error::*;
use std::sync::Arc;
use vm::{InitKind, StackIdx, VMResult};
use word::CustomIdentifier;

pub(super) struct LazyStmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) variables: Vec<LazyVariable>,
    pub(super) file: FilePtr,
}

impl<'a> LazyStmtParser<'a> {
    pub(super) fn new(
        input_placeholders: &[InputPlaceholder],
        db: &'a dyn InferQueryGroup,
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> Self {
        Self {
            db,
            arena,
            variables: input_placeholders
                .iter()
                .map(|input_placeholder| LazyVariable::from_input(input_placeholder))
                .collect(),
            file,
        }
    }

    pub(super) fn def_variable(
        &mut self,
        varname: CustomIdentifier,
        ty: EntityRoutePtr,
    ) -> VMResult<StackIdx> {
        let varidx = StackIdx::new(self.variables.len())?;
        msg_once!("todo: is reference variable");
        self.variables.push(LazyVariable {
            ident: varname,
            ty,
            is_reference: false,
        });
        Ok(varidx)
    }

    pub(super) fn varidx(&self, varname: CustomIdentifier) -> StackIdx {
        StackIdx::new(
            self.variables.len()
                - 1
                - self
                    .variables
                    .iter()
                    .rev()
                    .position(|v| v.ident == varname)
                    .unwrap(),
        )
        .unwrap()
    }

    pub(super) fn parse_lazy_stmts(
        &mut self,
        iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    ) -> SemanticResultArc<Vec<Arc<LazyStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            stmts.push(Arc::new(match item.value.as_ref()?.kind {
                AstKind::TypeDefnHead { .. } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::RoutineDefnHead { .. } => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::Use { .. } => todo!(),
                AstKind::Stmt(ref stmt) => match stmt.kind {
                    RawStmtKind::Loop(_) => todo!(),
                    RawStmtKind::Branch(branch_kind) => {
                        let mut branches = vec![];
                        match branch_kind {
                            RawBranchKind::If { condition } => {
                                branches.push(Arc::new(LazyBranch {
                                    kind: LazyBranchKind::If {
                                        condition: self.parse_lazy_expr(condition)?,
                                    },
                                    stmts: self.parse_lazy_stmts(not_none!(item.children))?,
                                }))
                            }
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
                                    RawBranchKind::If { .. } => break,
                                    RawBranchKind::Elif { condition } => {
                                        if branches.len() == 0 {
                                            todo!()
                                        }
                                        todo!()
                                    }
                                    RawBranchKind::Else => {
                                        branches.push(Arc::new(LazyBranch {
                                            kind: LazyBranchKind::Else,
                                            stmts: self
                                                .parse_lazy_stmts(not_none!(item.children))?,
                                        }));
                                        break;
                                    }
                                },
                                _ => break,
                            }
                        }
                        LazyStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: LazyStmtKind::Branches {
                                kind: LazyBranchGroupKind::If,
                                branches,
                            },
                            instruction_id: Default::default(),
                        }
                    }
                    RawStmtKind::Exec(_) => todo!(),
                    RawStmtKind::Init {
                        varname,
                        initial_value,
                        init_kind: kind,
                    } => {
                        let initial_value = self.parse_lazy_expr(initial_value)?;
                        if kind != InitKind::Decl {
                            todo!()
                        }
                        self.def_variable(varname, initial_value.ty)?;
                        LazyStmt {
                            file: self.file,
                            range: stmt.range,
                            indent: item.indent,
                            kind: LazyStmtKind::Init {
                                varname,
                                value: initial_value,
                            },
                            instruction_id: Default::default(),
                        }
                    }
                    RawStmtKind::Return(result) => LazyStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: LazyStmtKind::Return {
                            result: self.parse_lazy_expr(result)?,
                        },
                        instruction_id: Default::default(),
                    },
                    RawStmtKind::Assert(condition) => LazyStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: LazyStmtKind::Assert {
                            condition: self.parse_lazy_expr(condition)?,
                        },
                        instruction_id: Default::default(),
                    },
                },
                AstKind::EnumVariantDefnHead {
                    ident,
                    raw_variant_kind: ref variant_kind,
                } => todo!(),
                AstKind::MembVarDefn { .. } => todo!(),
                AstKind::MembRoutineDefnHead { .. } => todo!(),
                AstKind::FeatureDecl { .. } => todo!(),
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
            }))
        }
        Ok(Arc::new(stmts))
    }
}

impl<'a> LazyExprParser<'a> for LazyStmtParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        self.arena
    }

    fn vartype(&self, varname: CustomIdentifier) -> EntityRoutePtr {
        self.variables
            .iter()
            .find_map(|variable| {
                if variable.ident == varname {
                    Some(variable.ty)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn db(&self) -> &'a dyn InferQueryGroup {
        self.db
    }

    fn file(&self) -> FilePtr {
        self.file
    }
}
