use crate::*;
use ast::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use infer_contract::{ContractSheet, InferContract};
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use semantics_error::*;
use std::{iter::Peekable, sync::Arc};
use vm::{InitKind, StackIdx, VMCompileResult, VMRuntimeResult};
use word::CustomIdentifier;

pub(super) struct LazyStmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) variables: Vec<LazyVariable>,
    pub(super) file: FilePtr,
    entity_route_sheet: Arc<EntityRouteSheet>,
    contract_sheet: Arc<ContractSheet>,
}

impl<'a> LazyStmtParser<'a> {
    pub(super) fn new(
        input_placeholders: &[InputParameter],
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
            entity_route_sheet: db.entity_route_sheet(file).unwrap(),
            contract_sheet: db.contract_sheet(file).unwrap(),
        }
    }

    pub(super) fn def_variable(
        &mut self,
        varname: CustomIdentifier,
        ty: EntityRoutePtr,
    ) -> VMCompileResult<StackIdx> {
        let varidx = StackIdx::new(self.variables.len())?;
        emsg_once!("todo: is reference variable");
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
        iter: AstIter,
    ) -> SemanticResultArc<Vec<Arc<LazyStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            match item.value.as_ref()?.kind {
                AstKind::Use { .. } => todo!(),
                AstKind::Stmt(ref stmt) => {
                    let variant = match stmt.variant {
                        RawStmtVariant::Loop(_) => panic!(),
                        RawStmtVariant::ConditionBranch {
                            condition_branch_kind,
                        } => self.parse_condition_flow(
                            stmt,
                            not_none!(item.opt_children),
                            &mut iter,
                            condition_branch_kind,
                        )?,
                        RawStmtVariant::PatternBranch { .. } => panic!(),
                        RawStmtVariant::Exec { .. } => todo!(),
                        RawStmtVariant::Init {
                            varname,
                            initial_value,
                            init_kind: kind,
                        } => {
                            let initial_value = self.parse_lazy_expr(initial_value)?;
                            if kind != InitKind::Decl {
                                todo!()
                            }
                            self.def_variable(varname.ident, initial_value.ty)?;
                            LazyStmtVariant::Init {
                                varname,
                                value: initial_value,
                            }
                        }
                        RawStmtVariant::Return(result) => LazyStmtVariant::Return {
                            result: self.parse_lazy_expr(result)?,
                        },
                        RawStmtVariant::Assert(condition) => LazyStmtVariant::Assert {
                            condition: self.parse_lazy_expr(condition)?,
                        },
                        RawStmtVariant::Break => todo!(),
                        RawStmtVariant::Match { .. } => panic!(),
                    };
                    stmts.push(Arc::new(LazyStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        variant,
                        instruction_id: Default::default(),
                    }))
                }
                _ => panic!("Unexpected"),
            }
        }
        Ok(Arc::new(stmts))
    }

    fn parse_condition_flow(
        &mut self,
        stmt: &RawStmt,
        children: AstIter,
        iter: &mut Peekable<AstIter>,
        condition_branch_kind: RawConditionBranchKind,
    ) -> SemanticResult<LazyStmtVariant> {
        let mut branches = vec![];
        match condition_branch_kind {
            RawConditionBranchKind::If { condition } => {
                branches.push(Arc::new(LazyConditionBranch {
                    variant: LazyConditionBranchVariant::If {
                        condition: self.parse_lazy_expr(condition)?,
                    },
                    stmts: self.parse_lazy_stmts(children)?,
                }))
            }
            RawConditionBranchKind::Elif { condition } => todo!(),
            RawConditionBranchKind::Else => todo!(),
        }
        while let Some(item) = iter.peek() {
            let item = match item.value.as_ref()?.kind {
                AstKind::Stmt(RawStmt {
                    variant: RawStmtVariant::ConditionBranch { .. },
                    ..
                }) => iter.next().unwrap(),
                _ => break,
            };
            match item.value.as_ref()?.kind {
                AstKind::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::ConditionBranch {
                            ref condition_branch_kind,
                        },
                    ..
                }) => match condition_branch_kind {
                    RawConditionBranchKind::If { .. } => break,
                    RawConditionBranchKind::Elif { condition } => {
                        if branches.len() == 0 {
                            todo!()
                        }
                        todo!()
                    }
                    RawConditionBranchKind::Else => {
                        branches.push(Arc::new(LazyConditionBranch {
                            variant: LazyConditionBranchVariant::Else,
                            stmts: self.parse_lazy_stmts(not_none!(item.opt_children))?,
                        }));
                        break;
                    }
                },
                _ => break,
            }
        }
        Ok(LazyStmtVariant::ConditionFlow { branches })
    }
}

impl<'a> InferEntityRoute for LazyStmtParser<'a> {
    fn decl_db(&self) -> &dyn infer_decl::DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &EntityRouteSheet {
        &self.entity_route_sheet
    }
}

impl<'a> InferContract for LazyStmtParser<'a> {
    fn contract_sheet(&self) -> &ContractSheet {
        &self.contract_sheet
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
