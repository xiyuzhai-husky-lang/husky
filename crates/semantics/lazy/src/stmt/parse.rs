use crate::*;
use ast::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use infer_contract::{ContractSheet, InferContract};
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use semantics_error::*;
use std::sync::Arc;
use vm::{InitKind, StackIdx, VMResult};
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
            entity_route_sheet: db.entity_route_sheet(file).unwrap(),
            contract_sheet: db.contract_sheet(file).unwrap(),
        }
    }

    pub(super) fn def_variable(
        &mut self,
        varname: CustomIdentifier,
        ty: EntityRoutePtr,
    ) -> VMResult<StackIdx> {
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
                AstKind::Stmt(ref stmt) => match stmt.variant {
                    RawStmtVariant::Loop(_) => todo!(),
                    RawStmtVariant::Branch(ref branch_kind) => {
                        let mut branches = vec![];
                        match branch_kind {
                            RawBranchVariant::If { condition } => {
                                branches.push(Arc::new(LazyBranch {
                                    kind: LazyBranchKind::If {
                                        condition: self.parse_lazy_expr(*condition)?,
                                    },
                                    stmts: self.parse_lazy_stmts(not_none!(item.opt_children))?,
                                }))
                            }
                            RawBranchVariant::Elif { condition } => todo!(),
                            RawBranchVariant::Else => todo!(),
                            RawBranchVariant::Case { pattern } => todo!(),
                            RawBranchVariant::Default => todo!(),
                        }
                        while let Some(item) = iter.peek() {
                            let item = match item.value.as_ref()?.kind {
                                AstKind::Stmt(RawStmt {
                                    variant: RawStmtVariant::Branch(_),
                                    ..
                                }) => iter.next().unwrap(),
                                _ => break,
                            };
                            match item.value.as_ref()?.kind {
                                AstKind::Stmt(RawStmt {
                                    variant: RawStmtVariant::Branch(ref branch_variant),
                                    ..
                                }) => match branch_variant {
                                    RawBranchVariant::If { .. } => break,
                                    RawBranchVariant::Elif { condition } => {
                                        if branches.len() == 0 {
                                            todo!()
                                        }
                                        todo!()
                                    }
                                    RawBranchVariant::Else => {
                                        branches.push(Arc::new(LazyBranch {
                                            kind: LazyBranchKind::Else,
                                            stmts: self
                                                .parse_lazy_stmts(not_none!(item.opt_children))?,
                                        }));
                                        break;
                                    }
                                    RawBranchVariant::Case { pattern } => todo!(),
                                    RawBranchVariant::Default => todo!(),
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
                    RawStmtVariant::Exec(_) => todo!(),
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
                    RawStmtVariant::Return(result) => LazyStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: LazyStmtKind::Return {
                            result: self.parse_lazy_expr(result)?,
                        },
                        instruction_id: Default::default(),
                    },
                    RawStmtVariant::Assert(condition) => LazyStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        kind: LazyStmtKind::Assert {
                            condition: self.parse_lazy_expr(condition)?,
                        },
                        instruction_id: Default::default(),
                    },
                    RawStmtVariant::Break => todo!(),
                    RawStmtVariant::Match { .. } => panic!(),
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
