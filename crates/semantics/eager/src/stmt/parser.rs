use infer_contract::{ContractSheet, InferContract};
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use infer_qualifier::{InferQualifiedTy, QualifiedTySheet};
use infer_total::InferQueryGroup;
use text::RangedCustomIdentifier;
use vm::{StackIdx, VMRuntimeResult};

use super::*;

pub(super) struct EagerStmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) file: FilePtr,
    entity_route_sheet: Arc<EntityRouteSheet>,
    contract_sheet: Arc<ContractSheet>,
    qualified_ty_sheet: Arc<QualifiedTySheet>,
}

impl<'a> EagerStmtParser<'a> {
    pub(super) fn new(
        input_placeholders: &[InputParameter],
        db: &'a dyn InferQueryGroup,
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> Self {
        emsg_once!("check no errors in entity_route_sheet");
        let qualified_ty_sheet = db.qualified_ty_sheet(file).unwrap();
        Self {
            db,
            arena,
            file,
            entity_route_sheet: db.entity_route_sheet(file).unwrap(),
            contract_sheet: db.contract_sheet(file).unwrap(),
            qualified_ty_sheet,
        }
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

impl<'a> InferQualifiedTy for EagerStmtParser<'a> {
    fn qualified_ty_sheet(&self) -> &infer_qualifier::QualifiedTySheet {
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
