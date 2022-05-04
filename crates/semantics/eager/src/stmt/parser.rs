use infer_contract::{ContractSheet, InferContract};
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use infer_qualifier::{InferQualifiedType, QualifiedTypeSheet};
use infer_total::InferQueryGroup;
use vm::{StackIdx, VMResult};
use word::RangedCustomIdentifier;

use super::*;

pub(super) struct EagerStmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) variables: Vec<EagerVariable>,
    pub(super) file: FilePtr,
    entity_route_sheet: Arc<EntityRouteSheet>,
    contract_sheet: Arc<ContractSheet>,
    qualified_ty_sheet: Arc<QualifiedTypeSheet>,
}

impl<'a> EagerStmtParser<'a> {
    pub(super) fn new(
        input_placeholders: &[InputPlaceholder],
        db: &'a dyn InferQueryGroup,
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> Self {
        msg_once!("check no errors in entity_route_sheet");
        let qualified_ty_sheet = db.qualified_ty_sheet(file).unwrap();
        Self {
            db,
            arena,
            variables: input_placeholders
                .iter()
                .map(|input_placeholder| {
                    EagerVariable::from_input(&qualified_ty_sheet, input_placeholder).unwrap()
                })
                .collect(),
            file,
            entity_route_sheet: db.entity_route_sheet(file).unwrap(),
            contract_sheet: db.contract_sheet(file).unwrap(),
            qualified_ty_sheet,
        }
    }

    pub(super) fn def_variable(
        &mut self,
        varname: RangedCustomIdentifier,
    ) -> SemanticResult<StackIdx> {
        let varidx = StackIdx::new(self.variables.len())?;
        self.variables.push(EagerVariable {
            ident: varname.ident,
            qualified_ty: self
                .eager_variable_qualified_ty(varname.ident.into(), varname.range.start.row)?,
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
}

impl<'a> InferEntityRoute for EagerStmtParser<'a> {
    fn decl_db(&self) -> &dyn infer_decl::DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &EntityRouteSheet {
        &self.entity_route_sheet
    }
}

impl<'a> InferContract for EagerStmtParser<'a> {
    fn contract_sheet(&self) -> &ContractSheet {
        &self.contract_sheet
    }
}

impl<'a> InferQualifiedType for EagerStmtParser<'a> {
    fn qualified_ty_sheet(&self) -> &infer_qualifier::QualifiedTypeSheet {
        &self.qualified_ty_sheet
    }
}

impl<'a> EagerExprParser<'a> for EagerStmtParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        self.arena
    }

    fn file(&self) -> FilePtr {
        self.file
    }
}
