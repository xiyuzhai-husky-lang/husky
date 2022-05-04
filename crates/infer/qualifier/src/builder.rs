mod eager;
mod inputs;
mod lazy;

use std::sync::Arc;

use ast::{AstIter, AstKind, RawExprArena};
use entity_kind::FieldKind;
use entity_route_query::EntityRouteResult;
use file::FilePtr;
use infer_contract::{ContractSheet, InferContract};
use infer_decl::DeclQueryGroup;
use infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use word::RootIdentifier;

use crate::*;

pub(crate) struct QualifiedTySheetBuilder<'a> {
    db: &'a dyn InferQualifiedTyQueryGroup,
    contract_sheet: Arc<ContractSheet>,
    entity_route_sheet: Arc<EntityRouteSheet>,
    qualified_ty_sheet: QualifiedTySheet,
    main_file: FilePtr,
}

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn new(
        db: &'a dyn InferQualifiedTyQueryGroup,
        file: FilePtr,
    ) -> EntityRouteResult<Self> {
        let contract_sheet = db.contract_sheet(file)?;
        Ok(Self {
            db,
            entity_route_sheet: contract_sheet.entity_route_sheet.clone(),
            contract_sheet,
            qualified_ty_sheet: Default::default(),
            main_file: db.main_file(file).unwrap(),
        })
    }

    pub(super) fn infer_all(&mut self) {
        use fold::FoldStorage;
        let arena = self.entity_route_sheet.ast_text.arena.clone();
        let ast_text = self.entity_route_sheet.ast_text.clone();
        self.infer_ast(ast_text.folded_results.iter(), &arena)
    }

    pub(super) fn infer_ast(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        for item in ast_iter {
            match item.value {
                Ok(value) => match value.kind {
                    AstKind::TypeDefnHead { .. } | AstKind::EnumVariantDefnHead { .. } => {
                        item.children
                            .map(|children| self.infer_ast(children, arena));
                    }
                    AstKind::MainDefn => self.infer_morphism(
                        &arena,
                        item.children.unwrap(),
                        self.db.global_output_ty(self.main_file).ok(),
                    ),
                    AstKind::DatasetConfigDefnHead => self.build_routine(
                        &[],
                        item.children.unwrap(),
                        &arena,
                        Some(EntityRoutePtr::Root(RootIdentifier::DatasetType)),
                        OutputContract::Transitive,
                    ),
                    AstKind::RoutineDefnHead(ref head) => self.build_routine(
                        &head.input_placeholders,
                        item.children.unwrap(),
                        &arena,
                        Some(head.output_ty.route),
                        head.output_contract,
                    ),
                    AstKind::PatternDefnHead => todo!(),
                    AstKind::Use { .. } => (),
                    AstKind::FieldDefnHead(ref head) => match head.kind {
                        FieldKind::StructOriginal => (),
                        FieldKind::RecordOriginal => (),
                        FieldKind::StructDerived | FieldKind::RecordDerived => {
                            self.infer_morphism(&arena, item.children.unwrap(), Some(head.ty))
                        }
                    },
                    AstKind::Stmt(_) => todo!(),
                    AstKind::TypeMethodDefnHead(ref head) => self.build_routine(
                        &head.input_placeholders,
                        item.children.unwrap(),
                        &arena,
                        Some(head.output_ty.route),
                        head.output_contract,
                    ),
                    AstKind::FeatureDecl { ty, .. } => {
                        self.infer_morphism(&arena, item.children.unwrap(), Some(ty.route))
                    }
                },
                _ => (),
            }
        }
    }

    pub(super) fn finish(self) -> Arc<QualifiedTySheet> {
        Arc::new(self.qualified_ty_sheet)
    }
}

impl<'a> InferContract for QualifiedTySheetBuilder<'a> {
    fn contract_sheet(&self) -> &ContractSheet {
        &self.contract_sheet
    }
}

impl<'a> InferEntityRoute for QualifiedTySheetBuilder<'a> {
    fn decl_db(&self) -> &dyn DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &infer_entity_route::EntityRouteSheet {
        &self.entity_route_sheet
    }
}
