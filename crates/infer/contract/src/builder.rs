mod eager;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstKind};
use entity_kind::FieldKind;
use entity_route::EntityRouteKind;
use fold::LocalStack;
use infer_decl::DeclQueryGroup;
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use word::RootIdentifier;

use crate::*;

pub struct ContractSheetBuilder<'a> {
    db: &'a dyn InferContractSalsaQueryGroup,
    file: FilePtr,
    main_file: FilePtr,
    contract_sheet: ContractSheet,
}

impl<'a> InferEntityRoute for ContractSheetBuilder<'a> {
    fn decl_db(&self) -> &dyn DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &EntityRouteSheet {
        &self.contract_sheet.entity_route_sheet
    }
}

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn new(db: &'a dyn InferContractSalsaQueryGroup, file: FilePtr) -> Self {
        Self {
            db,
            file,
            main_file: db.main_file(file).unwrap(),
            contract_sheet: ContractSheet::new(db.entity_route_sheet(file).unwrap()),
        }
    }

    pub(crate) fn infer_all(&mut self, ast_iter: AstIter) {
        let arena = self
            .contract_sheet
            .entity_route_sheet
            .ast_text
            .arena
            .clone();
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                        item.children.map(|children| self.infer_all(children));
                    }
                    AstKind::MainDefn => self.infer_morphism(item.children.unwrap(), &arena),
                    AstKind::DatasetConfigDefnHead => self.infer_routine(
                        RootIdentifier::DatasetType.into(),
                        item.children.unwrap(),
                        &arena,
                    ),
                    AstKind::RoutineDefnHead(ref head) => {
                        self.infer_routine(head.output_ty.route, item.children.unwrap(), &arena)
                    }
                    AstKind::PatternDefnHead => todo!(),
                    AstKind::Use { .. } => (),
                    AstKind::FieldDefnHead(ref head) => match head.kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::RecordOriginal => (),
                        FieldKind::StructDerived | FieldKind::RecordDerived => {
                            self.infer_morphism(item.children.unwrap(), &arena)
                        }
                    },
                    AstKind::Stmt(_) => todo!(),
                    AstKind::TypeMethodDefnHead(ref head) => {
                        self.infer_routine(head.output_ty.route, item.children.unwrap(), &arena)
                    }
                    AstKind::FeatureDecl { ty, .. } => {
                        self.infer_morphism(item.children.unwrap(), &arena)
                    }
                },
                _ => (),
            }
        }
    }

    pub(crate) fn finish(self) -> ContractSheet {
        self.contract_sheet
    }
}
